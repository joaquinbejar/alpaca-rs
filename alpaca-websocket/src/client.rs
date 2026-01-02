//! WebSocket client for Alpaca streaming data.

#![allow(missing_docs)]

use crate::{messages::*, streams::*};
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
    time::{interval, sleep},
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

impl AlpacaWebSocketClient {
    /// Create a new WebSocket client
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

    /// Create a trading WebSocket client
    pub fn trading(credentials: Credentials, environment: Environment) -> Self {
        let url = environment.websocket_url();
        Self {
            credentials,
            environment,
            url: url.to_string(),
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

    /// Subscribe to market data
    pub async fn subscribe_market_data(
        &self,
        _subscription: SubscribeMessage,
    ) -> Result<MarketDataStream> {
        let stream = self.connect().await?;
        let (sender, receiver) = mpsc::unbounded_channel();

        // Send subscription
        // Note: In a real implementation, you'd need to send the subscription message
        // through the WebSocket connection. This is simplified for the example.

        tokio::spawn(async move {
            let mut market_data_stream = stream.market_data();
            while let Some(update) = market_data_stream.next().await {
                if sender.send(update).is_err() {
                    break;
                }
            }
        });

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
        let auth_msg = WebSocketMessage::Auth(AuthMessage {
            key: self.credentials.api_key.clone(),
            secret: self.credentials.secret_key.clone(),
        });

        let auth_json = serde_json::to_string(&auth_msg)?;
        sink.send(Message::Text(auth_json.into())).await?;

        debug!("Sent authentication message");
        Ok(())
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
}
