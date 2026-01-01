use crate::error::{AlpacaError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Generate a random client order ID
pub fn generate_client_order_id() -> String {
    Uuid::new_v4().to_string()
}

/// Parse a string to a decimal value with validation
pub fn parse_decimal(value: &str) -> Result<f64> {
    value
        .parse::<f64>()
        .map_err(|_| AlpacaError::InvalidData(format!("Invalid decimal value: {}", value)))
}

/// Format decimal value to string with specified precision
pub fn format_decimal(value: f64, precision: usize) -> String {
    format!("{:.prec$}", value, prec = precision)
}

/// Validate symbol format
pub fn validate_symbol(symbol: &str) -> Result<()> {
    if symbol.is_empty() {
        return Err(AlpacaError::InvalidData(
            "Symbol cannot be empty".to_string(),
        ));
    }

    if symbol.len() > 12 {
        return Err(AlpacaError::InvalidData("Symbol too long".to_string()));
    }

    if !symbol
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    {
        return Err(AlpacaError::InvalidData(
            "Invalid symbol format".to_string(),
        ));
    }

    Ok(())
}

/// Validate quantity value
pub fn validate_quantity(qty: &str) -> Result<()> {
    let value = parse_decimal(qty)?;

    if value <= 0.0 {
        return Err(AlpacaError::InvalidData(
            "Quantity must be positive".to_string(),
        ));
    }

    Ok(())
}

/// Validate price value
pub fn validate_price(price: &str) -> Result<()> {
    let value = parse_decimal(price)?;

    if value <= 0.0 {
        return Err(AlpacaError::InvalidData(
            "Price must be positive".to_string(),
        ));
    }

    Ok(())
}

/// Convert timestamp to RFC3339 format
pub fn timestamp_to_rfc3339(timestamp: DateTime<Utc>) -> String {
    timestamp.to_rfc3339()
}

/// Parse RFC3339 timestamp
pub fn parse_rfc3339(timestamp: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| AlpacaError::InvalidData(format!("Invalid timestamp format: {}", e)))
}

/// Rate limiter for API requests
#[derive(Debug)]
pub struct RateLimiter {
    requests_per_minute: u32,
    last_reset: DateTime<Utc>,
    current_count: u32,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            last_reset: Utc::now(),
            current_count: 0,
        }
    }

    /// Check if a request can be made
    pub fn can_make_request(&mut self) -> bool {
        let now = Utc::now();

        // Reset counter if a minute has passed
        if now.signed_duration_since(self.last_reset).num_seconds() >= 60 {
            self.last_reset = now;
            self.current_count = 0;
        }

        if self.current_count < self.requests_per_minute {
            self.current_count += 1;
            true
        } else {
            false
        }
    }

    /// Get remaining requests in current window
    pub fn remaining_requests(&self) -> u32 {
        self.requests_per_minute.saturating_sub(self.current_count)
    }
}

/// Pagination parameters for API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page_token: Option<String>,
    pub limit: Option<u32>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page_token: None,
            limit: Some(100),
        }
    }
}

/// Response wrapper with pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub next_page_token: Option<String>,
}

/// Retry configuration for API requests
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Logger configuration
pub fn init_logger() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .map_err(|e| AlpacaError::Config(format!("Failed to initialize logger: {}", e)))
}

/// URL builder helper
#[derive(Debug)]
pub struct UrlBuilder {
    base_url: String,
    path: String,
    query_params: Vec<(String, String)>,
}

impl UrlBuilder {
    /// Create a new URL builder
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            path: String::new(),
            query_params: Vec::new(),
        }
    }

    /// Add path segment
    pub fn path(mut self, segment: &str) -> Self {
        if !self.path.is_empty() && !self.path.ends_with('/') {
            self.path.push('/');
        }
        self.path.push_str(segment);
        self
    }

    /// Add query parameter
    pub fn query<T: fmt::Display>(mut self, key: &str, value: T) -> Self {
        self.query_params.push((key.to_string(), value.to_string()));
        self
    }

    /// Add optional query parameter
    pub fn query_opt<T: fmt::Display>(self, key: &str, value: Option<T>) -> Self {
        match value {
            Some(v) => self.query(key, v),
            None => self,
        }
    }

    /// Build the final URL
    pub fn build(self) -> Result<String> {
        let mut url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            self.path.trim_start_matches('/')
        );

        if !self.query_params.is_empty() {
            url.push('?');
            for (i, (key, value)) in self.query_params.iter().enumerate() {
                if i > 0 {
                    url.push('&');
                }
                url.push_str(&urlencoding::encode(key));
                url.push('=');
                url.push_str(&urlencoding::encode(value));
            }
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_client_order_id() {
        let id1 = generate_client_order_id();
        let id2 = generate_client_order_id();
        assert_ne!(id1, id2);
        assert!(Uuid::parse_str(&id1).is_ok());
    }

    #[test]
    fn test_validate_symbol() {
        assert!(validate_symbol("AAPL").is_ok());
        assert!(validate_symbol("BRK.A").is_ok());
        assert!(validate_symbol("").is_err());
        assert!(validate_symbol("VERYLONGSYMBOL").is_err());
    }

    #[test]
    fn test_validate_quantity() {
        assert!(validate_quantity("100").is_ok());
        assert!(validate_quantity("0.5").is_ok());
        assert!(validate_quantity("0").is_err());
        assert!(validate_quantity("-10").is_err());
        assert!(validate_quantity("invalid").is_err());
    }

    #[test]
    fn test_url_builder() {
        let url = UrlBuilder::new("https://api.example.com")
            .path("v2/orders")
            .query("symbol", "AAPL")
            .query("limit", 100)
            .build()
            .unwrap();

        assert_eq!(
            url,
            "https://api.example.com/v2/orders?symbol=AAPL&limit=100"
        );
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2);
        assert!(limiter.can_make_request());
        assert!(limiter.can_make_request());
        assert!(!limiter.can_make_request());
        assert_eq!(limiter.remaining_requests(), 0);
    }
}
