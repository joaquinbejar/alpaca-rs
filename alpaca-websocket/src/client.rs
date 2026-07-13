//! WebSocket client for Alpaca streaming data.

#![allow(missing_docs)]

use crate::{config::WebSocketConfig, messages::*, streams::*};
use alpaca_base::types::Quote;
use alpaca_base::{AlpacaError, Result, auth::Credentials, types::Environment};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde_json;
use std::sync::Once;
use std::time::Duration;
use tokio::{
    net::TcpStream,
    sync::mpsc,
    sync::mpsc::error::TrySendError,
    time::{interval, sleep, timeout},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

static CRYPTO_PROVIDER_INIT: Once = Once::new();

/// Initialize the rustls crypto provider (ring).
/// This must be called before any TLS connections are made.
fn init_crypto_provider() {
    CRYPTO_PROVIDER_INIT.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsReceiver = SplitStream<WsStream>;

/// WebSocket client for Alpaca API
#[derive(Debug)]
pub struct AlpacaWebSocketClient {
    credentials: Credentials,
    environment: Environment,
    url: String,
}

/// Data feed type for market data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFeed {
    /// IEX exchange data (free, delayed)
    Iex,
    /// SIP data (paid, real-time)
    Sip,
    /// 15-minute delayed SIP data
    DelayedSip,
    /// BOATS (Blue Ocean ATS) overnight trading data
    Boats,
    /// Derived Alpaca overnight feed
    Overnight,
    /// Crypto data
    Crypto,
}

impl AlpacaWebSocketClient {
    /// Create a new WebSocket client for stocks
    pub fn new(credentials: Credentials, environment: Environment) -> Self {
        let url = match environment {
            Environment::Paper => "wss://stream.data.alpaca.markets/v2/iex",
            Environment::Live => "wss://stream.data.alpaca.markets/v2/sip",
        };

        Self {
            credentials,
            environment,
            url: url.to_string(),
        }
    }

    /// Create a new client from environment variables
    pub fn from_env(environment: Environment) -> Result<Self> {
        let credentials = Credentials::from_env()?;
        Ok(Self::new(credentials, environment))
    }

    /// Create a WebSocket client for a specific data feed
    pub fn with_feed(credentials: Credentials, environment: Environment, feed: DataFeed) -> Self {
        let url = match feed {
            DataFeed::Iex => "wss://stream.data.alpaca.markets/v2/iex",
            DataFeed::Sip => "wss://stream.data.alpaca.markets/v2/sip",
            DataFeed::DelayedSip => "wss://stream.data.alpaca.markets/v2/delayed_sip",
            DataFeed::Boats => "wss://stream.data.alpaca.markets/v1beta1/boats",
            DataFeed::Overnight => "wss://stream.data.alpaca.markets/v1beta1/overnight",
            DataFeed::Crypto => "wss://stream.data.alpaca.markets/v1beta3/crypto/us",
        };

        Self {
            credentials,
            environment,
            url: url.to_string(),
        }
    }

    /// Create a crypto WebSocket client
    pub fn crypto(credentials: Credentials, environment: Environment) -> Self {
        Self::with_feed(credentials, environment, DataFeed::Crypto)
    }

    /// Create a crypto client from environment variables
    pub fn crypto_from_env(environment: Environment) -> Result<Self> {
        let credentials = Credentials::from_env()?;
        Ok(Self::crypto(credentials, environment))
    }

    /// Create a trading WebSocket client
    pub fn trading(credentials: Credentials, environment: Environment) -> Self {
        let url = environment.websocket_url();
        Self {
            credentials,
            environment,
            url: url.to_string(),
        }
    }

    /// Create a WebSocket client for an arbitrary stream URL.
    ///
    /// Useful for proxies or test servers; prefer [`Self::new`],
    /// [`Self::with_feed`], or [`Self::trading`] for the standard endpoints.
    pub fn with_url(
        credentials: Credentials,
        environment: Environment,
        url: impl Into<String>,
    ) -> Self {
        Self {
            credentials,
            environment,
            url: url.into(),
        }
    }

    /// Connect to the WebSocket and return a stream of messages
    pub async fn connect(&self) -> Result<AlpacaStream> {
        // Initialize crypto provider for TLS
        init_crypto_provider();

        let (sender, receiver) = mpsc::unbounded_channel();
        info!("Connecting to WebSocket: {}", self.url);
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut sink, mut stream) = ws_stream.split();

        // Authenticate
        self.authenticate(&mut sink).await?;

        // Spawn message handler
        let credentials = self.credentials.clone();
        tokio::spawn(async move {
            Self::handle_messages(&mut stream, sender, credentials).await;
        });

        Ok(AlpacaStream::new(receiver))
    }

    /// Connect with automatic reconnection
    pub async fn connect_with_reconnect(&self, max_retries: u32) -> Result<AlpacaStream> {
        let mut attempts = 0;
        let mut delay = Duration::from_secs(1);

        loop {
            match self.connect().await {
                Ok(stream) => {
                    info!("Successfully connected to WebSocket");
                    return Ok(stream);
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_retries {
                        error!("Failed to connect after {} attempts", attempts);
                        return Err(AlpacaError::WebSocket(format!(
                            "Connection failed after {} attempts: {}",
                            attempts, e
                        )));
                    }

                    warn!(
                        "Connection attempt {} failed: {}. Retrying in {:?}",
                        attempts, e, delay
                    );
                    sleep(delay).await;
                    delay = std::cmp::min(delay * 2, Duration::from_secs(60));
                }
            }
        }
    }

    /// Subscribe to market data with the default [`WebSocketConfig`].
    ///
    /// See [`Self::subscribe_market_data_with_config`] for connection
    /// ownership and lifecycle semantics.
    pub async fn subscribe_market_data(
        &self,
        subscription: SubscribeMessage,
    ) -> Result<MarketDataStream> {
        self.subscribe_market_data_with_config(subscription, WebSocketConfig::default())
            .await
    }

    /// Subscribe to market data with an explicit [`WebSocketConfig`].
    ///
    /// # Connection ownership and lifecycle
    ///
    /// The returned [`MarketDataStream`] is backed by a background task that
    /// owns the WebSocket connection:
    ///
    /// - The initial connection, authentication, and subscription happen
    ///   before this method returns; failures are returned as `Err`.
    /// - After a successful start, if the connection closes or errors the
    ///   task reconnects with capped exponential backoff
    ///   (`reconnect_base_delay_ms * 2^(attempt - 1)`, capped at
    ///   `reconnect_max_delay_ms`) and re-issues the active subscription
    ///   set. Progress is reported via [`MarketDataEvent::Reconnecting`]
    ///   and [`MarketDataEvent::Reconnected`].
    /// - When reconnection is disabled or `reconnect_max_attempts`
    ///   consecutive attempts fail, a final
    ///   [`MarketDataEvent::Disconnected`] is emitted and the stream ends.
    /// - Events are delivered over a bounded channel of
    ///   `message_buffer_size` entries. If the consumer falls behind, data
    ///   updates are dropped and reported via [`MarketDataEvent::Lagged`];
    ///   lifecycle events are never dropped.
    /// - Dropping the stream stops the background task and closes the
    ///   connection.
    pub async fn subscribe_market_data_with_config(
        &self,
        subscription: SubscribeMessage,
        config: WebSocketConfig,
    ) -> Result<MarketDataStream> {
        // Initialize crypto provider for TLS
        init_crypto_provider();

        let url = self.url.clone();
        let credentials = self.credentials.clone();
        let stream = open_market_data_stream(&url, &credentials, &subscription, &config).await?;

        let (sender, receiver) = mpsc::channel(config.message_buffer_size.max(1));
        tokio::spawn(run_market_data_task(
            stream,
            url,
            credentials,
            subscription,
            config,
            sender,
        ));

        Ok(MarketDataStream::new(receiver))
    }

    /// Subscribe to trading updates
    pub async fn subscribe_trading_updates(&self) -> Result<TradingStream> {
        let stream = self.connect().await?;
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut trading_stream = stream.trading_updates();
            while let Some(update) = trading_stream.next().await {
                if sender.send(update).is_err() {
                    break;
                }
            }
        });

        Ok(TradingStream::new(receiver))
    }

    /// Authenticate with the WebSocket
    async fn authenticate(&self, sink: &mut WsSink) -> Result<()> {
        send_auth(&self.credentials, sink).await
    }

    /// Handle incoming WebSocket messages
    async fn handle_messages(
        stream: &mut WsReceiver,
        sender: mpsc::UnboundedSender<WebSocketMessage>,
        _credentials: Credentials,
    ) {
        while let Some(message) = stream.next().await {
            match message {
                Ok(Message::Text(text)) => match Self::parse_message(&text) {
                    Ok(msg) => {
                        debug!("Received message: {:?}", msg);
                        if sender.send(msg).is_err() {
                            warn!("Failed to send message to channel");
                            break;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse message: {} - Raw: {}", e, text);
                    }
                },
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed");
                    break;
                }
                Ok(Message::Ping(_data)) => {
                    debug!("Received ping, sending pong");
                    // Note: tokio-tungstenite handles pong automatically
                }
                Ok(Message::Pong(_)) => {
                    debug!("Received pong");
                }
                Ok(Message::Binary(_)) => {
                    warn!("Received unexpected binary message");
                }
                Ok(Message::Frame(_)) => {
                    debug!("Received frame message");
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }

        info!("Message handler exiting");
    }

    /// Parse incoming WebSocket message
    fn parse_message(text: &str) -> Result<WebSocketMessage> {
        // Handle array of messages
        if text.starts_with('[') {
            let messages: Vec<serde_json::Value> = serde_json::from_str(text)?;
            if let Some(first_msg) = messages.first() {
                return serde_json::from_value(first_msg.clone())
                    .map_err(|e| AlpacaError::Json(e.to_string()));
            }
        }

        // Handle single message
        serde_json::from_str(text).map_err(|e| AlpacaError::Json(e.to_string()))
    }

    /// Send subscription message
    pub async fn send_subscription(&self, subscription: SubscribeMessage) -> Result<()> {
        // This would need to be implemented with a persistent connection
        // For now, this is a placeholder
        debug!("Would send subscription: {:?}", subscription);
        Ok(())
    }

    /// Send unsubscription message
    pub async fn send_unsubscription(&self, unsubscription: UnsubscribeMessage) -> Result<()> {
        // This would need to be implemented with a persistent connection
        // For now, this is a placeholder
        debug!("Would send unsubscription: {:?}", unsubscription);
        Ok(())
    }

    /// Get the WebSocket URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the environment
    pub fn environment(&self) -> &Environment {
        &self.environment
    }
}

/// Redact an API key for logging: show only its last four characters, and
/// nothing at all for short keys.
fn redact_key(key: &str) -> String {
    const VISIBLE: usize = 4;
    let len = key.chars().count();
    if len <= VISIBLE * 2 {
        return "****".to_string();
    }
    let suffix: String = key.chars().skip(len - VISIBLE).collect();
    format!("****{suffix}")
}

/// Send the authentication frame. The frame itself is never logged because
/// it contains the API key and secret.
async fn send_auth(credentials: &Credentials, sink: &mut WsSink) -> Result<()> {
    // Alpaca uses {"action": "auth", "key": "...", "secret": "..."}
    let auth_msg = serde_json::json!({
        "action": "auth",
        "key": credentials.api_key,
        "secret": credentials.secret_key
    });

    let auth_json = serde_json::to_string(&auth_msg)?;
    debug!(
        "Sending auth message for key {}",
        redact_key(&credentials.api_key)
    );
    sink.send(Message::Text(auth_json.into())).await?;
    Ok(())
}

/// Extract the error message from a server frame, if the frame (or any
/// element of a frame array) is a `{"T": "error"}` message.
fn frame_error(text: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let frames = match &value {
        serde_json::Value::Array(items) => items.as_slice(),
        _ => std::slice::from_ref(&value),
    };
    frames.iter().find_map(|frame| {
        if frame.get("T").and_then(|t| t.as_str()) == Some("error") {
            Some(
                frame
                    .get("msg")
                    .and_then(|m| m.as_str())
                    .unwrap_or("unknown error")
                    .to_string(),
            )
        } else {
            None
        }
    })
}

/// Read the next text frame during the handshake, failing on error frames,
/// unexpected frames, or a closed connection.
async fn expect_ok_frame(stream: &mut WsReceiver, phase: &str) -> Result<()> {
    loop {
        match stream.next().await {
            Some(Ok(Message::Text(text))) => {
                debug!("{} response: {}", phase, text);
                return match frame_error(&text) {
                    Some(msg) => Err(AlpacaError::WebSocket(format!("{phase} failed: {msg}"))),
                    None => Ok(()),
                };
            }
            Some(Ok(Message::Ping(_) | Message::Pong(_))) => continue,
            Some(Ok(other)) => {
                return Err(AlpacaError::WebSocket(format!(
                    "{phase} failed: unexpected frame: {other:?}"
                )));
            }
            Some(Err(e)) => return Err(e.into()),
            None => {
                return Err(AlpacaError::WebSocket(format!(
                    "{phase} failed: connection closed"
                )));
            }
        }
    }
}

/// Connect, authenticate, and subscribe on a market-data socket, bounded by
/// the configured connection timeout. Performs the full handshake (server
/// hello, auth, subscription) so the returned stream only yields data frames.
async fn open_market_data_stream(
    url: &str,
    credentials: &Credentials,
    subscription: &SubscribeMessage,
    config: &WebSocketConfig,
) -> Result<WsReceiver> {
    let handshake = async {
        info!("Connecting to WebSocket: {}", url);
        let (ws_stream, _) = connect_async(url).await?;
        let (mut sink, mut stream) = ws_stream.split();

        expect_ok_frame(&mut stream, "server hello").await?;

        send_auth(credentials, &mut sink).await?;
        expect_ok_frame(&mut stream, "authentication").await?;

        // Alpaca uses {"action": "subscribe", ...}
        let sub_msg = serde_json::json!({
            "action": "subscribe",
            "trades": subscription.trades.clone().unwrap_or_default(),
            "quotes": subscription.quotes.clone().unwrap_or_default(),
            "bars": subscription.bars.clone().unwrap_or_default()
        });
        let sub_json = serde_json::to_string(&sub_msg)?;
        debug!("Sending subscription: {}", sub_json);
        sink.send(Message::Text(sub_json.into())).await?;
        expect_ok_frame(&mut stream, "subscription").await?;

        Ok(stream)
    };

    match timeout(
        Duration::from_millis(config.connection_timeout_ms),
        handshake,
    )
    .await
    {
        Ok(result) => result,
        Err(_) => Err(AlpacaError::WebSocket(format!(
            "handshake timed out after {}ms",
            config.connection_timeout_ms
        ))),
    }
}

/// Parse a market-data text frame (a JSON array of messages) into updates.
fn parse_market_data_updates(text: &str) -> Vec<MarketDataUpdate> {
    let Ok(messages) = serde_json::from_str::<Vec<serde_json::Value>>(text) else {
        return Vec::new();
    };
    messages
        .into_iter()
        .filter_map(|msg_value| {
            let msg_type = msg_value.get("T").and_then(|t| t.as_str())?;
            match msg_type {
                "t" => serde_json::from_value::<TradeMessage>(msg_value.clone())
                    .ok()
                    .map(|trade_msg| MarketDataUpdate::Trade {
                        symbol: trade_msg.symbol.clone(),
                        trade: trade_msg.into(),
                    }),
                // Quote message - try crypto format first
                "q" => {
                    if let Ok(quote_msg) =
                        serde_json::from_value::<CryptoQuoteMessage>(msg_value.clone())
                    {
                        Some(MarketDataUpdate::Quote {
                            symbol: quote_msg.symbol.clone(),
                            quote: Quote {
                                timestamp: quote_msg.timestamp,
                                timeframe: "real-time".to_string(),
                                bid_price: quote_msg.bid_price,
                                bid_size: quote_msg.bid_size as u32,
                                ask_price: quote_msg.ask_price,
                                ask_size: quote_msg.ask_size as u32,
                                bid_exchange: String::new(),
                                ask_exchange: String::new(),
                            },
                        })
                    } else {
                        serde_json::from_value::<QuoteMessage>(msg_value)
                            .ok()
                            .map(|quote_msg| MarketDataUpdate::Quote {
                                symbol: quote_msg.symbol.clone(),
                                quote: quote_msg.into(),
                            })
                    }
                }
                "b" => serde_json::from_value::<BarMessage>(msg_value)
                    .ok()
                    .map(|bar_msg| MarketDataUpdate::Bar {
                        symbol: bar_msg.symbol.clone(),
                        bar: bar_msg.into(),
                    }),
                _ => {
                    debug!("Ignoring message type: {}", msg_type);
                    None
                }
            }
        })
        .collect()
}

/// Forward a data update without blocking the socket reader. When the
/// bounded channel is full the update is dropped and counted; the count is
/// delivered later as a [`MarketDataEvent::Lagged`] event. `Err(())` means
/// the consumer dropped the stream.
fn send_update(
    sender: &mpsc::Sender<MarketDataEvent>,
    missed: &mut u64,
    update: MarketDataUpdate,
) -> std::result::Result<(), ()> {
    if *missed > 0 {
        match sender.try_send(MarketDataEvent::Lagged { missed: *missed }) {
            Ok(()) => *missed = 0,
            Err(TrySendError::Full(_)) => {
                *missed += 1;
                return Ok(());
            }
            Err(TrySendError::Closed(_)) => return Err(()),
        }
    }
    match sender.try_send(MarketDataEvent::Update(update)) {
        Ok(()) => Ok(()),
        Err(TrySendError::Full(_)) => {
            *missed += 1;
            Ok(())
        }
        Err(TrySendError::Closed(_)) => Err(()),
    }
}

/// Forward a lifecycle event, waiting for channel capacity so it is never
/// dropped. Flushes any pending lag count first. Returns `false` when the
/// consumer dropped the stream.
async fn send_lifecycle(
    sender: &mpsc::Sender<MarketDataEvent>,
    missed: &mut u64,
    event: MarketDataEvent,
) -> bool {
    if *missed > 0 {
        if sender
            .send(MarketDataEvent::Lagged { missed: *missed })
            .await
            .is_err()
        {
            return false;
        }
        *missed = 0;
    }
    sender.send(event).await.is_ok()
}

/// Background task that owns the market-data socket: reads frames, forwards
/// events to the consumer, and reconnects with capped exponential backoff,
/// re-issuing the active subscription set after each reconnect. Exits when
/// the consumer drops the stream or reconnection gives up.
async fn run_market_data_task(
    mut stream: WsReceiver,
    url: String,
    credentials: Credentials,
    subscription: SubscribeMessage,
    config: WebSocketConfig,
    sender: mpsc::Sender<MarketDataEvent>,
) {
    let mut missed: u64 = 0;
    'connection: loop {
        let mut reason = loop {
            match stream.next().await {
                Some(Ok(Message::Text(text))) => {
                    for update in parse_market_data_updates(&text) {
                        if send_update(&sender, &mut missed, update).is_err() {
                            debug!("Market data stream dropped by consumer");
                            return;
                        }
                    }
                }
                Some(Ok(Message::Close(_))) => break "server closed the connection".to_string(),
                Some(Ok(_)) => {}
                Some(Err(e)) => break format!("websocket error: {e}"),
                None => break "connection ended".to_string(),
            }
        };

        if !config.reconnect_enabled {
            let _ = send_lifecycle(
                &sender,
                &mut missed,
                MarketDataEvent::Disconnected { reason },
            )
            .await;
            return;
        }

        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            if attempt > config.reconnect_max_attempts {
                error!(
                    "Market data reconnection gave up after {} attempts",
                    config.reconnect_max_attempts
                );
                let _ = send_lifecycle(
                    &sender,
                    &mut missed,
                    MarketDataEvent::Disconnected {
                        reason: format!(
                            "gave up after {} reconnect attempts: {}",
                            config.reconnect_max_attempts, reason
                        ),
                    },
                )
                .await;
                return;
            }

            let delay = Duration::from_millis(
                config
                    .reconnect_base_delay_ms
                    .saturating_mul(1u64 << (attempt - 1).min(16))
                    .min(config.reconnect_max_delay_ms),
            );
            warn!(
                "Market data connection lost ({}); reconnecting in {:?} (attempt {}/{})",
                reason, delay, attempt, config.reconnect_max_attempts
            );
            if !send_lifecycle(
                &sender,
                &mut missed,
                MarketDataEvent::Reconnecting { attempt, delay },
            )
            .await
            {
                return;
            }
            sleep(delay).await;

            match open_market_data_stream(&url, &credentials, &subscription, &config).await {
                Ok(new_stream) => {
                    stream = new_stream;
                    info!("Market data connection re-established and resubscribed");
                    if !send_lifecycle(&sender, &mut missed, MarketDataEvent::Reconnected).await {
                        return;
                    }
                    continue 'connection;
                }
                Err(e) => {
                    reason = format!("reconnect attempt {attempt} failed: {e}");
                }
            }
        }
    }
}

/// WebSocket connection manager with automatic reconnection
pub struct WebSocketManager {
    client: AlpacaWebSocketClient,
    max_retries: u32,
    heartbeat_interval: Duration,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new(client: AlpacaWebSocketClient) -> Self {
        Self {
            client,
            max_retries: 5,
            heartbeat_interval: Duration::from_secs(30),
        }
    }

    /// Set maximum retry attempts
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set heartbeat interval
    pub fn with_heartbeat_interval(mut self, interval: Duration) -> Self {
        self.heartbeat_interval = interval;
        self
    }

    /// Start the managed connection
    pub async fn start(&self) -> Result<AlpacaStream> {
        let stream = self.client.connect_with_reconnect(self.max_retries).await?;

        // Start heartbeat
        self.start_heartbeat().await;

        Ok(stream)
    }

    /// Start heartbeat to keep connection alive
    async fn start_heartbeat(&self) {
        let mut interval = interval(self.heartbeat_interval);

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                debug!("Heartbeat tick");
                // In a real implementation, you might send a ping message here
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alpaca_base::types::Environment;

    #[test]
    fn test_client_creation() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = AlpacaWebSocketClient::new(credentials, Environment::Paper);

        assert!(client.url().contains("stream.data.alpaca.markets"));
    }

    #[test]
    fn test_trading_client() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = AlpacaWebSocketClient::trading(credentials, Environment::Paper);

        assert!(client.url().contains("paper-api.alpaca.markets"));
    }

    #[test]
    fn test_parse_message() {
        let json = r#"{"T":"success","msg":"authenticated"}"#;
        let result = AlpacaWebSocketClient::parse_message(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_redact_key() {
        assert_eq!(redact_key("PKABCDEFGHIJKLMNOP"), "****MNOP");
        assert_eq!(redact_key("short"), "****");
        assert_eq!(redact_key(""), "****");
    }

    #[test]
    fn test_frame_error() {
        assert_eq!(
            frame_error(r#"[{"T":"error","code":402,"msg":"auth failed"}]"#),
            Some("auth failed".to_string())
        );
        assert_eq!(
            frame_error(r#"{"T":"error","code":405,"msg":"symbol limit exceeded"}"#),
            Some("symbol limit exceeded".to_string())
        );
        assert_eq!(frame_error(r#"[{"T":"success","msg":"connected"}]"#), None);
        assert_eq!(frame_error("not json"), None);
    }

    #[test]
    fn test_parse_market_data_updates() {
        let text = r#"[
            {"T":"t","S":"AAPL","t":"2026-07-13T10:00:00Z","p":190.5,"s":100,"x":"V","c":[],"i":1},
            {"T":"b","S":"AAPL","t":"2026-07-13T10:00:00Z","o":190.0,"h":191.0,"l":189.5,"c":190.5,"v":1000},
            {"T":"subscription","trades":["AAPL"]}
        ]"#;
        let updates = parse_market_data_updates(text);
        assert_eq!(updates.len(), 2);
        assert!(matches!(&updates[0], MarketDataUpdate::Trade { symbol, .. } if symbol == "AAPL"));
        assert!(matches!(&updates[1], MarketDataUpdate::Bar { symbol, .. } if symbol == "AAPL"));
        assert!(parse_market_data_updates("not json").is_empty());
    }

    #[test]
    fn test_with_feed_urls() {
        let cases = [
            (DataFeed::Iex, "wss://stream.data.alpaca.markets/v2/iex"),
            (DataFeed::Sip, "wss://stream.data.alpaca.markets/v2/sip"),
            (
                DataFeed::DelayedSip,
                "wss://stream.data.alpaca.markets/v2/delayed_sip",
            ),
            (
                DataFeed::Boats,
                "wss://stream.data.alpaca.markets/v1beta1/boats",
            ),
            (
                DataFeed::Overnight,
                "wss://stream.data.alpaca.markets/v1beta1/overnight",
            ),
            (
                DataFeed::Crypto,
                "wss://stream.data.alpaca.markets/v1beta3/crypto/us",
            ),
        ];

        for (feed, expected_url) in cases {
            let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
            let client = AlpacaWebSocketClient::with_feed(credentials, Environment::Paper, feed);
            assert_eq!(client.url(), expected_url);
        }
    }
}
