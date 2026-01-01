//! Core types for the Alpaca API.
//!
//! This module contains all the data structures used to interact with the Alpaca API.

#![allow(missing_docs)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Trading environment for Alpaca API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    /// Paper trading environment for testing.
    Paper,
    /// Live trading environment with real money.
    Live,
}

impl Environment {
    /// Returns the base URL for the trading API.
    #[must_use]
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Paper => "https://paper-api.alpaca.markets",
            Environment::Live => "https://api.alpaca.markets",
        }
    }

    /// Returns the base URL for the market data API.
    #[must_use]
    pub fn data_url(&self) -> &'static str {
        "https://data.alpaca.markets"
    }

    /// Returns the WebSocket URL for streaming data.
    #[must_use]
    pub fn websocket_url(&self) -> &'static str {
        match self {
            Environment::Paper => "wss://paper-api.alpaca.markets/stream",
            Environment::Live => "wss://api.alpaca.markets/stream",
        }
    }
}

/// Account information from Alpaca API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    /// Unique account identifier.
    pub id: Uuid,
    /// Account number.
    pub account_number: String,
    /// Current account status.
    pub status: AccountStatus,
    /// Account currency (e.g., "USD").
    pub currency: String,
    /// Current buying power in dollars.
    pub buying_power: String,
    /// Regulation T buying power.
    pub regt_buying_power: String,
    /// Day trading buying power.
    pub daytrading_buying_power: String,
    /// Cash balance in dollars.
    pub cash: String,
    /// Total portfolio value in dollars.
    pub portfolio_value: String,
    /// Whether account is flagged as pattern day trader.
    pub pattern_day_trader: bool,
    /// Whether trading is blocked.
    pub trading_blocked: bool,
    /// Whether transfers are blocked.
    pub transfers_blocked: bool,
    /// Whether account is blocked.
    pub account_blocked: bool,
    /// Account creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Whether trading is suspended by user.
    pub trade_suspended_by_user: bool,
    /// Margin multiplier.
    pub multiplier: String,
    /// Whether shorting is enabled.
    pub shorting_enabled: bool,
    /// Current equity value in dollars.
    pub equity: String,
    /// Previous day's equity value in dollars.
    pub last_equity: String,
    /// Long positions market value in dollars.
    pub long_market_value: String,
    /// Short positions market value in dollars.
    pub short_market_value: String,
    /// Initial margin requirement in dollars.
    pub initial_margin: String,
    /// Maintenance margin requirement in dollars.
    pub maintenance_margin: String,
    /// Previous day's maintenance margin in dollars.
    pub last_maintenance_margin: String,
    /// Special memorandum account value.
    pub sma: String,
    /// Number of day trades in the last 5 trading days.
    pub daytrade_count: i32,
}

/// Account status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    /// Account is in onboarding process.
    Onboarding,
    /// Account submission failed.
    SubmissionFailed,
    /// Account has been submitted for review.
    Submitted,
    /// Account information has been updated.
    AccountUpdated,
    /// Account is pending approval.
    ApprovalPending,
    /// Account is active and can trade.
    Active,
    /// Account application was rejected.
    Rejected,
    /// Account has been disabled.
    Disabled,
    /// Account has been closed.
    AccountClosed,
}

/// Asset information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    /// Unique asset identifier.
    pub id: Uuid,
    /// Asset class (equity or crypto).
    pub class: AssetClass,
    /// Exchange where the asset trades.
    pub exchange: String,
    /// Ticker symbol.
    pub symbol: String,
    /// Full asset name.
    pub name: String,
    /// Current asset status.
    pub status: AssetStatus,
    /// Whether the asset can be traded.
    pub tradable: bool,
    /// Whether the asset can be used as margin collateral.
    pub marginable: bool,
    /// Whether the asset can be shorted.
    pub shortable: bool,
    /// Whether the asset is easy to borrow for shorting.
    pub easy_to_borrow: bool,
    /// Whether fractional shares are supported.
    pub fractionable: bool,
}

/// Asset class.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    /// US equity.
    UsEquity,
    /// Cryptocurrency.
    Crypto,
}

/// Asset status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    /// Asset is active and tradable.
    Active,
    /// Asset is inactive.
    Inactive,
}

/// Order information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    /// Unique order identifier.
    pub id: Uuid,
    /// Client-specified order ID.
    pub client_order_id: String,
    /// Order creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
    /// Submission timestamp.
    pub submitted_at: Option<DateTime<Utc>>,
    /// Fill timestamp.
    pub filled_at: Option<DateTime<Utc>>,
    /// Expiration timestamp.
    pub expired_at: Option<DateTime<Utc>>,
    /// Cancellation timestamp.
    pub canceled_at: Option<DateTime<Utc>>,
    /// Failure timestamp.
    pub failed_at: Option<DateTime<Utc>>,
    /// Replacement timestamp.
    pub replaced_at: Option<DateTime<Utc>>,
    /// ID of the order that replaced this one.
    pub replaced_by: Option<Uuid>,
    /// ID of the order this one replaces.
    pub replaces: Option<Uuid>,
    /// Asset identifier.
    pub asset_id: Uuid,
    /// Ticker symbol.
    pub symbol: String,
    /// Asset class.
    pub asset_class: AssetClass,
    /// Notional value in dollars.
    pub notional: Option<String>,
    /// Number of shares.
    pub qty: Option<String>,
    /// Number of shares filled.
    pub filled_qty: String,
    /// Average fill price in dollars.
    pub filled_avg_price: Option<String>,
    /// Order class (simple, bracket, etc.).
    pub order_class: OrderClass,
    /// Order type (market, limit, etc.).
    pub order_type: OrderType,
    /// Buy or sell.
    pub side: OrderSide,
    /// Time in force.
    pub time_in_force: TimeInForce,
    /// Limit price in dollars.
    pub limit_price: Option<String>,
    /// Stop price in dollars.
    pub stop_price: Option<String>,
    /// Current order status.
    pub status: OrderStatus,
    /// Whether extended hours trading is enabled.
    pub extended_hours: bool,
    /// Child orders for bracket/OCO/OTO orders.
    pub legs: Option<Vec<Order>>,
    /// Trailing stop percentage.
    pub trail_percent: Option<String>,
    /// Trailing stop price offset in dollars.
    pub trail_price: Option<String>,
    /// High water mark for trailing stop.
    pub hwm: Option<String>,
}

/// Order class.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderClass {
    /// Simple order.
    Simple,
    /// Bracket order with take-profit and stop-loss.
    Bracket,
    /// One-cancels-other order.
    Oco,
    /// One-triggers-other order.
    Oto,
}

/// Order type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    /// Market order.
    #[default]
    Market,
    /// Limit order.
    Limit,
    /// Stop order.
    Stop,
    /// Stop-limit order.
    StopLimit,
    /// Trailing stop order.
    TrailingStop,
}

/// Order side.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    /// Buy order.
    #[default]
    Buy,
    /// Sell order.
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

// ============================================================================
// Options Trading Types
// ============================================================================

/// Option type (call or put).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
    /// Call option - right to buy.
    Call,
    /// Put option - right to sell.
    Put,
}

/// Option style (American or European).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OptionStyle {
    /// American style - can be exercised any time before expiration.
    American,
    /// European style - can only be exercised at expiration.
    European,
}

/// Options trading approval level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OptionsApprovalLevel {
    /// Level 1: Covered calls and cash-secured puts.
    #[serde(rename = "1")]
    Level1,
    /// Level 2: Long calls and puts, spreads.
    #[serde(rename = "2")]
    Level2,
    /// Level 3: Naked calls and puts.
    #[serde(rename = "3")]
    Level3,
    /// Options trading disabled.
    Disabled,
}

/// Options approval status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OptionsApprovalStatus {
    /// Approval is pending.
    Pending,
    /// Options trading is approved.
    Approved,
    /// Options trading request was rejected.
    Rejected,
    /// Options trading is inactive.
    Inactive,
}

/// Option contract information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionContract {
    /// Unique contract identifier.
    pub id: Uuid,
    /// OCC symbol for the contract.
    pub symbol: String,
    /// Human-readable contract name.
    pub name: String,
    /// Contract status.
    pub status: AssetStatus,
    /// Whether the contract is tradable.
    pub tradable: bool,
    /// Expiration date (YYYY-MM-DD).
    pub expiration_date: String,
    /// Strike price in dollars.
    pub strike_price: String,
    /// Option type (call or put).
    #[serde(rename = "type")]
    pub option_type: OptionType,
    /// Option style (American or European).
    pub style: OptionStyle,
    /// Underlying asset symbol.
    pub underlying_symbol: String,
    /// Underlying asset ID.
    pub underlying_asset_id: Uuid,
    /// Root symbol for the option chain.
    pub root_symbol: String,
    /// Open interest (number of open contracts).
    #[serde(default)]
    pub open_interest: Option<String>,
    /// Date when open interest was last updated.
    #[serde(default)]
    pub open_interest_date: Option<String>,
    /// Contract size (typically 100 shares).
    #[serde(default)]
    pub size: Option<String>,
    /// Close price from previous trading day.
    #[serde(default)]
    pub close_price: Option<String>,
    /// Date of close price.
    #[serde(default)]
    pub close_price_date: Option<String>,
}

/// Option Greeks for pricing and risk analysis.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionGreeks {
    /// Delta - rate of change of option price with respect to underlying price.
    pub delta: Option<f64>,
    /// Gamma - rate of change of delta with respect to underlying price.
    pub gamma: Option<f64>,
    /// Theta - rate of change of option price with respect to time (time decay).
    pub theta: Option<f64>,
    /// Vega - rate of change of option price with respect to volatility.
    pub vega: Option<f64>,
    /// Rho - rate of change of option price with respect to interest rate.
    pub rho: Option<f64>,
}

/// Option quote data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionQuote {
    /// Quote timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid price.
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size.
    #[serde(rename = "bs")]
    pub bid_size: u64,
    /// Ask price.
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size.
    #[serde(rename = "as")]
    pub ask_size: u64,
    /// Bid exchange.
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    /// Ask exchange.
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    /// Condition flags.
    #[serde(rename = "c", default)]
    pub conditions: Option<String>,
}

/// Option trade data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionTrade {
    /// Trade timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Trade size (number of contracts).
    #[serde(rename = "s")]
    pub size: u64,
    /// Exchange where trade occurred.
    #[serde(rename = "x")]
    pub exchange: String,
    /// Trade conditions.
    #[serde(rename = "c", default)]
    pub conditions: Option<String>,
}

/// Option bar (OHLCV) data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionBar {
    /// Bar timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Open price.
    #[serde(rename = "o")]
    pub open: f64,
    /// High price.
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price.
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price.
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume (number of contracts traded).
    #[serde(rename = "v")]
    pub volume: u64,
    /// Number of trades.
    #[serde(rename = "n", default)]
    pub trade_count: Option<u64>,
    /// Volume-weighted average price.
    #[serde(rename = "vw", default)]
    pub vwap: Option<f64>,
}

/// Option snapshot with latest quote, trade, and greeks.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionSnapshot {
    /// Latest quote.
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<OptionQuote>,
    /// Latest trade.
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<OptionTrade>,
    /// Option Greeks.
    pub greeks: Option<OptionGreeks>,
    /// Implied volatility.
    #[serde(rename = "impliedVolatility")]
    pub implied_volatility: Option<f64>,
}

/// Options chain entry for a specific strike/expiration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionChainEntry {
    /// Option contract.
    pub contract: OptionContract,
    /// Snapshot data.
    pub snapshot: Option<OptionSnapshot>,
}

/// Request to exercise an option.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionExerciseRequest {
    /// Symbol of the option contract to exercise.
    pub symbol: String,
    /// Number of contracts to exercise.
    #[serde(default)]
    pub qty: Option<String>,
}

/// Options approval request for an account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionsApprovalRequest {
    /// Requested options trading level.
    pub options_trading_level: OptionsApprovalLevel,
}

/// Options approval status response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionsApproval {
    /// Current options trading level.
    pub options_trading_level: Option<OptionsApprovalLevel>,
    /// Approval status.
    pub status: OptionsApprovalStatus,
    /// Reason for rejection (if applicable).
    #[serde(default)]
    pub reason: Option<String>,
}

/// Parameters for querying option contracts.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OptionContractParams {
    /// Filter by underlying symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,
    /// Filter by expiration date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Filter by expiration date greater than or equal to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_gte: Option<String>,
    /// Filter by expiration date less than or equal to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_lte: Option<String>,
    /// Filter by strike price greater than or equal to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price_gte: Option<String>,
    /// Filter by strike price less than or equal to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price_lte: Option<String>,
    /// Filter by option type (call or put).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub option_type: Option<OptionType>,
    /// Filter by root symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_symbol: Option<String>,
    /// Filter by style (american or european).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<OptionStyle>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl OptionContractParams {
    /// Create new empty parameters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by underlying symbol.
    #[must_use]
    pub fn underlying_symbol(mut self, symbol: &str) -> Self {
        self.underlying_symbol = Some(symbol.to_string());
        self
    }

    /// Filter by expiration date.
    #[must_use]
    pub fn expiration_date(mut self, date: &str) -> Self {
        self.expiration_date = Some(date.to_string());
        self
    }

    /// Filter by option type.
    #[must_use]
    pub fn option_type(mut self, option_type: OptionType) -> Self {
        self.option_type = Some(option_type);
        self
    }

    /// Filter by strike price range.
    #[must_use]
    pub fn strike_price_range(mut self, min: &str, max: &str) -> Self {
        self.strike_price_gte = Some(min.to_string());
        self.strike_price_lte = Some(max.to_string());
        self
    }

    /// Set maximum number of results.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for querying option bars.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OptionBarsParams {
    /// Option symbols to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Timeframe for bars (e.g., "1Min", "1Hour", "1Day").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// Start time (RFC3339 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time (RFC3339 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl OptionBarsParams {
    /// Create new parameters with symbols.
    #[must_use]
    pub fn new(symbols: &str) -> Self {
        Self {
            symbols: Some(symbols.to_string()),
            ..Default::default()
        }
    }

    /// Set timeframe.
    #[must_use]
    pub fn timeframe(mut self, timeframe: &str) -> Self {
        self.timeframe = Some(timeframe.to_string());
        self
    }

    /// Set time range.
    #[must_use]
    pub fn time_range(mut self, start: &str, end: &str) -> Self {
        self.start = Some(start.to_string());
        self.end = Some(end.to_string());
        self
    }

    /// Set maximum number of results.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
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

    #[test]
    fn test_option_type_serialization() {
        let ot = OptionType::Call;
        let json = serde_json::to_string(&ot).unwrap();
        assert_eq!(json, "\"call\"");

        let ot = OptionType::Put;
        let json = serde_json::to_string(&ot).unwrap();
        assert_eq!(json, "\"put\"");
    }

    #[test]
    fn test_option_style_serialization() {
        let style = OptionStyle::American;
        let json = serde_json::to_string(&style).unwrap();
        assert_eq!(json, "\"american\"");

        let style = OptionStyle::European;
        let json = serde_json::to_string(&style).unwrap();
        assert_eq!(json, "\"european\"");
    }

    #[test]
    fn test_options_approval_level_serialization() {
        let level = OptionsApprovalLevel::Level1;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"1\"");

        let level = OptionsApprovalLevel::Level3;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"3\"");
    }

    #[test]
    fn test_option_contract_params_builder() {
        let params = OptionContractParams::new()
            .underlying_symbol("AAPL")
            .expiration_date("2024-03-15")
            .option_type(OptionType::Call)
            .limit(10);

        assert_eq!(params.underlying_symbol, Some("AAPL".to_string()));
        assert_eq!(params.expiration_date, Some("2024-03-15".to_string()));
        assert_eq!(params.option_type, Some(OptionType::Call));
        assert_eq!(params.limit, Some(10));
    }

    #[test]
    fn test_option_bars_params_builder() {
        let params = OptionBarsParams::new("AAPL240315C00150000")
            .timeframe("1Day")
            .time_range("2024-01-01", "2024-03-01")
            .limit(100);

        assert_eq!(params.symbols, Some("AAPL240315C00150000".to_string()));
        assert_eq!(params.timeframe, Some("1Day".to_string()));
        assert_eq!(params.start, Some("2024-01-01".to_string()));
        assert_eq!(params.end, Some("2024-03-01".to_string()));
        assert_eq!(params.limit, Some(100));
    }
}
