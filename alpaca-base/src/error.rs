use thiserror::Error;

/// Result type for Alpaca API operations
pub type Result<T> = std::result::Result<T, AlpacaError>;

/// Error types for the Alpaca API client
#[derive(Error, Debug)]
pub enum AlpacaError {
    /// HTTP request errors
    #[error("HTTP error: {0}")]
    Http(String),

    /// JSON parsing errors
    #[error("JSON error: {0}")]
    Json(String),

    /// API errors returned by Alpaca
    #[error("API error: {code} - {message}")]
    Api { code: u16, message: String },

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),

    /// WebSocket errors
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Rate limiting errors
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Network connectivity errors
    #[error("Network error: {0}")]
    Network(String),

    /// Timeout errors
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// Invalid data format
    #[error("Invalid data format: {0}")]
    InvalidData(String),
}

impl From<serde_json::Error> for AlpacaError {
    fn from(err: serde_json::Error) -> Self {
        AlpacaError::Json(err.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for AlpacaError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        AlpacaError::WebSocket(err.to_string())
    }
}
