use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Environment for Alpaca API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Paper,
    Live,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Paper => "https://paper-api.alpaca.markets",
            Environment::Live => "https://api.alpaca.markets",
        }
    }

    pub fn data_url(&self) -> &'static str {
        "https://data.alpaca.markets"
    }

    pub fn websocket_url(&self) -> &'static str {
        match self {
            Environment::Paper => "wss://paper-api.alpaca.markets/stream",
            Environment::Live => "wss://api.alpaca.markets/stream",
        }
    }
}

/// Account information from Alpaca API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub account_number: String,
    pub status: AccountStatus,
    pub currency: String,
    pub buying_power: String,
    pub regt_buying_power: String,
    pub daytrading_buying_power: String,
    pub cash: String,
    pub portfolio_value: String,
    pub pattern_day_trader: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub created_at: DateTime<Utc>,
    pub trade_suspended_by_user: bool,
    pub multiplier: String,
    pub shorting_enabled: bool,
    pub equity: String,
    pub last_equity: String,
    pub long_market_value: String,
    pub short_market_value: String,
    pub initial_margin: String,
    pub maintenance_margin: String,
    pub last_maintenance_margin: String,
    pub sma: String,
    pub daytrade_count: i32,
}

/// Account status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    Onboarding,
    SubmissionFailed,
    Submitted,
    AccountUpdated,
    ApprovalPending,
    Active,
    Rejected,
    Disabled,
    AccountClosed,
}

/// Asset information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub class: AssetClass,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub status: AssetStatus,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
}

/// Asset class
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    UsEquity,
    Crypto,
}

/// Asset status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    Active,
    Inactive,
}

/// Order information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub client_order_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<Uuid>,
    pub replaces: Option<Uuid>,
    pub asset_id: Uuid,
    pub symbol: String,
    pub asset_class: AssetClass,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub filled_qty: String,
    pub filled_avg_price: Option<String>,
    pub order_class: OrderClass,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub time_in_force: TimeInForce,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: OrderStatus,
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}

/// Order class
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderClass {
    Simple,
    Bracket,
    Oco,
    Oto,
}

/// Order type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    #[default]
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}

/// Order side
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}

/// Time in force for orders.
///
/// Specifies how long an order remains active before it is executed or expires.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    /// Day order - valid for the trading day.
    #[default]
    Day,
    /// Good Till Canceled - remains active until filled or canceled.
    Gtc,
    /// Immediate Or Cancel - executes immediately, cancels unfilled portion.
    Ioc,
    /// Fill Or Kill - must be filled entirely immediately or canceled.
    Fok,
    /// Market On Open - executes at market open.
    Opg,
    /// Market On Close - executes at market close.
    Cls,
    /// Good Till Date - remains active until specified date.
    Gtd,
}

/// Order status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Canceled,
    Expired,
    Replaced,
    PendingCancel,
    PendingReplace,
    PendingReview,
    Accepted,
    PendingNew,
    AcceptedForBidding,
    Stopped,
    Rejected,
    Suspended,
    Calculated,
}

/// Position information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: AssetClass,
    pub avg_entry_price: String,
    pub qty: String,
    pub side: PositionSide,
    pub market_value: String,
    pub cost_basis: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
    pub unrealized_intraday_pl: String,
    pub unrealized_intraday_plpc: String,
    pub current_price: String,
    pub lastday_price: String,
    pub change_today: String,
}

/// Position side
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PositionSide {
    Long,
    Short,
}

/// Market data bar
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bar {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub trade_count: Option<u64>,
    pub vwap: Option<f64>,
}

/// Market data quote
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub timestamp: DateTime<Utc>,
    pub timeframe: String,
    pub bid_price: f64,
    pub bid_size: u32,
    pub ask_price: f64,
    pub ask_size: u32,
    pub bid_exchange: String,
    pub ask_exchange: String,
}

/// Market data trade
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub price: f64,
    pub size: u32,
    pub exchange: String,
    pub conditions: Vec<String>,
    pub id: u64,
}

/// Timeframe for market data
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Timeframe {
    #[serde(rename = "1Min")]
    OneMinute,
    #[serde(rename = "5Min")]
    FiveMinutes,
    #[serde(rename = "15Min")]
    FifteenMinutes,
    #[serde(rename = "30Min")]
    ThirtyMinutes,
    #[serde(rename = "1Hour")]
    OneHour,
    #[serde(rename = "1Day")]
    OneDay,
    #[serde(rename = "1Week")]
    OneWeek,
    #[serde(rename = "1Month")]
    OneMonth,
}

/// Watchlist information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Watchlist {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub assets: Vec<Asset>,
}

/// Calendar information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    pub date: String,
    pub open: String,
    pub close: String,
    pub session_open: String,
    pub session_close: String,
}

/// Clock information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clock {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}

/// Portfolio history
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<Option<f64>>,
    pub profit_loss: Vec<Option<f64>>,
    pub profit_loss_pct: Vec<Option<f64>>,
    pub base_value: f64,
    pub timeframe: String,
}

/// Account activity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountActivity {
    pub id: String,
    pub account_id: Uuid,
    pub activity_type: ActivityType,
    pub date: String,
    pub net_amount: String,
    pub symbol: Option<String>,
    pub qty: Option<String>,
    pub per_share_amount: Option<String>,
}

/// Activity type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    Fill,
    TransactionFee,
    Misc,
    AcatsIn,
    AcatsOut,
    Csd,
    Csr,
    Div,
    Divcgl,
    Divcgs,
    Divfee,
    Divft,
    Divnra,
    Divroc,
    Divtw,
    Divtxex,
    Int,
    Jnlc,
    Jnls,
    Ma,
    Nc,
    Opasn,
    Opexp,
    Opxrc,
    Pta,
    Ptc,
    Reorg,
    Sc,
    Sso,
    Tc,
}

/// News article
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub id: i64,
    pub headline: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub summary: String,
    pub content: String,
    pub url: String,
    pub symbols: Vec<String>,
}

/// Crypto wallet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoWallet {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
    pub balance: String,
    pub available_balance: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Position intent for options orders.
///
/// Specifies the intent of an options order in relation to opening or closing positions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PositionIntent {
    /// Buy to open a new long position.
    BuyToOpen,
    /// Buy to close an existing short position.
    BuyToClose,
    /// Sell to open a new short position.
    SellToOpen,
    /// Sell to close an existing long position.
    SellToClose,
}

/// Take profit configuration for bracket orders.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TakeProfit {
    /// The limit price for the take profit leg.
    pub limit_price: String,
}

impl TakeProfit {
    /// Creates a new take profit configuration.
    #[must_use]
    pub fn new(limit_price: impl Into<String>) -> Self {
        Self {
            limit_price: limit_price.into(),
        }
    }
}

/// Stop loss configuration for bracket orders.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StopLoss {
    /// The stop price that triggers the stop loss.
    pub stop_price: String,
    /// Optional limit price for a stop-limit order.
    pub limit_price: Option<String>,
}

impl StopLoss {
    /// Creates a new stop loss configuration with only a stop price (market order when triggered).
    #[must_use]
    pub fn new(stop_price: impl Into<String>) -> Self {
        Self {
            stop_price: stop_price.into(),
            limit_price: None,
        }
    }

    /// Creates a new stop loss configuration with both stop and limit prices (stop-limit order).
    #[must_use]
    pub fn with_limit(stop_price: impl Into<String>, limit_price: impl Into<String>) -> Self {
        Self {
            stop_price: stop_price.into(),
            limit_price: Some(limit_price.into()),
        }
    }
}

/// Sort direction for order queries.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// Ascending order (oldest first).
    Asc,
    /// Descending order (newest first).
    Desc,
}

/// Order query status filter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderQueryStatus {
    /// Only open orders.
    Open,
    /// Only closed orders.
    Closed,
    /// All orders.
    All,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_profit_new() {
        let tp = TakeProfit::new("150.00");
        assert_eq!(tp.limit_price, "150.00");
    }

    #[test]
    fn test_stop_loss_new() {
        let sl = StopLoss::new("95.00");
        assert_eq!(sl.stop_price, "95.00");
        assert!(sl.limit_price.is_none());
    }

    #[test]
    fn test_stop_loss_with_limit() {
        let sl = StopLoss::with_limit("95.00", "94.50");
        assert_eq!(sl.stop_price, "95.00");
        assert_eq!(sl.limit_price, Some("94.50".to_string()));
    }

    #[test]
    fn test_time_in_force_serialization() {
        let tif = TimeInForce::Gtc;
        let json = serde_json::to_string(&tif).unwrap();
        assert_eq!(json, "\"gtc\"");

        let tif = TimeInForce::Gtd;
        let json = serde_json::to_string(&tif).unwrap();
        assert_eq!(json, "\"gtd\"");
    }

    #[test]
    fn test_time_in_force_deserialization() {
        let tif: TimeInForce = serde_json::from_str("\"day\"").unwrap();
        assert_eq!(tif, TimeInForce::Day);

        let tif: TimeInForce = serde_json::from_str("\"gtc\"").unwrap();
        assert_eq!(tif, TimeInForce::Gtc);

        let tif: TimeInForce = serde_json::from_str("\"ioc\"").unwrap();
        assert_eq!(tif, TimeInForce::Ioc);
    }

    #[test]
    fn test_order_class_serialization() {
        let oc = OrderClass::Bracket;
        let json = serde_json::to_string(&oc).unwrap();
        assert_eq!(json, "\"bracket\"");

        let oc = OrderClass::Oco;
        let json = serde_json::to_string(&oc).unwrap();
        assert_eq!(json, "\"oco\"");

        let oc = OrderClass::Oto;
        let json = serde_json::to_string(&oc).unwrap();
        assert_eq!(json, "\"oto\"");
    }

    #[test]
    fn test_position_intent_serialization() {
        let pi = PositionIntent::BuyToOpen;
        let json = serde_json::to_string(&pi).unwrap();
        assert_eq!(json, "\"buy_to_open\"");

        let pi = PositionIntent::SellToClose;
        let json = serde_json::to_string(&pi).unwrap();
        assert_eq!(json, "\"sell_to_close\"");
    }

    #[test]
    fn test_order_query_status_serialization() {
        let status = OrderQueryStatus::Open;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"open\"");

        let status = OrderQueryStatus::All;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"all\"");
    }

    #[test]
    fn test_sort_direction_serialization() {
        let dir = SortDirection::Asc;
        let json = serde_json::to_string(&dir).unwrap();
        assert_eq!(json, "\"asc\"");

        let dir = SortDirection::Desc;
        let json = serde_json::to_string(&dir).unwrap();
        assert_eq!(json, "\"desc\"");
    }

    #[test]
    fn test_order_side_serialization() {
        let side = OrderSide::Buy;
        let json = serde_json::to_string(&side).unwrap();
        assert_eq!(json, "\"buy\"");

        let side = OrderSide::Sell;
        let json = serde_json::to_string(&side).unwrap();
        assert_eq!(json, "\"sell\"");
    }

    #[test]
    fn test_order_type_serialization() {
        let ot = OrderType::Market;
        let json = serde_json::to_string(&ot).unwrap();
        assert_eq!(json, "\"market\"");

        let ot = OrderType::StopLimit;
        let json = serde_json::to_string(&ot).unwrap();
        assert_eq!(json, "\"stop_limit\"");

        let ot = OrderType::TrailingStop;
        let json = serde_json::to_string(&ot).unwrap();
        assert_eq!(json, "\"trailing_stop\"");
    }
}
