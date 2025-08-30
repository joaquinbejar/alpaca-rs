use crate::error::{AlpacaError, Result};

/// Alpaca Markets API client
pub struct Client {
    api_key: String,
    secret_key: String,
    base_url: String,
}

impl Client {
    /// Create a new Alpaca client
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
            base_url: "https://paper-api.alpaca.markets".to_string(),
        }
    }

    /// Create a new Alpaca client for live trading
    pub fn new_live(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
            base_url: "https://api.alpaca.markets".to_string(),
        }
    }

    /// Get account information
    pub async fn get_account(&self) -> Result<()> {
        // TODO: Implement account endpoint
        Err(AlpacaError::Api("Not implemented yet".to_string()))
    }
}
