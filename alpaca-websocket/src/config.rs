//! WebSocket configuration types.

/// Configuration for WebSocket connections.
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// Whether automatic reconnection is enabled.
    pub reconnect_enabled: bool,
    /// Maximum number of reconnection attempts.
    pub reconnect_max_attempts: u32,
    /// Base delay between reconnection attempts in milliseconds.
    pub reconnect_base_delay_ms: u64,
    /// Maximum delay between reconnection attempts in milliseconds.
    pub reconnect_max_delay_ms: u64,
    /// Interval for sending ping messages in milliseconds.
    pub ping_interval_ms: u64,
    /// Size of the message buffer.
    pub message_buffer_size: usize,
    /// Connection timeout in milliseconds.
    pub connection_timeout_ms: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            reconnect_enabled: true,
            reconnect_max_attempts: 10,
            reconnect_base_delay_ms: 1000,
            reconnect_max_delay_ms: 60000,
            ping_interval_ms: 30000,
            message_buffer_size: 1000,
            connection_timeout_ms: 10000,
        }
    }
}

impl WebSocketConfig {
    /// Create a new configuration with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable automatic reconnection.
    #[must_use]
    pub fn no_reconnect(mut self) -> Self {
        self.reconnect_enabled = false;
        self
    }

    /// Set maximum reconnection attempts.
    #[must_use]
    pub fn max_reconnect_attempts(mut self, attempts: u32) -> Self {
        self.reconnect_max_attempts = attempts;
        self
    }

    /// Set base delay for reconnection in milliseconds.
    #[must_use]
    pub fn reconnect_base_delay(mut self, delay_ms: u64) -> Self {
        self.reconnect_base_delay_ms = delay_ms;
        self
    }

    /// Set ping interval in milliseconds.
    #[must_use]
    pub fn ping_interval(mut self, interval_ms: u64) -> Self {
        self.ping_interval_ms = interval_ms;
        self
    }

    /// Set message buffer size.
    #[must_use]
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.message_buffer_size = size;
        self
    }

    /// Set connection timeout in milliseconds.
    #[must_use]
    pub fn connection_timeout(mut self, timeout_ms: u64) -> Self {
        self.connection_timeout_ms = timeout_ms;
        self
    }
}

/// WebSocket stream type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    /// Stock market data stream (IEX or SIP).
    Stocks,
    /// Cryptocurrency market data stream.
    Crypto,
    /// Options market data stream.
    Options,
    /// News stream.
    News,
    /// Trading updates stream (order fills, etc.).
    Trading,
}

impl StreamType {
    /// Get the WebSocket URL for this stream type.
    #[must_use]
    pub fn url(&self, is_paper: bool) -> &'static str {
        match self {
            StreamType::Stocks => {
                if is_paper {
                    "wss://stream.data.alpaca.markets/v2/iex"
                } else {
                    "wss://stream.data.alpaca.markets/v2/sip"
                }
            }
            StreamType::Crypto => "wss://stream.data.alpaca.markets/v1beta3/crypto/us",
            StreamType::Options => "wss://stream.data.alpaca.markets/v1beta1/options",
            StreamType::News => "wss://stream.data.alpaca.markets/v1beta1/news",
            StreamType::Trading => {
                if is_paper {
                    "wss://paper-api.alpaca.markets/stream"
                } else {
                    "wss://api.alpaca.markets/stream"
                }
            }
        }
    }
}

/// Connection state for WebSocket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected.
    Disconnected,
    /// Attempting to connect.
    Connecting,
    /// Connected and authenticated.
    Connected,
    /// Reconnecting after disconnect.
    Reconnecting,
    /// Connection failed permanently.
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_config_default() {
        let config = WebSocketConfig::default();
        assert!(config.reconnect_enabled);
        assert_eq!(config.reconnect_max_attempts, 10);
        assert_eq!(config.reconnect_base_delay_ms, 1000);
    }

    #[test]
    fn test_websocket_config_builder() {
        let config = WebSocketConfig::new()
            .no_reconnect()
            .max_reconnect_attempts(5)
            .ping_interval(15000)
            .buffer_size(500);

        assert!(!config.reconnect_enabled);
        assert_eq!(config.reconnect_max_attempts, 5);
        assert_eq!(config.ping_interval_ms, 15000);
        assert_eq!(config.message_buffer_size, 500);
    }

    #[test]
    fn test_stream_type_urls() {
        assert_eq!(
            StreamType::Stocks.url(true),
            "wss://stream.data.alpaca.markets/v2/iex"
        );
        assert_eq!(
            StreamType::Stocks.url(false),
            "wss://stream.data.alpaca.markets/v2/sip"
        );
        assert_eq!(
            StreamType::Crypto.url(true),
            "wss://stream.data.alpaca.markets/v1beta3/crypto/us"
        );
        assert_eq!(
            StreamType::Options.url(true),
            "wss://stream.data.alpaca.markets/v1beta1/options"
        );
    }
}
