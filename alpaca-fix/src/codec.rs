//! FIX message encoding and decoding.

use crate::config::FixVersion;
use crate::error::{FixError, Result};
use std::collections::HashMap;

/// FIX field separator (SOH character).
pub const SOH: char = '\x01';

/// Common FIX tag numbers.
pub mod tags {
    /// BeginString (FIX version).
    pub const BEGIN_STRING: u32 = 8;
    /// Body length.
    pub const BODY_LENGTH: u32 = 9;
    /// Message type.
    pub const MSG_TYPE: u32 = 35;
    /// Sender CompID.
    pub const SENDER_COMP_ID: u32 = 49;
    /// Target CompID.
    pub const TARGET_COMP_ID: u32 = 56;
    /// Message sequence number.
    pub const MSG_SEQ_NUM: u32 = 34;
    /// Sending time.
    pub const SENDING_TIME: u32 = 52;
    /// Checksum.
    pub const CHECKSUM: u32 = 10;
    /// Client order ID.
    pub const CL_ORD_ID: u32 = 11;
    /// Order ID.
    pub const ORDER_ID: u32 = 37;
    /// Original client order ID.
    pub const ORIG_CL_ORD_ID: u32 = 41;
    /// Execution ID.
    pub const EXEC_ID: u32 = 17;
    /// Execution type.
    pub const EXEC_TYPE: u32 = 150;
    /// Order status.
    pub const ORD_STATUS: u32 = 39;
    /// Symbol.
    pub const SYMBOL: u32 = 55;
    /// Side.
    pub const SIDE: u32 = 54;
    /// Order type.
    pub const ORD_TYPE: u32 = 40;
    /// Order quantity.
    pub const ORDER_QTY: u32 = 38;
    /// Price.
    pub const PRICE: u32 = 44;
    /// Stop price.
    pub const STOP_PX: u32 = 99;
    /// Time in force.
    pub const TIME_IN_FORCE: u32 = 59;
    /// Last quantity.
    pub const LAST_QTY: u32 = 32;
    /// Last price.
    pub const LAST_PX: u32 = 31;
    /// Cumulative quantity.
    pub const CUM_QTY: u32 = 14;
    /// Average price.
    pub const AVG_PX: u32 = 6;
    /// Leaves quantity.
    pub const LEAVES_QTY: u32 = 151;
    /// Text.
    pub const TEXT: u32 = 58;
    /// Account.
    pub const ACCOUNT: u32 = 1;
    /// Heartbeat interval.
    pub const HEART_BT_INT: u32 = 108;
    /// Encrypt method.
    pub const ENCRYPT_METHOD: u32 = 98;
    /// Reset sequence number flag.
    pub const RESET_SEQ_NUM_FLAG: u32 = 141;
    /// Test request ID.
    pub const TEST_REQ_ID: u32 = 112;
    /// Begin sequence number.
    pub const BEGIN_SEQ_NO: u32 = 7;
    /// End sequence number.
    pub const END_SEQ_NO: u32 = 16;
    /// Market data request ID.
    pub const MD_REQ_ID: u32 = 262;
    /// Subscription request type.
    pub const SUBSCRIPTION_REQUEST_TYPE: u32 = 263;
    /// Market depth.
    pub const MARKET_DEPTH: u32 = 264;
    /// MD entry type.
    pub const MD_ENTRY_TYPE: u32 = 269;
    /// MD entry price.
    pub const MD_ENTRY_PX: u32 = 270;
    /// MD entry size.
    pub const MD_ENTRY_SIZE: u32 = 271;
}

/// Raw FIX message representation.
#[derive(Debug, Clone)]
pub struct FixMessage {
    /// Message fields as tag-value pairs.
    pub fields: HashMap<u32, String>,
    /// Original raw message.
    pub raw: String,
}

impl FixMessage {
    /// Create a new empty message.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            raw: String::new(),
        }
    }

    /// Get a field value by tag.
    #[must_use]
    pub fn get(&self, tag: u32) -> Option<&str> {
        self.fields.get(&tag).map(String::as_str)
    }

    /// Get message type.
    #[must_use]
    pub fn msg_type(&self) -> Option<&str> {
        self.get(tags::MSG_TYPE)
    }

    /// Set a field value.
    pub fn set(&mut self, tag: u32, value: impl Into<String>) {
        self.fields.insert(tag, value.into());
    }

    /// Check if field exists.
    #[must_use]
    pub fn has(&self, tag: u32) -> bool {
        self.fields.contains_key(&tag)
    }
}

impl Default for FixMessage {
    fn default() -> Self {
        Self::new()
    }
}

/// FIX message encoder.
#[derive(Debug)]
pub struct FixEncoder {
    version: FixVersion,
    sender_comp_id: String,
    target_comp_id: String,
}

impl FixEncoder {
    /// Create a new encoder.
    #[must_use]
    pub fn new(version: FixVersion, sender_comp_id: &str, target_comp_id: &str) -> Self {
        Self {
            version,
            sender_comp_id: sender_comp_id.to_string(),
            target_comp_id: target_comp_id.to_string(),
        }
    }

    /// Encode a message to FIX format.
    pub fn encode(&self, msg_type: &str, seq_num: u64, fields: &[(u32, String)]) -> String {
        let sending_time = chrono::Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string();

        // Build body (everything except BeginString, BodyLength, and CheckSum)
        let mut body = String::new();
        body.push_str(&format!("{}={}{}", tags::MSG_TYPE, msg_type, SOH));
        body.push_str(&format!(
            "{}={}{}",
            tags::SENDER_COMP_ID,
            self.sender_comp_id,
            SOH
        ));
        body.push_str(&format!(
            "{}={}{}",
            tags::TARGET_COMP_ID,
            self.target_comp_id,
            SOH
        ));
        body.push_str(&format!("{}={}{}", tags::MSG_SEQ_NUM, seq_num, SOH));
        body.push_str(&format!("{}={}{}", tags::SENDING_TIME, sending_time, SOH));

        for (tag, value) in fields {
            body.push_str(&format!("{}={}{}", tag, value, SOH));
        }

        // Build header
        let header = format!(
            "{}={}{}{}={}{}",
            tags::BEGIN_STRING,
            self.version.begin_string(),
            SOH,
            tags::BODY_LENGTH,
            body.len(),
            SOH
        );

        // Calculate checksum
        let checksum = Self::calculate_checksum(&format!("{}{}", header, body));

        format!(
            "{}{}{}={:03}{}",
            header,
            body,
            tags::CHECKSUM,
            checksum,
            SOH
        )
    }

    /// Calculate FIX checksum.
    fn calculate_checksum(data: &str) -> u8 {
        data.bytes().fold(0u32, |acc, b| acc + b as u32) as u8
    }
}

/// FIX message decoder.
#[derive(Debug, Default)]
pub struct FixDecoder;

impl FixDecoder {
    /// Create a new decoder.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Decode a FIX message.
    pub fn decode(&self, data: &str) -> Result<FixMessage> {
        let mut msg = FixMessage::new();
        msg.raw = data.to_string();

        for field in data.split(SOH) {
            if field.is_empty() {
                continue;
            }

            let parts: Vec<&str> = field.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(FixError::Decoding(format!("invalid field: {}", field)));
            }

            let tag: u32 = parts[0]
                .parse()
                .map_err(|_| FixError::Decoding(format!("invalid tag: {}", parts[0])))?;
            msg.fields.insert(tag, parts[1].to_string());
        }

        // Validate required fields
        if !msg.has(tags::BEGIN_STRING) {
            return Err(FixError::InvalidMessage("missing BeginString".to_string()));
        }
        if !msg.has(tags::MSG_TYPE) {
            return Err(FixError::InvalidMessage("missing MsgType".to_string()));
        }

        Ok(msg)
    }

    /// Validate message checksum.
    #[must_use]
    pub fn validate_checksum(&self, data: &str) -> bool {
        // Find checksum field position
        if let Some(pos) = data.rfind(&format!("{}=", tags::CHECKSUM)) {
            let body = &data[..pos];
            let checksum_str = &data[pos + 3..data.len() - 1]; // Skip "10=" and trailing SOH

            if let Ok(expected) = checksum_str.parse::<u8>() {
                let calculated = body.bytes().fold(0u32, |acc, b| acc + b as u32) as u8;
                return calculated == expected;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_message_fields() {
        let mut msg = FixMessage::new();
        msg.set(tags::SYMBOL, "AAPL");
        msg.set(tags::SIDE, "1");

        assert_eq!(msg.get(tags::SYMBOL), Some("AAPL"));
        assert_eq!(msg.get(tags::SIDE), Some("1"));
        assert!(msg.has(tags::SYMBOL));
        assert!(!msg.has(tags::PRICE));
    }

    #[test]
    fn test_fix_encoder() {
        let encoder = FixEncoder::new(FixVersion::Fix44, "SENDER", "TARGET");
        let fields = vec![
            (tags::SYMBOL, "AAPL".to_string()),
            (tags::SIDE, "1".to_string()),
        ];
        let encoded = encoder.encode("D", 1, &fields);

        assert!(encoded.contains("8=FIX.4.4"));
        assert!(encoded.contains("35=D"));
        assert!(encoded.contains("49=SENDER"));
        assert!(encoded.contains("56=TARGET"));
        assert!(encoded.contains("55=AAPL"));
    }

    #[test]
    fn test_fix_decoder() {
        let decoder = FixDecoder::new();
        let raw = "8=FIX.4.4\x0135=D\x0149=SENDER\x0156=TARGET\x0155=AAPL\x0110=000\x01";
        let msg = decoder.decode(raw).unwrap();

        assert_eq!(msg.get(tags::BEGIN_STRING), Some("FIX.4.4"));
        assert_eq!(msg.msg_type(), Some("D"));
        assert_eq!(msg.get(tags::SYMBOL), Some("AAPL"));
    }
}
