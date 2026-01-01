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

// ============================================================================
// Enhanced Stock Market Data Types
// ============================================================================

/// Data feed source.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataFeed {
    /// IEX exchange data.
    Iex,
    /// SIP (Securities Information Processor) data.
    Sip,
    /// OTC (Over-The-Counter) data.
    Otc,
}

/// Stock snapshot with latest market data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockSnapshot {
    /// Latest trade.
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<Trade>,
    /// Latest quote.
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<Quote>,
    /// Current minute bar.
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<Bar>,
    /// Current daily bar.
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<Bar>,
    /// Previous daily bar.
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<Bar>,
}

/// Corporate action type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CorporateActionType {
    /// Cash dividend.
    Dividend,
    /// Stock split.
    Split,
    /// Reverse stock split.
    ReverseSplit,
    /// Spinoff.
    Spinoff,
    /// Merger.
    Merger,
    /// Rights issue.
    Rights,
    /// Stock distribution.
    StockDividend,
    /// Redemption.
    Redemption,
    /// Name change.
    NameChange,
    /// Symbol change.
    SymbolChange,
    /// Worthless security.
    Worthless,
}

/// Corporate action announcement.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorporateAction {
    /// Unique identifier.
    pub id: String,
    /// Corporate action type.
    #[serde(rename = "ca_type")]
    pub action_type: CorporateActionType,
    /// Sub-type of the action.
    #[serde(rename = "ca_sub_type")]
    pub sub_type: Option<String>,
    /// Initiating symbol.
    pub initiating_symbol: Option<String>,
    /// Initiating original CUSIP.
    pub initiating_original_cusip: Option<String>,
    /// Target symbol.
    pub target_symbol: Option<String>,
    /// Target original CUSIP.
    pub target_original_cusip: Option<String>,
    /// Declaration date.
    pub declaration_date: Option<String>,
    /// Ex-date.
    pub ex_date: Option<String>,
    /// Record date.
    pub record_date: Option<String>,
    /// Payable date.
    pub payable_date: Option<String>,
    /// Cash amount per share.
    pub cash: Option<String>,
    /// Old rate (for splits).
    pub old_rate: Option<String>,
    /// New rate (for splits).
    pub new_rate: Option<String>,
}

/// Limit Up Limit Down (LULD) data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Luld {
    /// LULD indicator.
    #[serde(rename = "i")]
    pub indicator: String,
    /// Limit up price.
    #[serde(rename = "u")]
    pub limit_up_price: f64,
    /// Limit down price.
    #[serde(rename = "d")]
    pub limit_down_price: f64,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}

/// Trading status update.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradingStatus {
    /// Status code.
    #[serde(rename = "sc")]
    pub status_code: String,
    /// Status message.
    #[serde(rename = "sm")]
    pub status_message: String,
    /// Reason code.
    #[serde(rename = "rc")]
    pub reason_code: String,
    /// Reason message.
    #[serde(rename = "rm")]
    pub reason_message: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}

/// Auction data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auction {
    /// Auction type (open, close).
    #[serde(rename = "at")]
    pub auction_type: String,
    /// Auction price.
    #[serde(rename = "ap")]
    pub price: Option<f64>,
    /// Auction size.
    #[serde(rename = "as")]
    pub size: Option<u64>,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}

/// Parameters for multi-symbol bars request.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MultiBarsParams {
    /// Comma-separated list of symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Timeframe (e.g., "1Min", "1Hour", "1Day").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// Start time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Maximum number of bars per symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Data feed source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    /// Pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl MultiBarsParams {
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

    /// Set data feed.
    #[must_use]
    pub fn feed(mut self, feed: DataFeed) -> Self {
        self.feed = Some(feed);
        self
    }

    /// Set limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for multi-symbol quotes request.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MultiQuotesParams {
    /// Comma-separated list of symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Start time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Maximum number of quotes per symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Data feed source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    /// Pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl MultiQuotesParams {
    /// Create new parameters with symbols.
    #[must_use]
    pub fn new(symbols: &str) -> Self {
        Self {
            symbols: Some(symbols.to_string()),
            ..Default::default()
        }
    }

    /// Set time range.
    #[must_use]
    pub fn time_range(mut self, start: &str, end: &str) -> Self {
        self.start = Some(start.to_string());
        self.end = Some(end.to_string());
        self
    }

    /// Set data feed.
    #[must_use]
    pub fn feed(mut self, feed: DataFeed) -> Self {
        self.feed = Some(feed);
        self
    }

    /// Set limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for multi-symbol trades request.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MultiTradesParams {
    /// Comma-separated list of symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Start time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Maximum number of trades per symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Data feed source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    /// Pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl MultiTradesParams {
    /// Create new parameters with symbols.
    #[must_use]
    pub fn new(symbols: &str) -> Self {
        Self {
            symbols: Some(symbols.to_string()),
            ..Default::default()
        }
    }

    /// Set time range.
    #[must_use]
    pub fn time_range(mut self, start: &str, end: &str) -> Self {
        self.start = Some(start.to_string());
        self.end = Some(end.to_string());
        self
    }

    /// Set data feed.
    #[must_use]
    pub fn feed(mut self, feed: DataFeed) -> Self {
        self.feed = Some(feed);
        self
    }

    /// Set limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for corporate actions request.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CorporateActionsParams {
    /// Filter by symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Filter by action types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Start date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl CorporateActionsParams {
    /// Create new empty parameters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by symbols.
    #[must_use]
    pub fn symbols(mut self, symbols: &str) -> Self {
        self.symbols = Some(symbols.to_string());
        self
    }

    /// Filter by action types.
    #[must_use]
    pub fn types(mut self, types: &str) -> Self {
        self.types = Some(types.to_string());
        self
    }

    /// Set date range.
    #[must_use]
    pub fn date_range(mut self, start: &str, end: &str) -> Self {
        self.start = Some(start.to_string());
        self.end = Some(end.to_string());
        self
    }

    /// Set limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

// ============================================================================
// Broker API Types - Account Management
// ============================================================================

/// Broker account status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrokerAccountStatus {
    /// Account is being onboarded.
    Onboarding,
    /// Submission failed.
    SubmissionFailed,
    /// Submitted for review.
    Submitted,
    /// Account action required.
    ActionRequired,
    /// Account is active.
    Active,
    /// Account is rejected.
    Rejected,
    /// Account is approved.
    Approved,
    /// Account is disabled.
    Disabled,
    /// Account is closed.
    AccountClosed,
}

/// Agreement type for broker accounts.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgreementType {
    /// Margin agreement.
    MarginAgreement,
    /// Account agreement.
    AccountAgreement,
    /// Customer agreement.
    CustomerAgreement,
    /// Crypto agreement.
    CryptoAgreement,
    /// Options agreement.
    OptionsAgreement,
}

/// Funding source for broker accounts.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FundingSource {
    /// Employment income.
    EmploymentIncome,
    /// Investments.
    Investments,
    /// Inheritance.
    Inheritance,
    /// Business income.
    BusinessIncome,
    /// Savings.
    Savings,
    /// Family.
    Family,
}

/// Tax ID type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxIdType {
    /// USA Social Security Number.
    UsaSsn,
    /// Argentina CUIT.
    ArgArCuit,
    /// Australia Tax File Number.
    AusTfn,
    /// Australia Business Number.
    AusAbn,
    /// Brazil CPF.
    BraCpf,
    /// Canada SIN.
    CanSin,
    /// Chile RUT.
    ChlRut,
    /// Colombia NIT.
    ColNit,
    /// Germany Tax ID.
    DeuTaxId,
    /// Spain NIE.
    EspNie,
    /// France SPI.
    FraSpi,
    /// UK National Insurance Number.
    GbrNino,
    /// UK Unique Taxpayer Reference.
    GbrUtr,
    /// Hong Kong HKID.
    HkgHkid,
    /// Hungary Tax Number.
    HunTin,
    /// India PAN.
    IndPan,
    /// Israel ID.
    IsrId,
    /// Italy Fiscal Code.
    ItaCf,
    /// Japan My Number.
    JpnMyNumber,
    /// South Korea RRN.
    KorRrn,
    /// Mexico RFC.
    MexRfc,
    /// Netherlands BSN.
    NldBsn,
    /// New Zealand IRD.
    NzlIrd,
    /// Poland PESEL.
    PolPesel,
    /// Sweden Personal Number.
    SwePn,
    /// Singapore NRIC.
    SgpNric,
    /// Taiwan ID.
    TwnId,
    /// Not applicable.
    NotApplicable,
}

/// Contact information for broker account.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Contact {
    /// Email address.
    pub email_address: String,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Street address lines.
    pub street_address: Vec<String>,
    /// City.
    pub city: String,
    /// State or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postal code.
    pub postal_code: String,
    /// Country (ISO 3166-1 alpha-3).
    pub country: String,
}

impl Contact {
    /// Create new contact information.
    #[must_use]
    pub fn new(email: &str, city: &str, postal_code: &str, country: &str) -> Self {
        Self {
            email_address: email.to_string(),
            phone_number: None,
            street_address: Vec::new(),
            city: city.to_string(),
            state: None,
            postal_code: postal_code.to_string(),
            country: country.to_string(),
        }
    }

    /// Set phone number.
    #[must_use]
    pub fn phone(mut self, phone: &str) -> Self {
        self.phone_number = Some(phone.to_string());
        self
    }

    /// Add street address line.
    #[must_use]
    pub fn street(mut self, street: &str) -> Self {
        self.street_address.push(street.to_string());
        self
    }

    /// Set state.
    #[must_use]
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_string());
        self
    }
}

/// Identity information for broker account.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Identity {
    /// Given (first) name.
    pub given_name: String,
    /// Family (last) name.
    pub family_name: String,
    /// Middle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// Date of birth (YYYY-MM-DD).
    pub date_of_birth: String,
    /// Tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Tax ID type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_type: Option<TaxIdType>,
    /// Country of citizenship (ISO 3166-1 alpha-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_citizenship: Option<String>,
    /// Country of birth (ISO 3166-1 alpha-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_birth: Option<String>,
    /// Country of tax residence (ISO 3166-1 alpha-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_tax_residence: Option<String>,
    /// Funding source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_source: Option<Vec<FundingSource>>,
    /// Annual income minimum in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_income_min: Option<String>,
    /// Annual income maximum in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_income_max: Option<String>,
    /// Liquid net worth minimum in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquid_net_worth_min: Option<String>,
    /// Liquid net worth maximum in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquid_net_worth_max: Option<String>,
    /// Total net worth minimum in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_worth_min: Option<String>,
    /// Total net worth maximum in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_worth_max: Option<String>,
}

impl Identity {
    /// Create new identity information.
    #[must_use]
    pub fn new(given_name: &str, family_name: &str, date_of_birth: &str) -> Self {
        Self {
            given_name: given_name.to_string(),
            family_name: family_name.to_string(),
            date_of_birth: date_of_birth.to_string(),
            ..Default::default()
        }
    }

    /// Set tax ID.
    #[must_use]
    pub fn tax_id(mut self, tax_id: &str, tax_id_type: TaxIdType) -> Self {
        self.tax_id = Some(tax_id.to_string());
        self.tax_id_type = Some(tax_id_type);
        self
    }

    /// Set country of citizenship.
    #[must_use]
    pub fn citizenship(mut self, country: &str) -> Self {
        self.country_of_citizenship = Some(country.to_string());
        self
    }

    /// Set funding sources.
    #[must_use]
    pub fn funding_sources(mut self, sources: Vec<FundingSource>) -> Self {
        self.funding_source = Some(sources);
        self
    }
}

/// Disclosures for broker account.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Disclosures {
    /// Is the account holder a control person.
    pub is_control_person: bool,
    /// Is affiliated with exchange or FINRA.
    pub is_affiliated_exchange_or_finra: bool,
    /// Is politically exposed.
    pub is_politically_exposed: bool,
    /// Has immediate family exposed.
    pub immediate_family_exposed: bool,
    /// Employment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<String>,
    /// Employer name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_name: Option<String>,
    /// Employer address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_address: Option<String>,
    /// Employment position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_position: Option<String>,
}

impl Disclosures {
    /// Create new disclosures with all false.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set control person status.
    #[must_use]
    pub fn control_person(mut self, is_control: bool) -> Self {
        self.is_control_person = is_control;
        self
    }

    /// Set employment information.
    #[must_use]
    pub fn employment(mut self, status: &str, employer: &str, position: &str) -> Self {
        self.employment_status = Some(status.to_string());
        self.employer_name = Some(employer.to_string());
        self.employment_position = Some(position.to_string());
        self
    }
}

/// Agreement for broker account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agreement {
    /// Agreement type.
    pub agreement: AgreementType,
    /// When the agreement was signed (RFC3339).
    pub signed_at: String,
    /// IP address from which agreement was signed.
    pub ip_address: String,
    /// Revision of the agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

impl Agreement {
    /// Create new agreement.
    #[must_use]
    pub fn new(agreement_type: AgreementType, signed_at: &str, ip_address: &str) -> Self {
        Self {
            agreement: agreement_type,
            signed_at: signed_at.to_string(),
            ip_address: ip_address.to_string(),
            revision: None,
        }
    }
}

/// Trusted contact for broker account.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TrustedContact {
    /// Given name.
    pub given_name: String,
    /// Family name.
    pub family_name: String,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<Vec<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl TrustedContact {
    /// Create new trusted contact.
    #[must_use]
    pub fn new(given_name: &str, family_name: &str) -> Self {
        Self {
            given_name: given_name.to_string(),
            family_name: family_name.to_string(),
            ..Default::default()
        }
    }

    /// Set email.
    #[must_use]
    pub fn email(mut self, email: &str) -> Self {
        self.email_address = Some(email.to_string());
        self
    }

    /// Set phone.
    #[must_use]
    pub fn phone(mut self, phone: &str) -> Self {
        self.phone_number = Some(phone.to_string());
        self
    }
}

/// Document type for KYC.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    /// Identity verification.
    IdentityVerification,
    /// Address verification.
    AddressVerification,
    /// Date of birth verification.
    DateOfBirthVerification,
    /// Tax ID verification.
    TaxIdVerification,
    /// Account approval letter.
    AccountApprovalLetter,
    /// W8BEN form.
    W8ben,
}

/// Document for broker account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    /// Document type.
    pub document_type: DocumentType,
    /// Document sub-type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_sub_type: Option<String>,
    /// Content (base64 encoded).
    pub content: String,
    /// MIME type.
    pub mime_type: String,
}

/// Broker account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrokerAccount {
    /// Account ID.
    pub id: String,
    /// Account number.
    pub account_number: String,
    /// Account status.
    pub status: BrokerAccountStatus,
    /// Crypto status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_status: Option<BrokerAccountStatus>,
    /// Currency.
    pub currency: String,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
    /// Contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Identity information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    /// Disclosures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<Disclosures>,
    /// Agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreements: Option<Vec<Agreement>>,
    /// Trusted contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_contact: Option<TrustedContact>,
}

/// Request to create a broker account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBrokerAccountRequest {
    /// Contact information.
    pub contact: Contact,
    /// Identity information.
    pub identity: Identity,
    /// Disclosures.
    pub disclosures: Disclosures,
    /// Agreements.
    pub agreements: Vec<Agreement>,
    /// Documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Document>>,
    /// Trusted contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_contact: Option<TrustedContact>,
    /// Enabled assets (us_equity, crypto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_assets: Option<Vec<String>>,
}

impl CreateBrokerAccountRequest {
    /// Create new broker account request.
    #[must_use]
    pub fn new(
        contact: Contact,
        identity: Identity,
        disclosures: Disclosures,
        agreements: Vec<Agreement>,
    ) -> Self {
        Self {
            contact,
            identity,
            disclosures,
            agreements,
            documents: None,
            trusted_contact: None,
            enabled_assets: None,
        }
    }

    /// Add documents.
    #[must_use]
    pub fn documents(mut self, documents: Vec<Document>) -> Self {
        self.documents = Some(documents);
        self
    }

    /// Set trusted contact.
    #[must_use]
    pub fn trusted_contact(mut self, contact: TrustedContact) -> Self {
        self.trusted_contact = Some(contact);
        self
    }

    /// Set enabled assets.
    #[must_use]
    pub fn enabled_assets(mut self, assets: Vec<String>) -> Self {
        self.enabled_assets = Some(assets);
        self
    }
}

/// Request to update a broker account.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateBrokerAccountRequest {
    /// Contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Identity information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    /// Disclosures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<Disclosures>,
    /// Trusted contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_contact: Option<TrustedContact>,
}

/// CIP (Customer Identification Program) info.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CipInfo {
    /// Provider name.
    pub provider_name: Vec<String>,
    /// CIP ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// CIP result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// CIP status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Created at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// Updated at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Parameters for listing broker accounts.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListBrokerAccountsParams {
    /// Filter by query string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Created after timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Created before timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BrokerAccountStatus>,
    /// Sort order (asc or desc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Entities to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<String>,
}

impl ListBrokerAccountsParams {
    /// Create new empty parameters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by query.
    #[must_use]
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    /// Filter by status.
    #[must_use]
    pub fn status(mut self, status: BrokerAccountStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set sort order.
    #[must_use]
    pub fn sort_desc(mut self) -> Self {
        self.sort = Some("desc".to_string());
        self
    }
}

// ============================================================================
// Broker API Types - Funding & Transfers
// ============================================================================

/// ACH relationship status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AchRelationshipStatus {
    /// Queued for processing.
    Queued,
    /// Approved.
    Approved,
    /// Pending verification.
    Pending,
    /// Cancel requested.
    CancelRequested,
    /// Canceled.
    Canceled,
}

/// Bank account type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountType {
    /// Checking account.
    Checking,
    /// Savings account.
    Savings,
}

/// Transfer type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransferType {
    /// ACH transfer.
    Ach,
    /// Wire transfer.
    Wire,
}

/// Transfer direction.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferDirection {
    /// Incoming transfer (deposit).
    Incoming,
    /// Outgoing transfer (withdrawal).
    Outgoing,
}

/// Transfer status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferStatus {
    /// Queued for processing.
    Queued,
    /// Pending.
    Pending,
    /// Sent to clearing.
    SentToClearing,
    /// Approved.
    Approved,
    /// Complete.
    Complete,
    /// Returned.
    Returned,
    /// Canceled.
    Canceled,
}

/// Journal entry type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JournalEntryType {
    /// Cash journal.
    Jnlc,
    /// Security journal.
    Jnls,
}

/// Journal status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JournalStatus {
    /// Pending.
    Pending,
    /// Executed.
    Executed,
    /// Canceled.
    Canceled,
    /// Rejected.
    Rejected,
}

/// ACH relationship.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AchRelationship {
    /// Relationship ID.
    pub id: String,
    /// Account ID.
    pub account_id: String,
    /// Status.
    pub status: AchRelationshipStatus,
    /// Account owner name.
    pub account_owner_name: String,
    /// Bank account type.
    pub bank_account_type: BankAccountType,
    /// Bank account number (masked).
    pub bank_account_number: String,
    /// Bank routing number.
    pub bank_routing_number: String,
    /// Nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Request to create an ACH relationship.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAchRelationshipRequest {
    /// Account owner name.
    pub account_owner_name: String,
    /// Bank account type.
    pub bank_account_type: BankAccountType,
    /// Bank account number.
    pub bank_account_number: String,
    /// Bank routing number.
    pub bank_routing_number: String,
    /// Nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Processor token (for Plaid integration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_token: Option<String>,
}

impl CreateAchRelationshipRequest {
    /// Create new ACH relationship request.
    #[must_use]
    pub fn new(
        account_owner_name: &str,
        bank_account_type: BankAccountType,
        bank_account_number: &str,
        bank_routing_number: &str,
    ) -> Self {
        Self {
            account_owner_name: account_owner_name.to_string(),
            bank_account_type,
            bank_account_number: bank_account_number.to_string(),
            bank_routing_number: bank_routing_number.to_string(),
            nickname: None,
            processor_token: None,
        }
    }

    /// Set nickname.
    #[must_use]
    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_string());
        self
    }

    /// Set processor token.
    #[must_use]
    pub fn processor_token(mut self, token: &str) -> Self {
        self.processor_token = Some(token.to_string());
        self
    }
}

/// Transfer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    /// Transfer ID.
    pub id: String,
    /// Relationship ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    /// Account ID.
    pub account_id: String,
    /// Transfer type.
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    /// Status.
    pub status: TransferStatus,
    /// Amount in USD.
    pub amount: String,
    /// Direction.
    pub direction: TransferDirection,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Expires at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    /// Reason for status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Request to create a transfer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransferRequest {
    /// Transfer type.
    pub transfer_type: TransferType,
    /// Relationship ID (for ACH).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    /// Amount in USD.
    pub amount: String,
    /// Direction.
    pub direction: TransferDirection,
    /// Additional info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
}

impl CreateTransferRequest {
    /// Create new ACH transfer request.
    #[must_use]
    pub fn ach(relationship_id: &str, amount: &str, direction: TransferDirection) -> Self {
        Self {
            transfer_type: TransferType::Ach,
            relationship_id: Some(relationship_id.to_string()),
            amount: amount.to_string(),
            direction,
            additional_information: None,
        }
    }

    /// Create new wire transfer request.
    #[must_use]
    pub fn wire(amount: &str, direction: TransferDirection) -> Self {
        Self {
            transfer_type: TransferType::Wire,
            relationship_id: None,
            amount: amount.to_string(),
            direction,
            additional_information: None,
        }
    }
}

/// Wire bank details.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WireBank {
    /// Bank ID.
    pub id: String,
    /// Account ID.
    pub account_id: String,
    /// Bank name.
    pub name: String,
    /// Bank code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Bank code type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code_type: Option<String>,
    /// Country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// State/Province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    /// Account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request to create a wire bank.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateWireBankRequest {
    /// Bank name.
    pub name: String,
    /// Bank code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Bank code type (ABA, BIC, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code_type: Option<String>,
    /// Country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
}

/// Journal entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Journal {
    /// Journal ID.
    pub id: String,
    /// From account ID.
    pub from_account: String,
    /// To account ID.
    pub to_account: String,
    /// Entry type.
    pub entry_type: JournalEntryType,
    /// Status.
    pub status: JournalStatus,
    /// Net amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<String>,
    /// Symbol (for security journals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Quantity (for security journals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Settle date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_date: Option<String>,
    /// System date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_date: Option<String>,
}

/// Request to create a journal entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateJournalRequest {
    /// From account ID.
    pub from_account: String,
    /// To account ID.
    pub to_account: String,
    /// Entry type.
    pub entry_type: JournalEntryType,
    /// Amount (for cash journals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Symbol (for security journals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Quantity (for security journals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateJournalRequest {
    /// Create cash journal request.
    #[must_use]
    pub fn cash(from_account: &str, to_account: &str, amount: &str) -> Self {
        Self {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
            entry_type: JournalEntryType::Jnlc,
            amount: Some(amount.to_string()),
            symbol: None,
            qty: None,
            description: None,
        }
    }

    /// Create security journal request.
    #[must_use]
    pub fn security(from_account: &str, to_account: &str, symbol: &str, qty: &str) -> Self {
        Self {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
            entry_type: JournalEntryType::Jnls,
            amount: None,
            symbol: Some(symbol.to_string()),
            qty: Some(qty.to_string()),
            description: None,
        }
    }

    /// Set description.
    #[must_use]
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

/// Batch journal entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchJournalEntry {
    /// To account ID.
    pub to_account: String,
    /// Amount.
    pub amount: String,
}

/// Request to create batch journal entries.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBatchJournalRequest {
    /// From account ID.
    pub from_account: String,
    /// Entry type.
    pub entry_type: JournalEntryType,
    /// Entries.
    pub entries: Vec<BatchJournalEntry>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameters for listing transfers.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListTransfersParams {
    /// Filter by direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TransferDirection>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Offset for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Parameters for listing journals.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListJournalsParams {
    /// After timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Before timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JournalStatus>,
    /// Filter by entry type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<JournalEntryType>,
    /// Filter by to account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account: Option<String>,
    /// Filter by from account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_account: Option<String>,
}

// ============================================================================
// Enhanced Crypto Trading Types
// ============================================================================

/// Crypto blockchain chain.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoChain {
    /// Bitcoin.
    Btc,
    /// Ethereum.
    Eth,
    /// Solana.
    Sol,
    /// Avalanche.
    Avax,
    /// Polygon.
    Matic,
    /// Arbitrum.
    Arb,
    /// Base.
    Base,
}

/// Crypto transfer status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoTransferStatus {
    /// Pending approval.
    Pending,
    /// Approved.
    Approved,
    /// Pending send to blockchain.
    PendingSend,
    /// Sent to blockchain.
    Sent,
    /// Complete.
    Complete,
    /// Rejected.
    Rejected,
    /// Failed.
    Failed,
}

/// Crypto transfer direction.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoTransferDirection {
    /// Incoming (deposit).
    Incoming,
    /// Outgoing (withdrawal).
    Outgoing,
}

/// Crypto wallet status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoWalletStatus {
    /// Active.
    Active,
    /// Inactive.
    Inactive,
    /// Pending.
    Pending,
}

/// Broker crypto wallet for Broker API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrokerCryptoWallet {
    /// Wallet ID.
    pub id: String,
    /// Account ID.
    pub account_id: String,
    /// Asset symbol (e.g., BTC, ETH).
    pub asset: String,
    /// Wallet address.
    pub address: String,
    /// Blockchain chain.
    pub chain: CryptoChain,
    /// Wallet status.
    pub status: CryptoWalletStatus,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request to create a crypto wallet.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCryptoWalletRequest {
    /// Asset symbol (e.g., BTC, ETH).
    pub asset: String,
}

impl CreateCryptoWalletRequest {
    /// Create new wallet request.
    #[must_use]
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_string(),
        }
    }
}

/// Crypto transfer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoTransfer {
    /// Transfer ID.
    pub id: String,
    /// Wallet ID.
    pub wallet_id: String,
    /// Account ID.
    pub account_id: String,
    /// Asset symbol.
    pub asset: String,
    /// Amount.
    pub amount: String,
    /// Direction.
    pub direction: CryptoTransferDirection,
    /// Status.
    pub status: CryptoTransferStatus,
    /// Fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Transaction hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Request to create a crypto transfer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCryptoTransferRequest {
    /// Amount to transfer.
    pub amount: String,
    /// Destination address (for withdrawals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl CreateCryptoTransferRequest {
    /// Create withdrawal request.
    #[must_use]
    pub fn withdrawal(amount: &str, address: &str) -> Self {
        Self {
            amount: amount.to_string(),
            address: Some(address.to_string()),
        }
    }
}

/// Whitelisted crypto address.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoWhitelistAddress {
    /// Whitelist ID.
    pub id: String,
    /// Account ID.
    pub account_id: String,
    /// Asset symbol.
    pub asset: String,
    /// Whitelisted address.
    pub address: String,
    /// Chain.
    pub chain: CryptoChain,
    /// Label/nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
}

/// Request to add a whitelisted address.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCryptoWhitelistRequest {
    /// Asset symbol.
    pub asset: String,
    /// Address to whitelist.
    pub address: String,
    /// Label/nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl CreateCryptoWhitelistRequest {
    /// Create new whitelist request.
    #[must_use]
    pub fn new(asset: &str, address: &str) -> Self {
        Self {
            asset: asset.to_string(),
            address: address.to_string(),
            label: None,
        }
    }

    /// Set label.
    #[must_use]
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
}

/// Crypto snapshot with current price data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoSnapshot {
    /// Latest trade.
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<CryptoTrade>,
    /// Latest quote.
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<CryptoQuote>,
    /// Minute bar.
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<CryptoBar>,
    /// Daily bar.
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<CryptoBar>,
    /// Previous daily bar.
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<CryptoBar>,
}

/// Crypto trade data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoTrade {
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Size.
    #[serde(rename = "s")]
    pub size: f64,
    /// Taker side.
    #[serde(rename = "tks")]
    pub taker_side: String,
    /// Trade ID.
    #[serde(rename = "i")]
    pub id: u64,
}

/// Crypto quote data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoQuote {
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid price.
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size.
    #[serde(rename = "bs")]
    pub bid_size: f64,
    /// Ask price.
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size.
    #[serde(rename = "as")]
    pub ask_size: f64,
}

/// Crypto bar data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoBar {
    /// Timestamp.
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
    /// Volume.
    #[serde(rename = "v")]
    pub volume: f64,
    /// Number of trades.
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub trade_count: Option<u64>,
    /// Volume-weighted average price.
    #[serde(rename = "vw", skip_serializing_if = "Option::is_none")]
    pub vwap: Option<f64>,
}

/// Crypto orderbook entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoOrderbookEntry {
    /// Price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Size.
    #[serde(rename = "s")]
    pub size: f64,
}

/// Crypto orderbook.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoOrderbook {
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid entries.
    #[serde(rename = "b")]
    pub bids: Vec<CryptoOrderbookEntry>,
    /// Ask entries.
    #[serde(rename = "a")]
    pub asks: Vec<CryptoOrderbookEntry>,
}

/// Parameters for crypto bars request.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CryptoBarsParams {
    /// Comma-separated list of symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Timeframe (e.g., "1Min", "1Hour", "1Day").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// Start time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Maximum number of bars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl CryptoBarsParams {
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

    /// Set limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

// ============================================================================
// News API Types
// ============================================================================

/// News content type.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NewsContentType {
    /// Article.
    Article,
    /// Video.
    Video,
    /// Audio.
    Audio,
}

/// News image size.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NewsImageSize {
    /// Thumbnail size.
    Thumb,
    /// Small size.
    Small,
    /// Large size.
    Large,
}

/// News image.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsImage {
    /// Image size.
    pub size: NewsImageSize,
    /// Image URL.
    pub url: String,
}

/// News source.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsSource {
    /// Source name.
    pub name: String,
    /// Source URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Favicon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
}

/// Enhanced news article with images and additional fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedNewsArticle {
    /// Article ID.
    pub id: u64,
    /// Headline.
    pub headline: String,
    /// Author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Full content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Article URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Images.
    #[serde(default)]
    pub images: Vec<NewsImage>,
    /// Related symbols.
    #[serde(default)]
    pub symbols: Vec<String>,
    /// Source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Parameters for news request.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewsParams {
    /// Filter by symbols (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Start time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Sort order (asc or desc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Include content in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,
    /// Exclude articles without content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_contentless: Option<bool>,
    /// Maximum number of articles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page token for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl NewsParams {
    /// Create new empty parameters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by symbols.
    #[must_use]
    pub fn symbols(mut self, symbols: &str) -> Self {
        self.symbols = Some(symbols.to_string());
        self
    }

    /// Set time range.
    #[must_use]
    pub fn time_range(mut self, start: &str, end: &str) -> Self {
        self.start = Some(start.to_string());
        self.end = Some(end.to_string());
        self
    }

    /// Sort descending (newest first).
    #[must_use]
    pub fn sort_desc(mut self) -> Self {
        self.sort = Some("desc".to_string());
        self
    }

    /// Sort ascending (oldest first).
    #[must_use]
    pub fn sort_asc(mut self) -> Self {
        self.sort = Some("asc".to_string());
        self
    }

    /// Include full content.
    #[must_use]
    pub fn with_content(mut self) -> Self {
        self.include_content = Some(true);
        self
    }

    /// Exclude articles without content.
    #[must_use]
    pub fn exclude_empty(mut self) -> Self {
        self.exclude_contentless = Some(true);
        self
    }

    /// Set limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set page token.
    #[must_use]
    pub fn page_token(mut self, token: &str) -> Self {
        self.page_token = Some(token.to_string());
        self
    }
}

// ============================================================================
// OAuth 2.0 Types
// ============================================================================

/// OAuth 2.0 scope.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OAuthScope {
    /// Read account information.
    #[serde(rename = "account:write")]
    AccountWrite,
    /// Trading access.
    Trading,
    /// Market data access.
    Data,
}

impl std::fmt::Display for OAuthScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthScope::AccountWrite => write!(f, "account:write"),
            OAuthScope::Trading => write!(f, "trading"),
            OAuthScope::Data => write!(f, "data"),
        }
    }
}

/// OAuth 2.0 configuration.
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// Client ID.
    pub client_id: String,
    /// Client secret.
    pub client_secret: String,
    /// Redirect URI.
    pub redirect_uri: String,
    /// Requested scopes.
    pub scopes: Vec<OAuthScope>,
}

impl OAuthConfig {
    /// Create new OAuth configuration.
    #[must_use]
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec![],
        }
    }

    /// Add a scope.
    #[must_use]
    pub fn scope(mut self, scope: OAuthScope) -> Self {
        self.scopes.push(scope);
        self
    }

    /// Add multiple scopes.
    #[must_use]
    pub fn scopes(mut self, scopes: Vec<OAuthScope>) -> Self {
        self.scopes.extend(scopes);
        self
    }

    /// Generate authorization URL.
    #[must_use]
    pub fn authorization_url(&self, state: &str) -> String {
        let scopes_str: String = self
            .scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "https://app.alpaca.markets/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&state={}&scope={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(state),
            urlencoding::encode(&scopes_str)
        )
    }
}

/// Request to exchange authorization code for token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthTokenRequest {
    /// Grant type.
    pub grant_type: String,
    /// Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Client ID.
    pub client_id: String,
    /// Client secret.
    pub client_secret: String,
    /// Redirect URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// Refresh token (for refresh grant).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl OAuthTokenRequest {
    /// Create authorization code exchange request.
    #[must_use]
    pub fn authorization_code(config: &OAuthConfig, code: &str) -> Self {
        Self {
            grant_type: "authorization_code".to_string(),
            code: Some(code.to_string()),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            redirect_uri: Some(config.redirect_uri.clone()),
            refresh_token: None,
        }
    }

    /// Create refresh token request.
    #[must_use]
    pub fn refresh(config: &OAuthConfig, refresh_token: &str) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            code: None,
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            redirect_uri: None,
            refresh_token: Some(refresh_token.to_string()),
        }
    }
}

/// Request to revoke a token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthRevokeRequest {
    /// Token to revoke.
    pub token: String,
    /// Client ID.
    pub client_id: String,
    /// Client secret.
    pub client_secret: String,
}

impl OAuthRevokeRequest {
    /// Create revoke request.
    #[must_use]
    pub fn new(config: &OAuthConfig, token: &str) -> Self {
        Self {
            token: token.to_string(),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
        }
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

    #[test]
    fn test_data_feed_serialization() {
        let feed = DataFeed::Sip;
        let json = serde_json::to_string(&feed).unwrap();
        assert_eq!(json, "\"sip\"");

        let feed = DataFeed::Iex;
        let json = serde_json::to_string(&feed).unwrap();
        assert_eq!(json, "\"iex\"");
    }

    #[test]
    fn test_corporate_action_type_serialization() {
        let action = CorporateActionType::Dividend;
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(json, "\"dividend\"");

        let action = CorporateActionType::Split;
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(json, "\"split\"");
    }

    #[test]
    fn test_multi_bars_params_builder() {
        let params = MultiBarsParams::new("AAPL,MSFT,GOOGL")
            .timeframe("1Day")
            .time_range("2024-01-01", "2024-03-01")
            .feed(DataFeed::Sip)
            .limit(100);

        assert_eq!(params.symbols, Some("AAPL,MSFT,GOOGL".to_string()));
        assert_eq!(params.timeframe, Some("1Day".to_string()));
        assert_eq!(params.feed, Some(DataFeed::Sip));
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_corporate_actions_params_builder() {
        let params = CorporateActionsParams::new()
            .symbols("AAPL,MSFT")
            .types("dividend,split")
            .date_range("2024-01-01", "2024-12-31")
            .limit(50);

        assert_eq!(params.symbols, Some("AAPL,MSFT".to_string()));
        assert_eq!(params.types, Some("dividend,split".to_string()));
        assert_eq!(params.start, Some("2024-01-01".to_string()));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_broker_account_status_serialization() {
        let status = BrokerAccountStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"ACTIVE\"");

        let status = BrokerAccountStatus::Onboarding;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"ONBOARDING\"");
    }

    #[test]
    fn test_agreement_type_serialization() {
        let agreement = AgreementType::CustomerAgreement;
        let json = serde_json::to_string(&agreement).unwrap();
        assert_eq!(json, "\"customer_agreement\"");
    }

    #[test]
    fn test_contact_builder() {
        let contact = Contact::new("test@example.com", "New York", "10001", "USA")
            .phone("+1234567890")
            .street("123 Main St")
            .state("NY");

        assert_eq!(contact.email_address, "test@example.com");
        assert_eq!(contact.city, "New York");
        assert_eq!(contact.phone_number, Some("+1234567890".to_string()));
        assert_eq!(contact.state, Some("NY".to_string()));
    }

    #[test]
    fn test_identity_builder() {
        let identity = Identity::new("John", "Doe", "1990-01-15")
            .tax_id("123-45-6789", TaxIdType::UsaSsn)
            .citizenship("USA");

        assert_eq!(identity.given_name, "John");
        assert_eq!(identity.family_name, "Doe");
        assert_eq!(identity.tax_id, Some("123-45-6789".to_string()));
        assert_eq!(identity.tax_id_type, Some(TaxIdType::UsaSsn));
    }

    #[test]
    fn test_disclosures_builder() {
        let disclosures = Disclosures::new().control_person(false).employment(
            "employed",
            "Acme Corp",
            "Engineer",
        );

        assert!(!disclosures.is_control_person);
        assert_eq!(disclosures.employer_name, Some("Acme Corp".to_string()));
    }

    #[test]
    fn test_transfer_status_serialization() {
        let status = TransferStatus::Complete;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"COMPLETE\"");
    }

    #[test]
    fn test_create_ach_relationship_request() {
        let request = CreateAchRelationshipRequest::new(
            "John Doe",
            BankAccountType::Checking,
            "123456789",
            "021000021",
        )
        .nickname("Primary Account");

        assert_eq!(request.account_owner_name, "John Doe");
        assert_eq!(request.bank_account_type, BankAccountType::Checking);
        assert_eq!(request.nickname, Some("Primary Account".to_string()));
    }

    #[test]
    fn test_create_transfer_request_ach() {
        let request = CreateTransferRequest::ach("rel-123", "1000.00", TransferDirection::Incoming);

        assert_eq!(request.transfer_type, TransferType::Ach);
        assert_eq!(request.relationship_id, Some("rel-123".to_string()));
        assert_eq!(request.amount, "1000.00");
        assert_eq!(request.direction, TransferDirection::Incoming);
    }

    #[test]
    fn test_create_journal_request_cash() {
        let request =
            CreateJournalRequest::cash("acc-from", "acc-to", "500.00").description("Test transfer");

        assert_eq!(request.entry_type, JournalEntryType::Jnlc);
        assert_eq!(request.amount, Some("500.00".to_string()));
        assert_eq!(request.description, Some("Test transfer".to_string()));
    }

    #[test]
    fn test_crypto_chain_serialization() {
        let chain = CryptoChain::Eth;
        let json = serde_json::to_string(&chain).unwrap();
        assert_eq!(json, "\"ETH\"");
    }

    #[test]
    fn test_crypto_transfer_status_serialization() {
        let status = CryptoTransferStatus::Complete;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"COMPLETE\"");
    }

    #[test]
    fn test_create_crypto_whitelist_request() {
        let request =
            CreateCryptoWhitelistRequest::new("BTC", "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh")
                .label("My Hardware Wallet");

        assert_eq!(request.asset, "BTC");
        assert_eq!(request.label, Some("My Hardware Wallet".to_string()));
    }

    #[test]
    fn test_crypto_bars_params_builder() {
        let params = CryptoBarsParams::new("BTC/USD,ETH/USD")
            .timeframe("1Hour")
            .limit(100);

        assert_eq!(params.symbols, Some("BTC/USD,ETH/USD".to_string()));
        assert_eq!(params.timeframe, Some("1Hour".to_string()));
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_news_content_type_serialization() {
        let content_type = NewsContentType::Article;
        let json = serde_json::to_string(&content_type).unwrap();
        assert_eq!(json, "\"article\"");
    }

    #[test]
    fn test_news_params_builder() {
        let params = NewsParams::new()
            .symbols("AAPL,MSFT")
            .sort_desc()
            .with_content()
            .limit(50);

        assert_eq!(params.symbols, Some("AAPL,MSFT".to_string()));
        assert_eq!(params.sort, Some("desc".to_string()));
        assert_eq!(params.include_content, Some(true));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_oauth_scope_display() {
        assert_eq!(OAuthScope::AccountWrite.to_string(), "account:write");
        assert_eq!(OAuthScope::Trading.to_string(), "trading");
        assert_eq!(OAuthScope::Data.to_string(), "data");
    }

    #[test]
    fn test_oauth_config_builder() {
        let config = OAuthConfig::new("client123", "secret456", "https://example.com/callback")
            .scope(OAuthScope::Trading)
            .scope(OAuthScope::Data);

        assert_eq!(config.client_id, "client123");
        assert_eq!(config.scopes.len(), 2);
    }

    #[test]
    fn test_oauth_token_authorization_header() {
        let token = crate::OAuthToken {
            access_token: "abc123".to_string(),
            refresh_token: Some("refresh456".to_string()),
            token_type: "Bearer".to_string(),
            expires_in: Some(3600),
            scope: Some("trading data".to_string()),
        };

        assert_eq!(token.auth_header(), "Bearer abc123");
        assert!(token.has_refresh_token());
    }
}
