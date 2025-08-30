use std::fmt;

/// Result type for Alpaca API operations
pub type Result<T> = std::result::Result<T, AlpacaError>;

/// Error types for the Alpaca API client
#[derive(Debug)]
pub enum AlpacaError {
    /// HTTP request errors
    Http(String),
    /// JSON parsing errors
    Json(String),
    /// API errors returned by Alpaca
    Api(String),
    /// Authentication errors
    Auth(String),
    /// Invalid configuration
    Config(String),
}

impl fmt::Display for AlpacaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlpacaError::Http(msg) => write!(f, "HTTP error: {}", msg),
            AlpacaError::Json(msg) => write!(f, "JSON error: {}", msg),
            AlpacaError::Api(msg) => write!(f, "API error: {}", msg),
            AlpacaError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AlpacaError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for AlpacaError {}
