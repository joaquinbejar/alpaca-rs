//! HTTP-specific error types.

use alpaca_base::AlpacaError;
use thiserror::Error;

/// HTTP-specific errors for the Alpaca client.
#[derive(Error, Debug)]
pub enum HttpError {
    /// Wrapped base Alpaca error.
    #[error(transparent)]
    Base(#[from] AlpacaError),

    /// HTTP client errors.
    #[error("HTTP client error: {0}")]
    Client(#[from] reqwest::Error),

    /// URL parsing errors.
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    /// Request timeout.
    #[error("Request timeout")]
    Timeout,

    /// Too many requests (rate limited).
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// Server error with status code and message.
    #[error("Server error: {status} - {message}")]
    Server {
        /// HTTP status code.
        status: u16,
        /// Error message.
        message: String,
    },
}
