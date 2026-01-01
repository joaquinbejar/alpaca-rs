//! FIX protocol error types.

use thiserror::Error;

/// FIX protocol errors.
#[derive(Debug, Error)]
pub enum FixError {
    /// Connection error.
    #[error("connection error: {0}")]
    Connection(String),

    /// Session error.
    #[error("session error: {0}")]
    Session(String),

    /// Message encoding error.
    #[error("encoding error: {0}")]
    Encoding(String),

    /// Message decoding error.
    #[error("decoding error: {0}")]
    Decoding(String),

    /// Invalid message format.
    #[error("invalid message: {0}")]
    InvalidMessage(String),

    /// Sequence number error.
    #[error("sequence error: expected {expected}, got {actual}")]
    SequenceError {
        /// Expected sequence number.
        expected: u64,
        /// Actual sequence number.
        actual: u64,
    },

    /// Authentication error.
    #[error("authentication error: {0}")]
    Authentication(String),

    /// Timeout error.
    #[error("timeout: {0}")]
    Timeout(String),

    /// IO error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error.
    #[error("configuration error: {0}")]
    Configuration(String),

    /// Rejected message.
    #[error("message rejected: {0}")]
    Rejected(String),
}

/// Result type for FIX operations.
pub type Result<T> = std::result::Result<T, FixError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_error_display() {
        let err = FixError::Connection("failed to connect".to_string());
        assert_eq!(err.to_string(), "connection error: failed to connect");
    }

    #[test]
    fn test_sequence_error() {
        let err = FixError::SequenceError {
            expected: 10,
            actual: 5,
        };
        assert_eq!(err.to_string(), "sequence error: expected 10, got 5");
    }
}
