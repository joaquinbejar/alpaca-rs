//! FIX message types.

use serde::{Deserialize, Serialize};

/// FIX message type identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MsgType {
    /// Heartbeat (0).
    Heartbeat,
    /// Test Request (1).
    TestRequest,
    /// Resend Request (2).
    ResendRequest,
    /// Reject (3).
    Reject,
    /// Sequence Reset (4).
    SequenceReset,
    /// Logout (5).
    Logout,
    /// Execution Report (8).
    ExecutionReport,
    /// Order Cancel Reject (9).
    OrderCancelReject,
    /// Logon (A).
    Logon,
    /// New Order Single (D).
    NewOrderSingle,
    /// Order Cancel Request (F).
    OrderCancelRequest,
    /// Order Cancel/Replace Request (G).
    OrderCancelReplaceRequest,
    /// Market Data Request (V).
    MarketDataRequest,
    /// Market Data Snapshot (W).
    MarketDataSnapshot,
    /// Market Data Incremental Refresh (X).
    MarketDataIncrementalRefresh,
}

impl MsgType {
    /// Get the FIX tag value for this message type.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Heartbeat => "0",
            Self::TestRequest => "1",
            Self::ResendRequest => "2",
            Self::Reject => "3",
            Self::SequenceReset => "4",
            Self::Logout => "5",
            Self::ExecutionReport => "8",
            Self::OrderCancelReject => "9",
            Self::Logon => "A",
            Self::NewOrderSingle => "D",
            Self::OrderCancelRequest => "F",
            Self::OrderCancelReplaceRequest => "G",
            Self::MarketDataRequest => "V",
            Self::MarketDataSnapshot => "W",
            Self::MarketDataIncrementalRefresh => "X",
        }
    }

    /// Parse message type from string value.
    #[must_use]
    pub fn from_fix_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Heartbeat),
            "1" => Some(Self::TestRequest),
            "2" => Some(Self::ResendRequest),
            "3" => Some(Self::Reject),
            "4" => Some(Self::SequenceReset),
            "5" => Some(Self::Logout),
            "8" => Some(Self::ExecutionReport),
            "9" => Some(Self::OrderCancelReject),
            "A" => Some(Self::Logon),
            "D" => Some(Self::NewOrderSingle),
            "F" => Some(Self::OrderCancelRequest),
            "G" => Some(Self::OrderCancelReplaceRequest),
            "V" => Some(Self::MarketDataRequest),
            "W" => Some(Self::MarketDataSnapshot),
            "X" => Some(Self::MarketDataIncrementalRefresh),
            _ => None,
        }
    }
}

impl std::fmt::Display for MsgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    /// Buy order.
    Buy,
    /// Sell order.
    Sell,
    /// Sell short.
    SellShort,
}

impl Side {
    /// Get the FIX tag value.
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::Buy => '1',
            Self::Sell => '2',
            Self::SellShort => '5',
        }
    }

    /// Parse from FIX value.
    #[must_use]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::Buy),
            '2' => Some(Self::Sell),
            '5' => Some(Self::SellShort),
            _ => None,
        }
    }
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrdType {
    /// Market order.
    Market,
    /// Limit order.
    Limit,
    /// Stop order.
    Stop,
    /// Stop limit order.
    StopLimit,
}

impl OrdType {
    /// Get the FIX tag value.
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::Market => '1',
            Self::Limit => '2',
            Self::Stop => '3',
            Self::StopLimit => '4',
        }
    }

    /// Parse from FIX value.
    #[must_use]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::Market),
            '2' => Some(Self::Limit),
            '3' => Some(Self::Stop),
            '4' => Some(Self::StopLimit),
            _ => None,
        }
    }
}

/// Time in force.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Day order.
    Day,
    /// Good Till Cancel.
    Gtc,
    /// Immediate or Cancel.
    Ioc,
    /// Fill or Kill.
    Fok,
}

impl TimeInForce {
    /// Get the FIX tag value.
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::Day => '0',
            Self::Gtc => '1',
            Self::Ioc => '3',
            Self::Fok => '4',
        }
    }

    /// Parse from FIX value.
    #[must_use]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Day),
            '1' => Some(Self::Gtc),
            '3' => Some(Self::Ioc),
            '4' => Some(Self::Fok),
            _ => None,
        }
    }
}

/// Execution type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecType {
    /// New order.
    New,
    /// Partial fill.
    PartialFill,
    /// Full fill.
    Fill,
    /// Canceled.
    Canceled,
    /// Replaced.
    Replaced,
    /// Pending cancel.
    PendingCancel,
    /// Rejected.
    Rejected,
    /// Pending new.
    PendingNew,
    /// Expired.
    Expired,
}

impl ExecType {
    /// Get the FIX tag value.
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::New => '0',
            Self::PartialFill => '1',
            Self::Fill => '2',
            Self::Canceled => '4',
            Self::Replaced => '5',
            Self::PendingCancel => '6',
            Self::Rejected => '8',
            Self::PendingNew => 'A',
            Self::Expired => 'C',
        }
    }

    /// Parse from FIX value.
    #[must_use]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::New),
            '1' => Some(Self::PartialFill),
            '2' => Some(Self::Fill),
            '4' => Some(Self::Canceled),
            '5' => Some(Self::Replaced),
            '6' => Some(Self::PendingCancel),
            '8' => Some(Self::Rejected),
            'A' => Some(Self::PendingNew),
            'C' => Some(Self::Expired),
            _ => None,
        }
    }
}

/// Order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrdStatus {
    /// New order.
    New,
    /// Partially filled.
    PartiallyFilled,
    /// Filled.
    Filled,
    /// Canceled.
    Canceled,
    /// Replaced.
    Replaced,
    /// Pending cancel.
    PendingCancel,
    /// Rejected.
    Rejected,
    /// Pending new.
    PendingNew,
    /// Expired.
    Expired,
    /// Pending replace.
    PendingReplace,
}

impl OrdStatus {
    /// Get the FIX tag value.
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::New => '0',
            Self::PartiallyFilled => '1',
            Self::Filled => '2',
            Self::Canceled => '4',
            Self::Replaced => '5',
            Self::PendingCancel => '6',
            Self::Rejected => '8',
            Self::PendingNew => 'A',
            Self::Expired => 'C',
            Self::PendingReplace => 'E',
        }
    }

    /// Parse from FIX value.
    #[must_use]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::New),
            '1' => Some(Self::PartiallyFilled),
            '2' => Some(Self::Filled),
            '4' => Some(Self::Canceled),
            '5' => Some(Self::Replaced),
            '6' => Some(Self::PendingCancel),
            '8' => Some(Self::Rejected),
            'A' => Some(Self::PendingNew),
            'C' => Some(Self::Expired),
            'E' => Some(Self::PendingReplace),
            _ => None,
        }
    }
}

/// New Order Single message (MsgType D).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderSingle {
    /// Client order ID (Tag 11).
    pub cl_ord_id: String,
    /// Symbol (Tag 55).
    pub symbol: String,
    /// Side (Tag 54).
    pub side: Side,
    /// Order type (Tag 40).
    pub ord_type: OrdType,
    /// Order quantity (Tag 38).
    pub order_qty: f64,
    /// Limit price (Tag 44).
    pub price: Option<f64>,
    /// Stop price (Tag 99).
    pub stop_px: Option<f64>,
    /// Time in force (Tag 59).
    pub time_in_force: TimeInForce,
    /// Account (Tag 1).
    pub account: Option<String>,
}

impl NewOrderSingle {
    /// Create a market order.
    #[must_use]
    pub fn market(symbol: &str, side: Side, qty: f64) -> Self {
        Self {
            cl_ord_id: uuid::Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side,
            ord_type: OrdType::Market,
            order_qty: qty,
            price: None,
            stop_px: None,
            time_in_force: TimeInForce::Day,
            account: None,
        }
    }

    /// Create a limit order.
    #[must_use]
    pub fn limit(symbol: &str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            cl_ord_id: uuid::Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side,
            ord_type: OrdType::Limit,
            order_qty: qty,
            price: Some(price),
            stop_px: None,
            time_in_force: TimeInForce::Day,
            account: None,
        }
    }

    /// Create a stop order.
    #[must_use]
    pub fn stop(symbol: &str, side: Side, qty: f64, stop_price: f64) -> Self {
        Self {
            cl_ord_id: uuid::Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side,
            ord_type: OrdType::Stop,
            order_qty: qty,
            price: None,
            stop_px: Some(stop_price),
            time_in_force: TimeInForce::Day,
            account: None,
        }
    }

    /// Set client order ID.
    #[must_use]
    pub fn with_cl_ord_id(mut self, id: &str) -> Self {
        self.cl_ord_id = id.to_string();
        self
    }

    /// Set time in force.
    #[must_use]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// Set account.
    #[must_use]
    pub fn with_account(mut self, account: &str) -> Self {
        self.account = Some(account.to_string());
        self
    }
}

/// Order Cancel Request message (MsgType F).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelRequest {
    /// Original client order ID (Tag 41).
    pub orig_cl_ord_id: String,
    /// New client order ID (Tag 11).
    pub cl_ord_id: String,
    /// Symbol (Tag 55).
    pub symbol: String,
    /// Side (Tag 54).
    pub side: Side,
}

impl OrderCancelRequest {
    /// Create a cancel request.
    #[must_use]
    pub fn new(orig_cl_ord_id: &str, symbol: &str, side: Side) -> Self {
        Self {
            orig_cl_ord_id: orig_cl_ord_id.to_string(),
            cl_ord_id: uuid::Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side,
        }
    }
}

/// Order Cancel/Replace Request message (MsgType G).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelReplaceRequest {
    /// Original client order ID (Tag 41).
    pub orig_cl_ord_id: String,
    /// New client order ID (Tag 11).
    pub cl_ord_id: String,
    /// Symbol (Tag 55).
    pub symbol: String,
    /// Side (Tag 54).
    pub side: Side,
    /// Order type (Tag 40).
    pub ord_type: OrdType,
    /// New quantity (Tag 38).
    pub order_qty: f64,
    /// New price (Tag 44).
    pub price: Option<f64>,
}

impl OrderCancelReplaceRequest {
    /// Create a replace request.
    #[must_use]
    pub fn new(
        orig_cl_ord_id: &str,
        symbol: &str,
        side: Side,
        ord_type: OrdType,
        qty: f64,
    ) -> Self {
        Self {
            orig_cl_ord_id: orig_cl_ord_id.to_string(),
            cl_ord_id: uuid::Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side,
            ord_type,
            order_qty: qty,
            price: None,
        }
    }

    /// Set new price.
    #[must_use]
    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
}

/// Execution Report message (MsgType 8).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    /// Order ID (Tag 37).
    pub order_id: String,
    /// Client order ID (Tag 11).
    pub cl_ord_id: String,
    /// Execution ID (Tag 17).
    pub exec_id: String,
    /// Execution type (Tag 150).
    pub exec_type: ExecType,
    /// Order status (Tag 39).
    pub ord_status: OrdStatus,
    /// Symbol (Tag 55).
    pub symbol: String,
    /// Side (Tag 54).
    pub side: Side,
    /// Order quantity (Tag 38).
    pub order_qty: f64,
    /// Last quantity (Tag 32).
    pub last_qty: Option<f64>,
    /// Last price (Tag 31).
    pub last_px: Option<f64>,
    /// Cumulative quantity (Tag 14).
    pub cum_qty: f64,
    /// Average price (Tag 6).
    pub avg_px: f64,
    /// Leaves quantity (Tag 151).
    pub leaves_qty: f64,
    /// Text (Tag 58).
    pub text: Option<String>,
}

/// Order Cancel Reject message (MsgType 9).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelReject {
    /// Order ID (Tag 37).
    pub order_id: String,
    /// Client order ID (Tag 11).
    pub cl_ord_id: String,
    /// Original client order ID (Tag 41).
    pub orig_cl_ord_id: String,
    /// Order status (Tag 39).
    pub ord_status: OrdStatus,
    /// Cancel reject reason (Tag 102).
    pub cxl_rej_reason: Option<String>,
    /// Text (Tag 58).
    pub text: Option<String>,
}

/// Market Data Request message (MsgType V).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataRequest {
    /// Request ID (Tag 262).
    pub md_req_id: String,
    /// Subscription type (Tag 263).
    pub subscription_request_type: char,
    /// Market depth (Tag 264).
    pub market_depth: u32,
    /// Symbols to subscribe.
    pub symbols: Vec<String>,
}

impl MarketDataRequest {
    /// Create a snapshot request.
    #[must_use]
    pub fn snapshot(symbols: Vec<String>) -> Self {
        Self {
            md_req_id: uuid::Uuid::new_v4().to_string(),
            subscription_request_type: '0',
            market_depth: 1,
            symbols,
        }
    }

    /// Create a subscription request.
    #[must_use]
    pub fn subscribe(symbols: Vec<String>) -> Self {
        Self {
            md_req_id: uuid::Uuid::new_v4().to_string(),
            subscription_request_type: '1',
            market_depth: 1,
            symbols,
        }
    }

    /// Create an unsubscribe request.
    #[must_use]
    pub fn unsubscribe(symbols: Vec<String>) -> Self {
        Self {
            md_req_id: uuid::Uuid::new_v4().to_string(),
            subscription_request_type: '2',
            market_depth: 1,
            symbols,
        }
    }
}

/// Market Data Snapshot message (MsgType W).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataSnapshot {
    /// Request ID (Tag 262).
    pub md_req_id: String,
    /// Symbol (Tag 55).
    pub symbol: String,
    /// Entries.
    pub entries: Vec<MarketDataEntry>,
}

/// Market data entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataEntry {
    /// Entry type (Tag 269).
    pub md_entry_type: char,
    /// Price (Tag 270).
    pub md_entry_px: f64,
    /// Size (Tag 271).
    pub md_entry_size: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_type_conversion() {
        assert_eq!(MsgType::NewOrderSingle.as_str(), "D");
        assert_eq!(MsgType::ExecutionReport.as_str(), "8");
        assert_eq!(MsgType::from_fix_str("D"), Some(MsgType::NewOrderSingle));
        assert_eq!(MsgType::from_fix_str("8"), Some(MsgType::ExecutionReport));
    }

    #[test]
    fn test_new_order_single_market() {
        let order = NewOrderSingle::market("AAPL", Side::Buy, 100.0);
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, Side::Buy);
        assert_eq!(order.ord_type, OrdType::Market);
        assert_eq!(order.order_qty, 100.0);
        assert!(order.price.is_none());
    }

    #[test]
    fn test_new_order_single_limit() {
        let order = NewOrderSingle::limit("AAPL", Side::Sell, 50.0, 150.00);
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, Side::Sell);
        assert_eq!(order.ord_type, OrdType::Limit);
        assert_eq!(order.order_qty, 50.0);
        assert_eq!(order.price, Some(150.00));
    }

    #[test]
    fn test_order_cancel_request() {
        let cancel = OrderCancelRequest::new("orig123", "AAPL", Side::Buy);
        assert_eq!(cancel.orig_cl_ord_id, "orig123");
        assert_eq!(cancel.symbol, "AAPL");
        assert_eq!(cancel.side, Side::Buy);
    }

    #[test]
    fn test_market_data_request() {
        let req = MarketDataRequest::subscribe(vec!["AAPL".to_string(), "TSLA".to_string()]);
        assert_eq!(req.subscription_request_type, '1');
        assert_eq!(req.symbols.len(), 2);
    }
}
