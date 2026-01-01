use alpaca_base::AlpacaError;
use thiserror::Error;

/// WebSocket-specific errors for the Alpaca client
#[derive(Error, Debug)]
pub enum WebSocketError {
    /// Wrapped base Alpaca error
    #[error(transparent)]
    Base(#[from] AlpacaError),

    /// WebSocket connection errors
    #[error("WebSocket connection error: {0}")]
    Connection(#[from] tokio_tungstenite::tungstenite::Error),

    /// URL parsing errors
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    /// Connection closed unexpectedly
    #[error("Connection closed: {0}")]
    ConnectionClosed(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Subscription error
    #[error("Subscription error: {0}")]
    Subscription(String),

    /// Message parsing error
    #[error("Message parsing error: {0}")]
    MessageParsing(String),

    /// Channel send error
    #[error("Channel send error")]
    ChannelSend,

    /// Reconnection failed
    #[error("Reconnection failed after {attempts} attempts")]
    ReconnectionFailed { attempts: u32 },
}
