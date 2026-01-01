use crate::error::{AlpacaError, Result};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

/// Authentication credentials for Alpaca API
#[derive(Debug, Clone)]
pub struct Credentials {
    pub api_key: String,
    pub secret_key: String,
}

impl Credentials {
    /// Create new credentials
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
        }
    }

    /// Create credentials from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("ALPACA_API_KEY")
            .map_err(|_| AlpacaError::Config("ALPACA_API_KEY not found".to_string()))?;
        let secret_key = std::env::var("ALPACA_SECRET_KEY")
            .map_err(|_| AlpacaError::Config("ALPACA_SECRET_KEY not found".to_string()))?;

        Ok(Self::new(api_key, secret_key))
    }

    /// Generate authorization header for HTTP requests
    pub fn auth_header(&self) -> String {
        format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("{}:{}", self.api_key, self.secret_key))
        )
    }

    /// Generate HMAC signature for request authentication
    pub fn sign_request(
        &self,
        method: &str,
        path: &str,
        body: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<String> {
        let timestamp_str = timestamp.timestamp().to_string();
        let message = format!("{}{}{}{}", timestamp_str, method, path, body);

        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|e| AlpacaError::Auth(format!("Invalid secret key: {}", e)))?;

        mac.update(message.as_bytes());
        let result = mac.finalize();

        Ok(general_purpose::STANDARD.encode(result.into_bytes()))
    }

    /// Generate headers for authenticated requests
    pub fn auth_headers(
        &self,
        method: &str,
        path: &str,
        body: &str,
    ) -> Result<HashMap<String, String>> {
        let timestamp = Utc::now();
        let signature = self.sign_request(method, path, body, timestamp)?;

        let mut headers = HashMap::new();
        headers.insert("APCA-API-KEY-ID".to_string(), self.api_key.clone());
        headers.insert("APCA-API-SECRET-KEY".to_string(), self.secret_key.clone());
        headers.insert(
            "APCA-API-TIMESTAMP".to_string(),
            timestamp.timestamp().to_string(),
        );
        headers.insert("APCA-API-SIGNATURE".to_string(), signature);

        Ok(headers)
    }
}

/// OAuth token for API access
#[derive(Debug, Clone)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
}

impl OAuthToken {
    /// Create authorization header from OAuth token
    pub fn auth_header(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}
