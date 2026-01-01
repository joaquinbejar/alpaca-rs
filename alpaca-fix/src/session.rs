//! FIX session management.

use crate::codec::{FixEncoder, FixMessage, tags};
use crate::config::FixConfig;
use crate::error::{FixError, Result};
use crate::messages::MsgType;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// FIX session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Disconnected.
    Disconnected,
    /// Connecting.
    Connecting,
    /// Logging on.
    LoggingOn,
    /// Active session.
    Active,
    /// Logging out.
    LoggingOut,
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Connecting => write!(f, "Connecting"),
            Self::LoggingOn => write!(f, "LoggingOn"),
            Self::Active => write!(f, "Active"),
            Self::LoggingOut => write!(f, "LoggingOut"),
        }
    }
}

/// Sequence number manager.
#[derive(Debug)]
pub struct SequenceNumbers {
    /// Outgoing sequence number.
    outgoing: AtomicU64,
    /// Expected incoming sequence number.
    incoming: AtomicU64,
}

impl SequenceNumbers {
    /// Create new sequence numbers.
    #[must_use]
    pub fn new() -> Self {
        Self {
            outgoing: AtomicU64::new(1),
            incoming: AtomicU64::new(1),
        }
    }

    /// Get and increment outgoing sequence number.
    pub fn next_outgoing(&self) -> u64 {
        self.outgoing.fetch_add(1, Ordering::SeqCst)
    }

    /// Get current outgoing sequence number.
    #[must_use]
    pub fn current_outgoing(&self) -> u64 {
        self.outgoing.load(Ordering::SeqCst)
    }

    /// Get expected incoming sequence number.
    #[must_use]
    pub fn expected_incoming(&self) -> u64 {
        self.incoming.load(Ordering::SeqCst)
    }

    /// Increment incoming sequence number.
    pub fn increment_incoming(&self) {
        self.incoming.fetch_add(1, Ordering::SeqCst);
    }

    /// Set incoming sequence number.
    pub fn set_incoming(&self, seq: u64) {
        self.incoming.store(seq, Ordering::SeqCst);
    }

    /// Reset sequence numbers.
    pub fn reset(&self) {
        self.outgoing.store(1, Ordering::SeqCst);
        self.incoming.store(1, Ordering::SeqCst);
    }
}

impl Default for SequenceNumbers {
    fn default() -> Self {
        Self::new()
    }
}

/// FIX session.
#[derive(Debug)]
pub struct FixSession {
    /// Session configuration.
    config: FixConfig,
    /// Session state.
    state: SessionState,
    /// Sequence numbers.
    seq_nums: Arc<SequenceNumbers>,
    /// Message encoder.
    encoder: FixEncoder,
}

impl FixSession {
    /// Create a new session.
    #[must_use]
    pub fn new(config: FixConfig) -> Self {
        let encoder = FixEncoder::new(
            config.version,
            &config.sender_comp_id,
            &config.target_comp_id,
        );
        Self {
            config,
            state: SessionState::Disconnected,
            seq_nums: Arc::new(SequenceNumbers::new()),
            encoder,
        }
    }

    /// Get session state.
    #[must_use]
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Set session state.
    pub fn set_state(&mut self, state: SessionState) {
        tracing::info!("Session state: {} -> {}", self.state, state);
        self.state = state;
    }

    /// Get sequence numbers.
    #[must_use]
    pub fn seq_nums(&self) -> Arc<SequenceNumbers> {
        Arc::clone(&self.seq_nums)
    }

    /// Create logon message.
    #[must_use]
    pub fn create_logon(&self) -> String {
        let mut fields = vec![
            (tags::ENCRYPT_METHOD, "0".to_string()),
            (
                tags::HEART_BT_INT,
                self.config.heartbeat_interval_secs.to_string(),
            ),
        ];

        if self.config.reset_on_logon {
            fields.push((tags::RESET_SEQ_NUM_FLAG, "Y".to_string()));
        }

        self.encoder.encode(
            MsgType::Logon.as_str(),
            self.seq_nums.next_outgoing(),
            &fields,
        )
    }

    /// Create logout message.
    #[must_use]
    pub fn create_logout(&self, text: Option<&str>) -> String {
        let fields = if let Some(t) = text {
            vec![(tags::TEXT, t.to_string())]
        } else {
            vec![]
        };

        self.encoder.encode(
            MsgType::Logout.as_str(),
            self.seq_nums.next_outgoing(),
            &fields,
        )
    }

    /// Create heartbeat message.
    #[must_use]
    pub fn create_heartbeat(&self, test_req_id: Option<&str>) -> String {
        let fields = if let Some(id) = test_req_id {
            vec![(tags::TEST_REQ_ID, id.to_string())]
        } else {
            vec![]
        };

        self.encoder.encode(
            MsgType::Heartbeat.as_str(),
            self.seq_nums.next_outgoing(),
            &fields,
        )
    }

    /// Create test request message.
    #[must_use]
    pub fn create_test_request(&self, test_req_id: &str) -> String {
        let fields = vec![(tags::TEST_REQ_ID, test_req_id.to_string())];

        self.encoder.encode(
            MsgType::TestRequest.as_str(),
            self.seq_nums.next_outgoing(),
            &fields,
        )
    }

    /// Create resend request message.
    #[must_use]
    pub fn create_resend_request(&self, begin_seq: u64, end_seq: u64) -> String {
        let fields = vec![
            (tags::BEGIN_SEQ_NO, begin_seq.to_string()),
            (tags::END_SEQ_NO, end_seq.to_string()),
        ];

        self.encoder.encode(
            MsgType::ResendRequest.as_str(),
            self.seq_nums.next_outgoing(),
            &fields,
        )
    }

    /// Validate incoming message sequence number.
    pub fn validate_sequence(&self, msg: &FixMessage) -> Result<()> {
        if let Some(seq_str) = msg.get(tags::MSG_SEQ_NUM) {
            let seq: u64 = seq_str
                .parse()
                .map_err(|_| FixError::Decoding("invalid sequence number".to_string()))?;

            let expected = self.seq_nums.expected_incoming();

            if seq < expected {
                // Duplicate or old message
                tracing::warn!("Received old message: seq={}, expected={}", seq, expected);
            } else if seq > expected {
                // Gap detected
                return Err(FixError::SequenceError {
                    expected,
                    actual: seq,
                });
            }

            self.seq_nums.increment_incoming();
        }

        Ok(())
    }

    /// Encode a message with session headers.
    pub fn encode_message(&self, msg_type: &str, fields: &[(u32, String)]) -> String {
        self.encoder
            .encode(msg_type, self.seq_nums.next_outgoing(), fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_numbers() {
        let seq = SequenceNumbers::new();
        assert_eq!(seq.next_outgoing(), 1);
        assert_eq!(seq.next_outgoing(), 2);
        assert_eq!(seq.current_outgoing(), 3);

        assert_eq!(seq.expected_incoming(), 1);
        seq.increment_incoming();
        assert_eq!(seq.expected_incoming(), 2);

        seq.reset();
        assert_eq!(seq.current_outgoing(), 1);
        assert_eq!(seq.expected_incoming(), 1);
    }

    #[test]
    fn test_session_state() {
        let config = FixConfig::builder()
            .sender_comp_id("SENDER")
            .target_comp_id("TARGET")
            .build();
        let mut session = FixSession::new(config);

        assert_eq!(session.state(), SessionState::Disconnected);
        session.set_state(SessionState::Active);
        assert_eq!(session.state(), SessionState::Active);
    }

    #[test]
    fn test_create_logon() {
        let config = FixConfig::builder()
            .sender_comp_id("SENDER")
            .target_comp_id("TARGET")
            .heartbeat_interval_secs(30)
            .build();
        let session = FixSession::new(config);
        let logon = session.create_logon();

        assert!(logon.contains("35=A"));
        assert!(logon.contains("108=30"));
    }

    #[test]
    fn test_create_heartbeat() {
        let config = FixConfig::builder()
            .sender_comp_id("SENDER")
            .target_comp_id("TARGET")
            .build();
        let session = FixSession::new(config);
        let heartbeat = session.create_heartbeat(Some("TEST123"));

        assert!(heartbeat.contains("35=0"));
        assert!(heartbeat.contains("112=TEST123"));
    }
}
