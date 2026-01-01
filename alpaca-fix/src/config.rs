//! FIX protocol configuration types.

use serde::{Deserialize, Serialize};

/// FIX protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FixVersion {
    /// FIX 4.2.
    #[serde(rename = "FIX.4.2")]
    Fix42,
    /// FIX 4.4.
    #[default]
    #[serde(rename = "FIX.4.4")]
    Fix44,
}

impl FixVersion {
    /// Get the BeginString value for this version.
    #[must_use]
    pub fn begin_string(&self) -> &'static str {
        match self {
            Self::Fix42 => "FIX.4.2",
            Self::Fix44 => "FIX.4.4",
        }
    }
}

impl std::fmt::Display for FixVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.begin_string())
    }
}

/// FIX session configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixConfig {
    /// FIX protocol version.
    pub version: FixVersion,
    /// Sender CompID.
    pub sender_comp_id: String,
    /// Target CompID.
    pub target_comp_id: String,
    /// FIX server host.
    pub host: String,
    /// FIX server port.
    pub port: u16,
    /// Heartbeat interval in seconds.
    pub heartbeat_interval_secs: u32,
    /// Enable automatic reconnection.
    pub reconnect_enabled: bool,
    /// Maximum reconnection attempts.
    pub reconnect_max_attempts: u32,
    /// Reconnection delay in milliseconds.
    pub reconnect_delay_ms: u64,
    /// Enable message logging.
    pub message_logging: bool,
    /// Reset sequence numbers on logon.
    pub reset_on_logon: bool,
}

impl Default for FixConfig {
    fn default() -> Self {
        Self {
            version: FixVersion::Fix44,
            sender_comp_id: String::new(),
            target_comp_id: "ALPACA".to_string(),
            host: "fix.alpaca.markets".to_string(),
            port: 5001,
            heartbeat_interval_secs: 30,
            reconnect_enabled: true,
            reconnect_max_attempts: 5,
            reconnect_delay_ms: 1000,
            message_logging: false,
            reset_on_logon: false,
        }
    }
}

impl FixConfig {
    /// Create a new configuration builder.
    #[must_use]
    pub fn builder() -> FixConfigBuilder {
        FixConfigBuilder::default()
    }
}

/// Builder for FIX configuration.
#[derive(Debug, Default)]
pub struct FixConfigBuilder {
    config: FixConfig,
}

impl FixConfigBuilder {
    /// Set FIX version.
    #[must_use]
    pub fn version(mut self, version: FixVersion) -> Self {
        self.config.version = version;
        self
    }

    /// Set sender CompID.
    #[must_use]
    pub fn sender_comp_id(mut self, id: &str) -> Self {
        self.config.sender_comp_id = id.to_string();
        self
    }

    /// Set target CompID.
    #[must_use]
    pub fn target_comp_id(mut self, id: &str) -> Self {
        self.config.target_comp_id = id.to_string();
        self
    }

    /// Set server host.
    #[must_use]
    pub fn host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }

    /// Set server port.
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set heartbeat interval in seconds.
    #[must_use]
    pub fn heartbeat_interval_secs(mut self, secs: u32) -> Self {
        self.config.heartbeat_interval_secs = secs;
        self
    }

    /// Enable or disable reconnection.
    #[must_use]
    pub fn reconnect_enabled(mut self, enabled: bool) -> Self {
        self.config.reconnect_enabled = enabled;
        self
    }

    /// Set maximum reconnection attempts.
    #[must_use]
    pub fn reconnect_max_attempts(mut self, attempts: u32) -> Self {
        self.config.reconnect_max_attempts = attempts;
        self
    }

    /// Set reconnection delay in milliseconds.
    #[must_use]
    pub fn reconnect_delay_ms(mut self, ms: u64) -> Self {
        self.config.reconnect_delay_ms = ms;
        self
    }

    /// Enable or disable message logging.
    #[must_use]
    pub fn message_logging(mut self, enabled: bool) -> Self {
        self.config.message_logging = enabled;
        self
    }

    /// Enable or disable sequence reset on logon.
    #[must_use]
    pub fn reset_on_logon(mut self, reset: bool) -> Self {
        self.config.reset_on_logon = reset;
        self
    }

    /// Build the configuration.
    #[must_use]
    pub fn build(self) -> FixConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_version_begin_string() {
        assert_eq!(FixVersion::Fix42.begin_string(), "FIX.4.2");
        assert_eq!(FixVersion::Fix44.begin_string(), "FIX.4.4");
    }

    #[test]
    fn test_fix_config_builder() {
        let config = FixConfig::builder()
            .version(FixVersion::Fix42)
            .sender_comp_id("SENDER")
            .target_comp_id("TARGET")
            .host("localhost")
            .port(9000)
            .heartbeat_interval_secs(60)
            .build();

        assert_eq!(config.version, FixVersion::Fix42);
        assert_eq!(config.sender_comp_id, "SENDER");
        assert_eq!(config.target_comp_id, "TARGET");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 9000);
        assert_eq!(config.heartbeat_interval_secs, 60);
    }

    #[test]
    fn test_fix_config_default() {
        let config = FixConfig::default();
        assert_eq!(config.version, FixVersion::Fix44);
        assert_eq!(config.target_comp_id, "ALPACA");
        assert_eq!(config.port, 5001);
    }
}
