use alpaca_base::{AlpacaError, Result, auth::Credentials, types::Environment, utils::UrlBuilder};
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;
use tracing::{debug, error};

/// HTTP client for Alpaca API
#[derive(Debug, Clone)]
pub struct AlpacaHttpClient {
    client: Client,
    credentials: Credentials,
    environment: Environment,
    base_url: String,
    data_url: String,
}

impl AlpacaHttpClient {
    /// Create a new HTTP client
    pub fn new(credentials: Credentials, environment: Environment) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("alpaca-rs/0.1.0")
            .build()
            .map_err(|e| AlpacaError::Http(e.to_string()))?;

        Ok(Self {
            client,
            credentials,
            base_url: environment.base_url().to_string(),
            data_url: environment.data_url().to_string(),
            environment,
        })
    }

    /// Create a new client from environment variables
    pub fn from_env(environment: Environment) -> Result<Self> {
        let credentials = Credentials::from_env()?;
        Self::new(credentials, environment)
    }

    /// Make a GET request
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request::<T, ()>(Method::GET, path, None).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_params<T, P>(&self, path: &str, params: &P) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Serialize params to query string
        let query_string = serde_urlencoded::to_string(params)
            .map_err(|e| AlpacaError::Json(format!("Failed to serialize query params: {}", e)))?;

        let url = if query_string.is_empty() {
            self.build_url(path)?
        } else {
            format!("{}?{}", self.build_url(path)?, query_string)
        };

        let request = self.client.get(&url).headers(self.build_headers()?);

        self.execute_request(request).await
    }

    /// Make a POST request
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request(Method::POST, path, Some(body)).await
    }

    /// Make a PUT request
    pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// Make a PATCH request
    pub async fn patch<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request(Method::PATCH, path, Some(body)).await
    }

    /// Make a DELETE request
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request::<T, ()>(Method::DELETE, path, None).await
    }

    /// Make a generic request
    async fn request<T, B>(&self, method: Method, path: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.build_url(path)?;
        let mut request = self
            .client
            .request(method.clone(), &url)
            .headers(self.build_headers()?);

        if let Some(body) = body {
            request = request.json(body);
        }

        debug!("Making {} request to {}", method, url);
        self.execute_request(request).await
    }

    /// Execute the request and handle the response
    async fn execute_request<T>(&self, request: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = request
            .send()
            .await
            .map_err(|e| AlpacaError::Network(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Handle the HTTP response
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let headers = response.headers().clone();

        debug!("Response status: {}", status);

        // Check for rate limiting
        if status == 429 {
            let retry_after = headers
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("60");
            return Err(AlpacaError::RateLimit(format!(
                "Retry after {} seconds",
                retry_after
            )));
        }

        // Get response text for error handling
        let response_text = response
            .text()
            .await
            .map_err(|e| AlpacaError::Network(e.to_string()))?;

        if !status.is_success() {
            error!("API error response: {}", response_text);

            // Try to parse error response
            if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(&response_text) {
                let message = error_response
                    .get("message")
                    .or_else(|| error_response.get("error"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&response_text);

                return Err(AlpacaError::Api {
                    code: status.as_u16(),
                    message: message.to_string(),
                });
            }

            return Err(AlpacaError::Api {
                code: status.as_u16(),
                message: response_text,
            });
        }

        // Parse successful response
        serde_json::from_str(&response_text).map_err(|e| {
            AlpacaError::Json(format!(
                "Failed to parse response: {} - Response: {}",
                e, response_text
            ))
        })
    }

    /// Build the full URL for a request
    fn build_url(&self, path: &str) -> Result<String> {
        // Use data URL for market data endpoints
        let base_url = if path.starts_with("/v2/stocks") || path.starts_with("/v1beta1/crypto") {
            &self.data_url
        } else {
            &self.base_url
        };

        UrlBuilder::new(base_url)
            .path(path.trim_start_matches('/'))
            .build()
    }

    /// Build authentication headers
    fn build_headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            "APCA-API-KEY-ID",
            self.credentials
                .api_key
                .parse()
                .map_err(|_| AlpacaError::Auth("Invalid API key format".to_string()))?,
        );

        headers.insert(
            "APCA-API-SECRET-KEY",
            self.credentials
                .secret_key
                .parse()
                .map_err(|_| AlpacaError::Auth("Invalid secret key format".to_string()))?,
        );

        headers.insert("Content-Type", "application/json".parse().unwrap());

        Ok(headers)
    }

    /// Get the current environment
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the data URL
    pub fn data_url(&self) -> &str {
        &self.data_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alpaca_base::types::Environment;

    #[test]
    fn test_build_url() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = AlpacaHttpClient::new(credentials, Environment::Paper).unwrap();

        let url = client.build_url("/v2/account").unwrap();
        assert_eq!(url, "https://paper-api.alpaca.markets/v2/account");

        let data_url = client.build_url("/v2/stocks/AAPL/bars").unwrap();
        assert_eq!(data_url, "https://data.alpaca.markets/v2/stocks/AAPL/bars");
    }

    #[test]
    fn test_environment_urls() {
        assert_eq!(
            Environment::Paper.base_url(),
            "https://paper-api.alpaca.markets"
        );
        assert_eq!(Environment::Live.base_url(), "https://api.alpaca.markets");
        assert_eq!(Environment::Paper.data_url(), "https://data.alpaca.markets");
    }
}
