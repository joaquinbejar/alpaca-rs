//! Error types for the Alpaca API client.
//!
//! This module provides comprehensive error handling with typed errors
//! for all API error responses, including Alpaca-specific error codes.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Result type for Alpaca API operations.
pub type Result<T> = std::result::Result<T, AlpacaError>;

/// Alpaca-specific API error codes.
///
/// These codes are returned by the Alpaca API to indicate specific error conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ApiErrorCode {
    /// Malformed request body or parameters.
    MalformedRequest = 40010000,
    /// Invalid or missing authentication credentials.
    InvalidCredentials = 40110000,
    /// Access forbidden for this resource.
    Forbidden = 40310000,
    /// Requested resource not found.
    NotFound = 40410000,
    /// Request validation failed.
    UnprocessableEntity = 42210000,
    /// Rate limit exceeded.
    RateLimitExceeded = 42910000,
    /// Internal server error.
    InternalServerError = 50010000,
    /// Unknown error code.
    Unknown = 0,
}

impl ApiErrorCode {
    /// Creates an `ApiErrorCode` from a numeric code.
    #[must_use]
    pub fn from_code(code: u32) -> Self {
        match code {
            40010000 => Self::MalformedRequest,
            40110000 => Self::InvalidCredentials,
            40310000 => Self::Forbidden,
            40410000 => Self::NotFound,
            42210000 => Self::UnprocessableEntity,
            42910000 => Self::RateLimitExceeded,
            50010000 => Self::InternalServerError,
            _ => Self::Unknown,
        }
    }

    /// Returns the numeric code for this error.
    #[must_use]
    pub fn as_code(&self) -> u32 {
        *self as u32
    }

    /// Returns true if this is a client error (4xx).
    #[must_use]
    pub fn is_client_error(&self) -> bool {
        let code = self.as_code();
        (40000000..50000000).contains(&code)
    }

    /// Returns true if this is a server error (5xx).
    #[must_use]
    pub fn is_server_error(&self) -> bool {
        let code = self.as_code();
        code >= 50000000
    }

    /// Returns true if this error is retryable.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::RateLimitExceeded | Self::InternalServerError)
    }
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedRequest => write!(f, "malformed request"),
            Self::InvalidCredentials => write!(f, "invalid credentials"),
            Self::Forbidden => write!(f, "forbidden"),
            Self::NotFound => write!(f, "not found"),
            Self::UnprocessableEntity => write!(f, "unprocessable entity"),
            Self::RateLimitExceeded => write!(f, "rate limit exceeded"),
            Self::InternalServerError => write!(f, "internal server error"),
            Self::Unknown => write!(f, "unknown error"),
        }
    }
}

/// Detailed API error response from Alpaca.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// The error code returned by Alpaca.
    #[serde(default)]
    pub code: u32,
    /// The error message.
    #[serde(default)]
    pub message: String,
    /// Optional request ID for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl ApiErrorResponse {
    /// Creates a new API error response.
    #[must_use]
    pub fn new(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            request_id: None,
        }
    }

    /// Sets the request ID.
    #[must_use]
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Returns the typed error code.
    #[must_use]
    pub fn error_code(&self) -> ApiErrorCode {
        ApiErrorCode::from_code(self.code)
    }
}

/// Rate limit information from API response headers.
#[derive(Debug, Clone, Default)]
pub struct RateLimitInfo {
    /// Number of requests remaining in the current window.
    pub remaining: Option<u32>,
    /// Total request limit for the current window.
    pub limit: Option<u32>,
    /// Seconds until the rate limit resets.
    pub retry_after: Option<u64>,
}

impl RateLimitInfo {
    /// Creates a new rate limit info.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the remaining requests.
    #[must_use]
    pub fn with_remaining(mut self, remaining: u32) -> Self {
        self.remaining = Some(remaining);
        self
    }

    /// Sets the limit.
    #[must_use]
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the retry after duration in seconds.
    #[must_use]
    pub fn with_retry_after(mut self, seconds: u64) -> Self {
        self.retry_after = Some(seconds);
        self
    }

    /// Returns true if rate limited.
    #[must_use]
    pub fn is_limited(&self) -> bool {
        self.remaining == Some(0)
    }
}

/// Validation error for a specific field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// The field that failed validation.
    pub field: String,
    /// The validation error message.
    pub message: String,
}

impl ValidationError {
    /// Creates a new validation error.
    #[must_use]
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error types for the Alpaca API client.
#[derive(Error, Debug)]
pub enum AlpacaError {
    /// HTTP request errors.
    #[error("http error: {0}")]
    Http(String),

    /// JSON parsing errors.
    #[error("json error: {0}")]
    Json(String),

    /// API errors returned by Alpaca with full details.
    #[error("api error {status}: {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
        /// Alpaca-specific error code.
        #[source]
        error_code: Option<ApiErrorCode>,
        /// Request ID for debugging.
        request_id: Option<String>,
    },

    /// Authentication errors.
    #[error("authentication error: {0}")]
    Auth(String),

    /// Invalid configuration.
    #[error("configuration error: {0}")]
    Config(String),

    /// WebSocket errors.
    #[error("websocket error: {0}")]
    WebSocket(String),

    /// Rate limiting errors with retry information.
    #[error("rate limit exceeded, retry after {retry_after_secs} seconds")]
    RateLimit {
        /// Seconds to wait before retrying.
        retry_after_secs: u64,
        /// Full rate limit information.
        info: RateLimitInfo,
    },

    /// Network connectivity errors.
    #[error("network error: {0}")]
    Network(String),

    /// Timeout errors.
    #[error("timeout error: {0}")]
    Timeout(String),

    /// Invalid data format.
    #[error("invalid data format: {0}")]
    InvalidData(String),

    /// Validation errors with field-level details.
    #[error("validation error: {0}")]
    Validation(String),

    /// Multiple validation errors.
    #[error("validation errors: {}", .0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))]
    ValidationErrors(Vec<ValidationError>),
}

impl AlpacaError {
    /// Creates an API error from HTTP status and message.
    #[must_use]
    pub fn api(status: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
            error_code: None,
            request_id: None,
        }
    }

    /// Creates an API error with full details.
    #[must_use]
    pub fn api_with_details(
        status: u16,
        message: impl Into<String>,
        error_code: ApiErrorCode,
        request_id: Option<String>,
    ) -> Self {
        Self::Api {
            status,
            message: message.into(),
            error_code: Some(error_code),
            request_id,
        }
    }

    /// Creates a rate limit error.
    #[must_use]
    pub fn rate_limit(retry_after_secs: u64) -> Self {
        Self::RateLimit {
            retry_after_secs,
            info: RateLimitInfo::new().with_retry_after(retry_after_secs),
        }
    }

    /// Creates a rate limit error with full info.
    #[must_use]
    pub fn rate_limit_with_info(info: RateLimitInfo) -> Self {
        Self::RateLimit {
            retry_after_secs: info.retry_after.unwrap_or(60),
            info,
        }
    }

    /// Returns true if this error is retryable.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimit { .. } => true,
            Self::Network(_) => true,
            Self::Timeout(_) => true,
            Self::Api {
                status, error_code, ..
            } => {
                // 5xx errors are retryable
                if *status >= 500 {
                    return true;
                }
                // Check Alpaca-specific error codes
                error_code.is_some_and(|code| code.is_retryable())
            }
            _ => false,
        }
    }

    /// Returns the retry-after duration in seconds, if applicable.
    #[must_use]
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Self::RateLimit {
                retry_after_secs, ..
            } => Some(*retry_after_secs),
            _ => None,
        }
    }

    /// Returns the request ID if available.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// Returns the HTTP status code if this is an API error.
    #[must_use]
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
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

// Implement std::error::Error for ApiErrorCode to use as #[source]
impl std::error::Error for ApiErrorCode {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_code_from_code() {
        assert_eq!(
            ApiErrorCode::from_code(40010000),
            ApiErrorCode::MalformedRequest
        );
        assert_eq!(
            ApiErrorCode::from_code(40110000),
            ApiErrorCode::InvalidCredentials
        );
        assert_eq!(
            ApiErrorCode::from_code(42910000),
            ApiErrorCode::RateLimitExceeded
        );
        assert_eq!(ApiErrorCode::from_code(99999999), ApiErrorCode::Unknown);
    }

    #[test]
    fn test_api_error_code_is_retryable() {
        assert!(ApiErrorCode::RateLimitExceeded.is_retryable());
        assert!(ApiErrorCode::InternalServerError.is_retryable());
        assert!(!ApiErrorCode::NotFound.is_retryable());
        assert!(!ApiErrorCode::InvalidCredentials.is_retryable());
    }

    #[test]
    fn test_api_error_code_is_client_error() {
        assert!(ApiErrorCode::MalformedRequest.is_client_error());
        assert!(ApiErrorCode::NotFound.is_client_error());
        assert!(!ApiErrorCode::InternalServerError.is_client_error());
    }

    #[test]
    fn test_api_error_code_is_server_error() {
        assert!(ApiErrorCode::InternalServerError.is_server_error());
        assert!(!ApiErrorCode::NotFound.is_server_error());
    }

    #[test]
    fn test_rate_limit_info() {
        let info = RateLimitInfo::new()
            .with_remaining(0)
            .with_limit(200)
            .with_retry_after(60);

        assert!(info.is_limited());
        assert_eq!(info.remaining, Some(0));
        assert_eq!(info.limit, Some(200));
        assert_eq!(info.retry_after, Some(60));
    }

    #[test]
    fn test_validation_error() {
        let err = ValidationError::new("qty", "must be positive");
        assert_eq!(err.field, "qty");
        assert_eq!(err.message, "must be positive");
        assert_eq!(err.to_string(), "qty: must be positive");
    }

    #[test]
    fn test_alpaca_error_is_retryable() {
        let rate_limit = AlpacaError::rate_limit(60);
        assert!(rate_limit.is_retryable());
        assert_eq!(rate_limit.retry_after(), Some(60));

        let network = AlpacaError::Network("connection reset".to_string());
        assert!(network.is_retryable());

        let auth = AlpacaError::Auth("invalid key".to_string());
        assert!(!auth.is_retryable());
    }

    #[test]
    fn test_alpaca_error_api_with_details() {
        let err = AlpacaError::api_with_details(
            404,
            "order not found",
            ApiErrorCode::NotFound,
            Some("req-123".to_string()),
        );

        assert_eq!(err.status_code(), Some(404));
        assert_eq!(err.request_id(), Some("req-123"));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_api_error_response() {
        let response = ApiErrorResponse::new(40410000, "not found").with_request_id("req-456");

        assert_eq!(response.error_code(), ApiErrorCode::NotFound);
        assert_eq!(response.request_id, Some("req-456".to_string()));
    }
}
