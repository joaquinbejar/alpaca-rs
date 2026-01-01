//! FIX protocol client implementation.

use crate::codec::{FixDecoder, FixMessage, tags};
use crate::config::FixConfig;
use crate::error::{FixError, Result};
use crate::messages::{
    ExecType, ExecutionReport, MarketDataRequest, MsgType, NewOrderSingle, OrdStatus,
    OrderCancelReplaceRequest, OrderCancelRequest, Side,
};
use crate::session::{FixSession, SessionState};
use crate::transport::{self, FixTransport};
use alpaca_base::Credentials;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{interval, timeout};

/// Channel buffer size for incoming messages.
const MESSAGE_CHANNEL_SIZE: usize = 1000;

/// Default timeout for operations in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// FIX protocol client for Alpaca.
pub struct FixClient {
    /// Alpaca credentials.
    #[allow(dead_code)]
    credentials: Credentials,
    /// FIX session.
    session: Arc<Mutex<FixSession>>,
    /// TCP transport.
    transport: Arc<Mutex<Option<FixTransport>>>,
    /// Message decoder.
    #[allow(dead_code)]
    decoder: FixDecoder,
    /// Configuration.
    config: FixConfig,
    /// Incoming message receiver.
    message_rx: Arc<Mutex<Option<mpsc::Receiver<FixMessage>>>>,
    /// Shutdown signal sender.
    shutdown_tx: Arc<Mutex<Option<mpsc::Sender<()>>>>,
}

impl std::fmt::Debug for FixClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixClient")
            .field("config", &self.config)
            .finish()
    }
}

impl FixClient {
    /// Create a new FIX client.
    #[must_use]
    pub fn new(credentials: Credentials, config: FixConfig) -> Self {
        let session = FixSession::new(config.clone());
        Self {
            credentials,
            session: Arc::new(Mutex::new(session)),
            transport: Arc::new(Mutex::new(None)),
            decoder: FixDecoder::new(),
            config,
            message_rx: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the current session state.
    pub async fn state(&self) -> SessionState {
        self.session.lock().await.state()
    }

    /// Connect to the FIX server and establish a session.
    ///
    /// # Errors
    /// Returns error if connection or logon fails.
    pub async fn connect(&self) -> Result<()> {
        let mut session = self.session.lock().await;
        session.set_state(SessionState::Connecting);

        // Establish TCP connection
        tracing::info!(
            "Connecting to FIX server at {}:{}",
            self.config.host,
            self.config.port
        );

        let tcp_transport = transport::connect(&self.config.host, self.config.port).await?;

        // Store transport
        {
            let mut transport_guard = self.transport.lock().await;
            *transport_guard = Some(tcp_transport);
        }

        session.set_state(SessionState::LoggingOn);

        // Send logon message
        let logon = session.create_logon();
        self.send_raw(&logon).await?;

        // Wait for logon response
        let logon_response = self
            .receive_with_timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .await?;

        // Validate logon response
        if let Some(msg_type) = logon_response.msg_type() {
            match MsgType::from_fix_str(msg_type) {
                Some(MsgType::Logon) => {
                    tracing::info!("Logon successful");
                    session.set_state(SessionState::Active);
                }
                Some(MsgType::Logout) => {
                    let text = logon_response.get(tags::TEXT).unwrap_or("unknown reason");
                    session.set_state(SessionState::Disconnected);
                    return Err(FixError::Authentication(format!(
                        "logon rejected: {}",
                        text
                    )));
                }
                _ => {
                    return Err(FixError::Session(format!(
                        "unexpected response to logon: {:?}",
                        msg_type
                    )));
                }
            }
        } else {
            return Err(FixError::InvalidMessage(
                "missing MsgType in response".to_string(),
            ));
        }

        // Start background tasks
        self.start_background_tasks().await;

        tracing::info!("FIX session established");
        Ok(())
    }

    /// Disconnect from the FIX server.
    ///
    /// # Errors
    /// Returns error if disconnect fails.
    pub async fn disconnect(&self) -> Result<()> {
        // Send shutdown signal to background tasks
        if let Some(tx) = self.shutdown_tx.lock().await.take() {
            let _ = tx.send(()).await;
        }

        let mut session = self.session.lock().await;

        if session.state() == SessionState::Active {
            session.set_state(SessionState::LoggingOut);

            // Send logout message
            let logout = session.create_logout(None);
            if let Err(e) = self.send_raw(&logout).await {
                tracing::warn!("Failed to send logout: {}", e);
            }

            // Wait briefly for logout response
            if let Ok(response) = self.receive_with_timeout(Duration::from_secs(5)).await
                && let Some(msg_type) = response.msg_type()
                && MsgType::from_fix_str(msg_type) == Some(MsgType::Logout)
            {
                tracing::info!("Logout confirmed by server");
            }
        }

        // Close transport
        if let Some(transport) = self.transport.lock().await.take() {
            let _ = transport.close().await;
        }

        session.set_state(SessionState::Disconnected);
        tracing::info!("FIX session terminated");

        Ok(())
    }

    /// Send a new order.
    ///
    /// # Arguments
    /// * `order` - New order single message
    ///
    /// # Errors
    /// Returns error if order submission fails.
    pub async fn send_order(&self, order: &NewOrderSingle) -> Result<String> {
        let session = self.session.lock().await;

        if session.state() != SessionState::Active {
            return Err(FixError::Session("session not active".to_string()));
        }

        let fields = self.build_new_order_fields(order);
        let msg = session.encode_message(MsgType::NewOrderSingle.as_str(), &fields);
        drop(session);

        self.send_raw(&msg).await?;

        tracing::debug!("Sent new order: cl_ord_id={}", order.cl_ord_id);
        Ok(order.cl_ord_id.clone())
    }

    /// Cancel an order.
    ///
    /// # Arguments
    /// * `cancel` - Order cancel request
    ///
    /// # Errors
    /// Returns error if cancel request fails.
    pub async fn cancel_order(&self, cancel: &OrderCancelRequest) -> Result<String> {
        let session = self.session.lock().await;

        if session.state() != SessionState::Active {
            return Err(FixError::Session("session not active".to_string()));
        }

        let fields = vec![
            (tags::ORIG_CL_ORD_ID, cancel.orig_cl_ord_id.clone()),
            (tags::CL_ORD_ID, cancel.cl_ord_id.clone()),
            (tags::SYMBOL, cancel.symbol.clone()),
            (tags::SIDE, cancel.side.as_char().to_string()),
        ];

        let msg = session.encode_message(MsgType::OrderCancelRequest.as_str(), &fields);
        drop(session);

        self.send_raw(&msg).await?;

        tracing::debug!("Sent cancel request: cl_ord_id={}", cancel.cl_ord_id);
        Ok(cancel.cl_ord_id.clone())
    }

    /// Replace an order.
    ///
    /// # Arguments
    /// * `replace` - Order cancel/replace request
    ///
    /// # Errors
    /// Returns error if replace request fails.
    pub async fn replace_order(&self, replace: &OrderCancelReplaceRequest) -> Result<String> {
        let session = self.session.lock().await;

        if session.state() != SessionState::Active {
            return Err(FixError::Session("session not active".to_string()));
        }

        let mut fields = vec![
            (tags::ORIG_CL_ORD_ID, replace.orig_cl_ord_id.clone()),
            (tags::CL_ORD_ID, replace.cl_ord_id.clone()),
            (tags::SYMBOL, replace.symbol.clone()),
            (tags::SIDE, replace.side.as_char().to_string()),
            (tags::ORD_TYPE, replace.ord_type.as_char().to_string()),
            (tags::ORDER_QTY, replace.order_qty.to_string()),
        ];

        if let Some(price) = replace.price {
            fields.push((tags::PRICE, price.to_string()));
        }

        let msg = session.encode_message(MsgType::OrderCancelReplaceRequest.as_str(), &fields);
        drop(session);

        self.send_raw(&msg).await?;

        tracing::debug!("Sent replace request: cl_ord_id={}", replace.cl_ord_id);
        Ok(replace.cl_ord_id.clone())
    }

    /// Request market data.
    ///
    /// # Arguments
    /// * `request` - Market data request
    ///
    /// # Errors
    /// Returns error if request fails.
    pub async fn request_market_data(&self, request: &MarketDataRequest) -> Result<String> {
        let session = self.session.lock().await;

        if session.state() != SessionState::Active {
            return Err(FixError::Session("session not active".to_string()));
        }

        let fields = vec![
            (tags::MD_REQ_ID, request.md_req_id.clone()),
            (
                tags::SUBSCRIPTION_REQUEST_TYPE,
                request.subscription_request_type.to_string(),
            ),
            (tags::MARKET_DEPTH, request.market_depth.to_string()),
        ];

        let msg = session.encode_message(MsgType::MarketDataRequest.as_str(), &fields);
        drop(session);

        self.send_raw(&msg).await?;

        tracing::debug!("Sent market data request: md_req_id={}", request.md_req_id);
        Ok(request.md_req_id.clone())
    }

    /// Receive the next message from the server.
    ///
    /// # Errors
    /// Returns error if no message is available or channel is closed.
    pub async fn next_message(&self) -> Result<FixMessage> {
        let mut rx_guard = self.message_rx.lock().await;
        if let Some(ref mut rx) = *rx_guard {
            rx.recv()
                .await
                .ok_or_else(|| FixError::Connection("message channel closed".to_string()))
        } else {
            Err(FixError::Session("not connected".to_string()))
        }
    }

    /// Process an incoming message.
    ///
    /// # Arguments
    /// * `msg` - FIX message
    ///
    /// # Errors
    /// Returns error if message processing fails.
    pub async fn process_message(&self, msg: &FixMessage) -> Result<()> {
        let mut session = self.session.lock().await;
        session.validate_sequence(msg)?;

        // Handle session-level messages
        if let Some(msg_type) = msg.msg_type() {
            match MsgType::from_fix_str(msg_type) {
                Some(MsgType::Heartbeat) => {
                    tracing::debug!("Received heartbeat");
                }
                Some(MsgType::TestRequest) => {
                    if let Some(test_req_id) = msg.get(tags::TEST_REQ_ID) {
                        let heartbeat = session.create_heartbeat(Some(test_req_id));
                        drop(session);
                        self.send_raw(&heartbeat).await?;
                        tracing::debug!("Sent heartbeat response");
                    }
                }
                Some(MsgType::Logout) => {
                    session.set_state(SessionState::Disconnected);
                    tracing::info!("Received logout from server");
                }
                Some(MsgType::ResendRequest) => {
                    tracing::warn!("Resend request received - not fully implemented");
                    // TODO: Implement message resend
                }
                Some(MsgType::SequenceReset) => {
                    if let Some(new_seq) = msg.get(tags::MSG_SEQ_NUM)
                        && let Ok(seq) = new_seq.parse::<u64>()
                    {
                        session.seq_nums().set_incoming(seq);
                        tracing::info!("Sequence reset to {}", seq);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse an execution report from a FIX message.
    ///
    /// # Arguments
    /// * `msg` - FIX message
    ///
    /// # Errors
    /// Returns error if parsing fails.
    pub fn parse_execution_report(&self, msg: &FixMessage) -> Result<ExecutionReport> {
        let order_id = msg
            .get(tags::ORDER_ID)
            .ok_or_else(|| FixError::InvalidMessage("missing OrderID".to_string()))?
            .to_string();

        let cl_ord_id = msg
            .get(tags::CL_ORD_ID)
            .ok_or_else(|| FixError::InvalidMessage("missing ClOrdID".to_string()))?
            .to_string();

        let exec_id = msg
            .get(tags::EXEC_ID)
            .ok_or_else(|| FixError::InvalidMessage("missing ExecID".to_string()))?
            .to_string();

        let exec_type_char = msg
            .get(tags::EXEC_TYPE)
            .and_then(|s| s.chars().next())
            .ok_or_else(|| FixError::InvalidMessage("missing ExecType".to_string()))?;

        let exec_type = ExecType::from_char(exec_type_char)
            .ok_or_else(|| FixError::InvalidMessage("invalid ExecType".to_string()))?;

        let ord_status_char = msg
            .get(tags::ORD_STATUS)
            .and_then(|s| s.chars().next())
            .ok_or_else(|| FixError::InvalidMessage("missing OrdStatus".to_string()))?;

        let ord_status = OrdStatus::from_char(ord_status_char)
            .ok_or_else(|| FixError::InvalidMessage("invalid OrdStatus".to_string()))?;

        let symbol = msg
            .get(tags::SYMBOL)
            .ok_or_else(|| FixError::InvalidMessage("missing Symbol".to_string()))?
            .to_string();

        let side_char = msg
            .get(tags::SIDE)
            .and_then(|s| s.chars().next())
            .ok_or_else(|| FixError::InvalidMessage("missing Side".to_string()))?;

        let side = Side::from_char(side_char)
            .ok_or_else(|| FixError::InvalidMessage("invalid Side".to_string()))?;

        let order_qty: f64 = msg
            .get(tags::ORDER_QTY)
            .ok_or_else(|| FixError::InvalidMessage("missing OrderQty".to_string()))?
            .parse()
            .map_err(|_| FixError::Decoding("invalid OrderQty".to_string()))?;

        let cum_qty: f64 = msg.get(tags::CUM_QTY).unwrap_or("0").parse().unwrap_or(0.0);

        let avg_px: f64 = msg.get(tags::AVG_PX).unwrap_or("0").parse().unwrap_or(0.0);

        let leaves_qty: f64 = msg
            .get(tags::LEAVES_QTY)
            .unwrap_or("0")
            .parse()
            .unwrap_or(0.0);

        let last_qty = msg.get(tags::LAST_QTY).and_then(|s| s.parse().ok());
        let last_px = msg.get(tags::LAST_PX).and_then(|s| s.parse().ok());
        let text = msg.get(tags::TEXT).map(String::from);

        Ok(ExecutionReport {
            order_id,
            cl_ord_id,
            exec_id,
            exec_type,
            ord_status,
            symbol,
            side,
            order_qty,
            last_qty,
            last_px,
            cum_qty,
            avg_px,
            leaves_qty,
            text,
        })
    }

    /// Build FIX fields for a new order.
    fn build_new_order_fields(&self, order: &NewOrderSingle) -> Vec<(u32, String)> {
        let mut fields = vec![
            (tags::CL_ORD_ID, order.cl_ord_id.clone()),
            (tags::SYMBOL, order.symbol.clone()),
            (tags::SIDE, order.side.as_char().to_string()),
            (tags::ORD_TYPE, order.ord_type.as_char().to_string()),
            (tags::ORDER_QTY, order.order_qty.to_string()),
            (
                tags::TIME_IN_FORCE,
                order.time_in_force.as_char().to_string(),
            ),
        ];

        if let Some(price) = order.price {
            fields.push((tags::PRICE, price.to_string()));
        }

        if let Some(stop_px) = order.stop_px {
            fields.push((tags::STOP_PX, stop_px.to_string()));
        }

        if let Some(ref account) = order.account {
            fields.push((tags::ACCOUNT, account.clone()));
        }

        fields
    }

    /// Send a raw FIX message over the transport.
    async fn send_raw(&self, message: &str) -> Result<()> {
        let transport_guard = self.transport.lock().await;
        if let Some(ref transport) = *transport_guard {
            transport.send(message).await
        } else {
            Err(FixError::Connection("not connected".to_string()))
        }
    }

    /// Receive a message with timeout.
    async fn receive_with_timeout(&self, duration: Duration) -> Result<FixMessage> {
        let transport_guard = self.transport.lock().await;
        if transport_guard.is_some() {
            drop(transport_guard);

            let transport_clone = self.transport.clone();
            timeout(duration, async move {
                let guard = transport_clone.lock().await;
                if let Some(ref t) = *guard {
                    t.receive().await
                } else {
                    Err(FixError::Connection("not connected".to_string()))
                }
            })
            .await
            .map_err(|_| FixError::Timeout("receive timeout".to_string()))?
        } else {
            Err(FixError::Connection("not connected".to_string()))
        }
    }

    /// Start background tasks for heartbeat and message receiving.
    async fn start_background_tasks(&self) {
        let (msg_tx, msg_rx) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

        // Store receivers
        *self.message_rx.lock().await = Some(msg_rx);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Clone references for background tasks
        let transport = Arc::clone(&self.transport);
        let session = Arc::clone(&self.session);
        let heartbeat_interval = self.config.heartbeat_interval_secs;

        // Spawn message receiver task
        let transport_recv = Arc::clone(&transport);
        let session_recv = Arc::clone(&session);
        let msg_tx_clone = msg_tx.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        tracing::debug!("Message receiver shutting down");
                        break;
                    }
                    result = async {
                        let guard = transport_recv.lock().await;
                        if guard.is_some() {
                            drop(guard);
                            let guard2 = transport_recv.lock().await;
                            if let Some(ref t) = *guard2 {
                                t.receive().await
                            } else {
                                Err(FixError::Connection("disconnected".to_string()))
                            }
                        } else {
                            // Not connected, wait a bit
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            Err(FixError::Connection("not connected".to_string()))
                        }
                    } => {
                        match result {
                            Ok(msg) => {
                                // Process session-level messages
                                if let Some(msg_type) = msg.msg_type() {
                                    match MsgType::from_fix_str(msg_type) {
                                        Some(MsgType::TestRequest) => {
                                            // Respond to test request with heartbeat
                                            if let Some(test_req_id) = msg.get(tags::TEST_REQ_ID) {
                                                let session_guard = session_recv.lock().await;
                                                let heartbeat = session_guard.create_heartbeat(Some(test_req_id));
                                                drop(session_guard);

                                                let transport_guard = transport_recv.lock().await;
                                                if let Some(ref t) = *transport_guard {
                                                    let _ = t.send(&heartbeat).await;
                                                }
                                            }
                                        }
                                        Some(MsgType::Logout) => {
                                            let mut session_guard = session_recv.lock().await;
                                            session_guard.set_state(SessionState::Disconnected);
                                            tracing::info!("Server initiated logout");
                                        }
                                        _ => {}
                                    }
                                }

                                // Forward message to channel
                                if msg_tx_clone.send(msg).await.is_err() {
                                    tracing::debug!("Message channel closed");
                                    break;
                                }
                            }
                            Err(FixError::Connection(_)) => {
                                // Connection lost
                                let mut session_guard = session_recv.lock().await;
                                if session_guard.state() == SessionState::Active {
                                    session_guard.set_state(SessionState::Disconnected);
                                    tracing::warn!("Connection lost");
                                }
                                break;
                            }
                            Err(e) => {
                                tracing::error!("Error receiving message: {}", e);
                            }
                        }
                    }
                }
            }
        });

        // Spawn heartbeat task
        let transport_hb = Arc::clone(&transport);
        let session_hb = Arc::clone(&session);

        tokio::spawn(async move {
            let mut heartbeat_timer = interval(Duration::from_secs(heartbeat_interval.into()));

            loop {
                heartbeat_timer.tick().await;

                let session_guard = session_hb.lock().await;
                if session_guard.state() != SessionState::Active {
                    break;
                }

                let heartbeat = session_guard.create_heartbeat(None);
                drop(session_guard);

                let transport_guard = transport_hb.lock().await;
                if let Some(ref t) = *transport_guard {
                    if let Err(e) = t.send(&heartbeat).await {
                        tracing::warn!("Failed to send heartbeat: {}", e);
                        break;
                    }
                    tracing::debug!("Sent heartbeat");
                } else {
                    break;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FixVersion;

    fn test_credentials() -> Credentials {
        Credentials::new("test_key".to_string(), "test_secret".to_string())
    }

    #[tokio::test]
    async fn test_client_creation() {
        let config = FixConfig::builder()
            .version(FixVersion::Fix44)
            .sender_comp_id("SENDER")
            .target_comp_id("TARGET")
            .build();

        let client = FixClient::new(test_credentials(), config);
        assert_eq!(client.state().await, SessionState::Disconnected);
    }

    #[tokio::test]
    async fn test_send_order_requires_active_session() {
        let config = FixConfig::builder()
            .sender_comp_id("SENDER")
            .target_comp_id("TARGET")
            .build();

        let client = FixClient::new(test_credentials(), config);
        let order = NewOrderSingle::market("AAPL", Side::Buy, 100.0);

        let result = client.send_order(&order).await;
        assert!(result.is_err());
    }
}
