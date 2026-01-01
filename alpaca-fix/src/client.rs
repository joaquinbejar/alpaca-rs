//! FIX protocol client implementation.

use crate::codec::{FixDecoder, FixMessage, tags};
use crate::config::FixConfig;
use crate::error::{FixError, Result};
use crate::messages::{
    ExecType, ExecutionReport, MarketDataRequest, MsgType, NewOrderSingle, OrdStatus,
    OrderCancelReplaceRequest, OrderCancelRequest, Side,
};
use crate::session::{FixSession, SessionState};
use alpaca_base::Credentials;
use std::sync::Arc;
use tokio::sync::Mutex;

/// FIX protocol client for Alpaca.
#[derive(Debug)]
pub struct FixClient {
    /// Alpaca credentials.
    #[allow(dead_code)]
    credentials: Credentials,
    /// FIX session.
    session: Arc<Mutex<FixSession>>,
    /// Message decoder.
    decoder: FixDecoder,
    /// Configuration.
    config: FixConfig,
}

impl FixClient {
    /// Create a new FIX client.
    #[must_use]
    pub fn new(credentials: Credentials, config: FixConfig) -> Self {
        let session = FixSession::new(config.clone());
        Self {
            credentials,
            session: Arc::new(Mutex::new(session)),
            decoder: FixDecoder::new(),
            config,
        }
    }

    /// Get the current session state.
    pub async fn state(&self) -> SessionState {
        self.session.lock().await.state()
    }

    /// Connect to the FIX server.
    ///
    /// # Errors
    /// Returns error if connection fails.
    pub async fn connect(&self) -> Result<()> {
        let mut session = self.session.lock().await;
        session.set_state(SessionState::Connecting);

        // TODO: Implement actual TCP connection
        // For now, this is a placeholder for the connection logic
        tracing::info!("Connecting to {}:{}", self.config.host, self.config.port);

        session.set_state(SessionState::LoggingOn);

        // Create and send logon message
        let _logon = session.create_logon();
        // TODO: Send logon message over TCP

        session.set_state(SessionState::Active);
        tracing::info!("FIX session established");

        Ok(())
    }

    /// Disconnect from the FIX server.
    ///
    /// # Errors
    /// Returns error if disconnect fails.
    pub async fn disconnect(&self) -> Result<()> {
        let mut session = self.session.lock().await;
        session.set_state(SessionState::LoggingOut);

        let _logout = session.create_logout(None);
        // TODO: Send logout message over TCP

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
        let _msg = session.encode_message(MsgType::NewOrderSingle.as_str(), &fields);

        // TODO: Send message over TCP and wait for acknowledgment

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

        let _msg = session.encode_message(MsgType::OrderCancelRequest.as_str(), &fields);

        // TODO: Send message over TCP

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

        let _msg = session.encode_message(MsgType::OrderCancelReplaceRequest.as_str(), &fields);

        // TODO: Send message over TCP

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

        let _msg = session.encode_message(MsgType::MarketDataRequest.as_str(), &fields);

        // TODO: Send message over TCP

        Ok(request.md_req_id.clone())
    }

    /// Process an incoming message.
    ///
    /// # Arguments
    /// * `data` - Raw FIX message data
    ///
    /// # Errors
    /// Returns error if message processing fails.
    pub async fn process_message(&self, data: &str) -> Result<FixMessage> {
        let msg = self.decoder.decode(data)?;

        let mut session = self.session.lock().await;
        session.validate_sequence(&msg)?;

        // Handle session-level messages
        if let Some(msg_type) = msg.msg_type() {
            match MsgType::from_fix_str(msg_type) {
                Some(MsgType::Heartbeat) => {
                    tracing::debug!("Received heartbeat");
                }
                Some(MsgType::TestRequest) => {
                    if let Some(test_req_id) = msg.get(tags::TEST_REQ_ID) {
                        let _heartbeat = session.create_heartbeat(Some(test_req_id));
                        // TODO: Send heartbeat response
                    }
                }
                Some(MsgType::Logout) => {
                    session.set_state(SessionState::Disconnected);
                    tracing::info!("Received logout");
                }
                Some(MsgType::ResendRequest) => {
                    // TODO: Handle resend request
                    tracing::warn!("Resend request not implemented");
                }
                _ => {}
            }
        }

        Ok(msg)
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
