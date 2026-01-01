//! HTTP API endpoints for Alpaca trading platform.
//!
//! This module provides implementations for all Alpaca REST API endpoints
//! including account management, order operations, position management,
//! market data, and more.

#![allow(missing_docs)]

use crate::client::AlpacaHttpClient;
use alpaca_base::{OAuthToken, Result, types::*};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl AlpacaHttpClient {
    // Account endpoints

    /// Get account information
    pub async fn get_account(&self) -> Result<Account> {
        self.get("/v2/account").await
    }

    /// Get account configurations
    pub async fn get_account_configurations(&self) -> Result<AccountConfigurations> {
        self.get("/v2/account/configurations").await
    }

    /// Update account configurations
    pub async fn update_account_configurations(
        &self,
        config: &AccountConfigurations,
    ) -> Result<AccountConfigurations> {
        self.patch("/v2/account/configurations", config).await
    }

    /// Get account activities
    pub async fn get_account_activities(
        &self,
        params: &ActivityParams,
    ) -> Result<Vec<AccountActivity>> {
        self.get_with_params("/v2/account/activities", params).await
    }

    /// Get portfolio history
    pub async fn get_portfolio_history(
        &self,
        params: &PortfolioHistoryParams,
    ) -> Result<PortfolioHistory> {
        self.get_with_params("/v2/account/portfolio/history", params)
            .await
    }

    // Asset endpoints

    /// Get all assets
    pub async fn get_assets(&self, params: &AssetParams) -> Result<Vec<Asset>> {
        self.get_with_params("/v2/assets", params).await
    }

    /// Get asset by ID
    pub async fn get_asset(&self, asset_id: &Uuid) -> Result<Asset> {
        self.get(&format!("/v2/assets/{}", asset_id)).await
    }

    /// Get asset by symbol
    pub async fn get_asset_by_symbol(&self, symbol: &str) -> Result<Asset> {
        self.get(&format!("/v2/assets/{}", symbol)).await
    }

    // Order endpoints

    /// Get all orders
    pub async fn get_orders(&self, params: &OrderParams) -> Result<Vec<Order>> {
        self.get_with_params("/v2/orders", params).await
    }

    /// Create a new order
    pub async fn create_order(&self, order: &CreateOrderRequest) -> Result<Order> {
        self.post("/v2/orders", order).await
    }

    /// Get order by ID
    pub async fn get_order(&self, order_id: &Uuid) -> Result<Order> {
        self.get(&format!("/v2/orders/{}", order_id)).await
    }

    /// Get order by client order ID
    pub async fn get_order_by_client_id(&self, client_order_id: &str) -> Result<Order> {
        self.get(&format!(
            "/v2/orders:by_client_order_id?client_order_id={}",
            client_order_id
        ))
        .await
    }

    /// Replace an order
    pub async fn replace_order(
        &self,
        order_id: &Uuid,
        order: &ReplaceOrderRequest,
    ) -> Result<Order> {
        self.patch(&format!("/v2/orders/{}", order_id), order).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: &Uuid) -> Result<()> {
        self.delete(&format!("/v2/orders/{}", order_id)).await
    }

    /// Cancel all orders
    pub async fn cancel_all_orders(&self) -> Result<Vec<CancelOrderResponse>> {
        self.delete("/v2/orders").await
    }

    // Position endpoints

    /// Get all positions
    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        self.get("/v2/positions").await
    }

    /// Get position by symbol
    pub async fn get_position(&self, symbol: &str) -> Result<Position> {
        self.get(&format!("/v2/positions/{}", symbol)).await
    }

    /// Close all positions
    pub async fn close_all_positions(
        &self,
        cancel_orders: bool,
    ) -> Result<Vec<ClosePositionResponse>> {
        let url = format!("/v2/positions?cancel_orders={}", cancel_orders);
        self.delete(&url).await
    }

    /// Close position by symbol
    pub async fn close_position(
        &self,
        symbol: &str,
        _params: &ClosePositionRequest,
    ) -> Result<Order> {
        self.delete(&format!("/v2/positions/{}", symbol)).await
    }

    // Watchlist endpoints

    /// Get all watchlists
    pub async fn get_watchlists(&self) -> Result<Vec<Watchlist>> {
        self.get("/v2/watchlists").await
    }

    /// Create a new watchlist
    pub async fn create_watchlist(&self, watchlist: &CreateWatchlistRequest) -> Result<Watchlist> {
        self.post("/v2/watchlists", watchlist).await
    }

    /// Get watchlist by ID
    pub async fn get_watchlist(&self, watchlist_id: &Uuid) -> Result<Watchlist> {
        self.get(&format!("/v2/watchlists/{}", watchlist_id)).await
    }

    /// Update watchlist
    pub async fn update_watchlist(
        &self,
        watchlist_id: &Uuid,
        watchlist: &UpdateWatchlistRequest,
    ) -> Result<Watchlist> {
        self.put(&format!("/v2/watchlists/{}", watchlist_id), watchlist)
            .await
    }

    /// Delete watchlist
    pub async fn delete_watchlist(&self, watchlist_id: &Uuid) -> Result<()> {
        self.delete(&format!("/v2/watchlists/{}", watchlist_id))
            .await
    }

    /// Add asset to watchlist
    pub async fn add_to_watchlist(&self, watchlist_id: &Uuid, symbol: &str) -> Result<Watchlist> {
        let request = AddToWatchlistRequest {
            symbol: symbol.to_string(),
        };
        self.post(&format!("/v2/watchlists/{}", watchlist_id), &request)
            .await
    }

    /// Remove asset from watchlist
    pub async fn remove_from_watchlist(&self, watchlist_id: &Uuid, symbol: &str) -> Result<()> {
        self.delete(&format!("/v2/watchlists/{}/{}", watchlist_id, symbol))
            .await
    }

    // Market data endpoints

    /// Get bars for a symbol
    pub async fn get_bars(&self, symbol: &str, params: &BarsParams) -> Result<BarsResponse> {
        self.get_with_params(&format!("/v2/stocks/{}/bars", symbol), params)
            .await
    }

    /// Get quotes for a symbol
    pub async fn get_quotes(&self, symbol: &str, params: &QuotesParams) -> Result<QuotesResponse> {
        self.get_with_params(&format!("/v2/stocks/{}/quotes", symbol), params)
            .await
    }

    /// Get trades for a symbol
    pub async fn get_trades(&self, symbol: &str, params: &TradesParams) -> Result<TradesResponse> {
        self.get_with_params(&format!("/v2/stocks/{}/trades", symbol), params)
            .await
    }

    /// Get latest bar for a symbol
    pub async fn get_latest_bar(&self, symbol: &str) -> Result<LatestBarResponse> {
        self.get(&format!("/v2/stocks/{}/bars/latest", symbol))
            .await
    }

    /// Get latest quote for a symbol
    pub async fn get_latest_quote(&self, symbol: &str) -> Result<LatestQuoteResponse> {
        self.get(&format!("/v2/stocks/{}/quotes/latest", symbol))
            .await
    }

    /// Get latest trade for a symbol
    pub async fn get_latest_trade(&self, symbol: &str) -> Result<LatestTradeResponse> {
        self.get(&format!("/v2/stocks/{}/trades/latest", symbol))
            .await
    }

    // Calendar and clock endpoints

    /// Get market calendar
    pub async fn get_calendar(&self, params: &CalendarParams) -> Result<Vec<Calendar>> {
        self.get_with_params("/v2/calendar", params).await
    }

    /// Get market clock
    pub async fn get_clock(&self) -> Result<Clock> {
        self.get("/v2/clock").await
    }

    // News endpoints

    /// Get news articles
    pub async fn get_news(&self, params: &NewsParams) -> Result<NewsResponse> {
        self.get_with_params("/v1beta1/news", params).await
    }

    // Crypto endpoints

    /// Get crypto bars
    pub async fn get_crypto_bars(
        &self,
        symbol: &str,
        params: &CryptoBarsParams,
    ) -> Result<CryptoBarsResponse> {
        self.get_with_params(&format!("/v1beta1/crypto/{}/bars", symbol), params)
            .await
    }

    /// Get crypto quotes
    pub async fn get_crypto_quotes(
        &self,
        symbol: &str,
        params: &CryptoQuotesParams,
    ) -> Result<CryptoQuotesResponse> {
        self.get_with_params(&format!("/v1beta1/crypto/{}/quotes", symbol), params)
            .await
    }

    /// Get crypto trades
    pub async fn get_crypto_trades(
        &self,
        symbol: &str,
        params: &CryptoTradesParams,
    ) -> Result<CryptoTradesResponse> {
        self.get_with_params(&format!("/v1beta1/crypto/{}/trades", symbol), params)
            .await
    }
}

// Request/Response types

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountConfigurations {
    pub dtbp_check: Option<String>,
    pub trade_confirm_email: Option<String>,
    pub suspend_trade: Option<bool>,
    pub no_shorting: Option<bool>,
    pub max_margin_multiplier: Option<String>,
    pub pdt_check: Option<String>,
    pub max_dte: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityParams {
    pub activity_type: Option<ActivityType>,
    pub date: Option<String>,
    pub until: Option<String>,
    pub after: Option<String>,
    pub direction: Option<String>,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioHistoryParams {
    pub period: Option<String>,
    pub timeframe: Option<String>,
    pub date_end: Option<String>,
    pub extended_hours: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetParams {
    pub status: Option<AssetStatus>,
    pub asset_class: Option<AssetClass>,
    pub exchange: Option<String>,
    pub attributes: Option<String>,
}

/// Parameters for querying orders.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderParams {
    /// Filter by order status (open, closed, all).
    pub status: Option<OrderQueryStatus>,
    /// Maximum number of orders to return (default 50, max 500).
    pub limit: Option<u32>,
    /// Filter orders created after this timestamp.
    pub after: Option<DateTime<Utc>>,
    /// Filter orders created until this timestamp.
    pub until: Option<DateTime<Utc>>,
    /// Sort direction (asc or desc).
    pub direction: Option<SortDirection>,
    /// Include nested leg orders for multi-leg orders.
    pub nested: Option<bool>,
    /// Comma-separated list of symbols to filter by.
    pub symbols: Option<String>,
    /// Filter by order side (buy or sell).
    pub side: Option<OrderSide>,
}

impl OrderParams {
    /// Creates a new empty OrderParams.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the status filter.
    #[must_use]
    pub fn status(mut self, status: OrderQueryStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the limit.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the after timestamp filter.
    #[must_use]
    pub fn after(mut self, after: DateTime<Utc>) -> Self {
        self.after = Some(after);
        self
    }

    /// Sets the until timestamp filter.
    #[must_use]
    pub fn until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until);
        self
    }

    /// Sets the sort direction.
    #[must_use]
    pub fn direction(mut self, direction: SortDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Enables nested leg orders in response.
    #[must_use]
    pub fn nested(mut self, nested: bool) -> Self {
        self.nested = Some(nested);
        self
    }

    /// Sets the symbols filter.
    #[must_use]
    pub fn symbols(mut self, symbols: impl Into<String>) -> Self {
        self.symbols = Some(symbols.into());
        self
    }

    /// Sets the side filter.
    #[must_use]
    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }
}

/// Request to create a new order.
///
/// Supports all order types including simple, bracket, OCO, and OTO orders.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    /// The symbol to trade.
    pub symbol: String,
    /// The quantity to trade (mutually exclusive with notional).
    pub qty: Option<String>,
    /// The notional dollar amount to trade (mutually exclusive with qty).
    pub notional: Option<String>,
    /// The side of the order (buy or sell).
    pub side: OrderSide,
    /// The type of order.
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// How long the order remains active.
    pub time_in_force: TimeInForce,
    /// Limit price for limit orders.
    pub limit_price: Option<String>,
    /// Stop price for stop orders.
    pub stop_price: Option<String>,
    /// Trail price for trailing stop orders (dollar amount).
    pub trail_price: Option<String>,
    /// Trail percent for trailing stop orders (percentage).
    pub trail_percent: Option<String>,
    /// Whether to allow trading during extended hours.
    pub extended_hours: Option<bool>,
    /// Client-specified order ID for idempotency.
    pub client_order_id: Option<String>,
    /// Order class (simple, bracket, oco, oto).
    pub order_class: Option<OrderClass>,
    /// Take profit configuration for bracket orders.
    pub take_profit: Option<TakeProfit>,
    /// Stop loss configuration for bracket orders.
    pub stop_loss: Option<StopLoss>,
    /// Position intent for options orders.
    pub position_intent: Option<PositionIntent>,
    /// Good-till-date expiration (for GTD orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtd_date: Option<NaiveDate>,
}

impl CreateOrderRequest {
    /// Creates a new market order request.
    #[must_use]
    pub fn market(symbol: impl Into<String>, side: OrderSide, qty: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            ..Default::default()
        }
    }

    /// Creates a new limit order request.
    #[must_use]
    pub fn limit(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        limit_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            limit_price: Some(limit_price.into()),
            ..Default::default()
        }
    }

    /// Creates a new stop order request.
    #[must_use]
    pub fn stop(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        stop_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::Stop,
            time_in_force: TimeInForce::Day,
            stop_price: Some(stop_price.into()),
            ..Default::default()
        }
    }

    /// Creates a new stop-limit order request.
    #[must_use]
    pub fn stop_limit(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        stop_price: impl Into<String>,
        limit_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::StopLimit,
            time_in_force: TimeInForce::Day,
            stop_price: Some(stop_price.into()),
            limit_price: Some(limit_price.into()),
            ..Default::default()
        }
    }

    /// Creates a new trailing stop order request with a trail price.
    #[must_use]
    pub fn trailing_stop_price(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        trail_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::TrailingStop,
            time_in_force: TimeInForce::Day,
            trail_price: Some(trail_price.into()),
            ..Default::default()
        }
    }

    /// Creates a new trailing stop order request with a trail percent.
    #[must_use]
    pub fn trailing_stop_percent(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        trail_percent: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::TrailingStop,
            time_in_force: TimeInForce::Day,
            trail_percent: Some(trail_percent.into()),
            ..Default::default()
        }
    }

    /// Creates a new bracket order request.
    ///
    /// A bracket order is a set of three orders: a primary order and two
    /// conditional orders (take profit and stop loss) that are triggered
    /// when the primary order fills.
    #[must_use]
    pub fn bracket(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        order_type: OrderType,
        take_profit: TakeProfit,
        stop_loss: StopLoss,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type,
            time_in_force: TimeInForce::Day,
            order_class: Some(OrderClass::Bracket),
            take_profit: Some(take_profit),
            stop_loss: Some(stop_loss),
            ..Default::default()
        }
    }

    /// Creates a new OCO (One-Cancels-Other) order request.
    ///
    /// An OCO order is a set of two orders where if one is executed,
    /// the other is automatically canceled.
    #[must_use]
    pub fn oco(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        take_profit: TakeProfit,
        stop_loss: StopLoss,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            order_class: Some(OrderClass::Oco),
            take_profit: Some(take_profit),
            stop_loss: Some(stop_loss),
            ..Default::default()
        }
    }

    /// Creates a new OTO (One-Triggers-Other) order request.
    ///
    /// An OTO order is a primary order that, when filled, triggers
    /// a secondary order.
    #[must_use]
    pub fn oto(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        order_type: OrderType,
        stop_loss: StopLoss,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            side,
            order_type,
            time_in_force: TimeInForce::Day,
            order_class: Some(OrderClass::Oto),
            stop_loss: Some(stop_loss),
            ..Default::default()
        }
    }

    /// Sets the time in force for the order.
    #[must_use]
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// Sets the limit price for the order.
    #[must_use]
    pub fn with_limit_price(mut self, price: impl Into<String>) -> Self {
        self.limit_price = Some(price.into());
        self
    }

    /// Enables extended hours trading.
    #[must_use]
    pub fn extended_hours(mut self, enabled: bool) -> Self {
        self.extended_hours = Some(enabled);
        self
    }

    /// Sets a client order ID for idempotency.
    #[must_use]
    pub fn client_order_id(mut self, id: impl Into<String>) -> Self {
        self.client_order_id = Some(id.into());
        self
    }

    /// Sets the position intent for options orders.
    #[must_use]
    pub fn position_intent(mut self, intent: PositionIntent) -> Self {
        self.position_intent = Some(intent);
        self
    }

    /// Sets the GTD (Good Till Date) expiration date.
    #[must_use]
    pub fn gtd_date(mut self, date: NaiveDate) -> Self {
        self.time_in_force = TimeInForce::Gtd;
        self.gtd_date = Some(date);
        self
    }
}

/// Request to replace (modify) an existing order.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplaceOrderRequest {
    /// New quantity for the order.
    pub qty: Option<String>,
    /// New time in force.
    pub time_in_force: Option<TimeInForce>,
    /// New limit price.
    pub limit_price: Option<String>,
    /// New stop price.
    pub stop_price: Option<String>,
    /// New trail value (price or percent depending on original order).
    pub trail: Option<String>,
    /// New client order ID.
    pub client_order_id: Option<String>,
}

impl ReplaceOrderRequest {
    /// Creates a new empty replace order request.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new quantity.
    #[must_use]
    pub fn qty(mut self, qty: impl Into<String>) -> Self {
        self.qty = Some(qty.into());
        self
    }

    /// Sets the new time in force.
    #[must_use]
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Sets the new limit price.
    #[must_use]
    pub fn limit_price(mut self, price: impl Into<String>) -> Self {
        self.limit_price = Some(price.into());
        self
    }

    /// Sets the new stop price.
    #[must_use]
    pub fn stop_price(mut self, price: impl Into<String>) -> Self {
        self.stop_price = Some(price.into());
        self
    }

    /// Sets the new trail value.
    #[must_use]
    pub fn trail(mut self, trail: impl Into<String>) -> Self {
        self.trail = Some(trail.into());
        self
    }

    /// Sets the new client order ID.
    #[must_use]
    pub fn client_order_id(mut self, id: impl Into<String>) -> Self {
        self.client_order_id = Some(id.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    pub id: Uuid,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosePositionResponse {
    pub symbol: String,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosePositionRequest {
    pub qty: Option<String>,
    pub percentage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWatchlistRequest {
    pub name: String,
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWatchlistRequest {
    pub name: Option<String>,
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddToWatchlistRequest {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarsParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub timeframe: Option<String>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
    pub asof: Option<String>,
    pub feed: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarsResponse {
    pub bars: Vec<Bar>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
    pub asof: Option<String>,
    pub feed: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotesResponse {
    pub quotes: Vec<Quote>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
    pub asof: Option<String>,
    pub feed: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesResponse {
    pub trades: Vec<Trade>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestBarResponse {
    pub bar: Bar,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestQuoteResponse {
    pub quote: Quote,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestTradeResponse {
    pub trade: Trade,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarParams {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsParams {
    pub symbols: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub sort: Option<String>,
    pub include_content: Option<bool>,
    pub exclude_contentless: Option<bool>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsResponse {
    pub news: Vec<NewsArticle>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoBarsParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub timeframe: Option<String>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoBarsResponse {
    pub bars: Vec<Bar>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoQuotesParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoQuotesResponse {
    pub quotes: Vec<Quote>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoTradesParams {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub page_token: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoTradesResponse {
    pub trades: Vec<Trade>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

// ============================================================================
// Options Trading Endpoints
// ============================================================================

/// Response for listing option contracts.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionContractsResponse {
    /// List of option contracts.
    pub option_contracts: Vec<OptionContract>,
    /// Token for next page of results.
    pub next_page_token: Option<String>,
}

/// Response for option bars.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionBarsResponse {
    /// Map of symbol to bars.
    pub bars: std::collections::HashMap<String, Vec<OptionBar>>,
    /// Token for next page of results.
    pub next_page_token: Option<String>,
}

/// Response for option snapshots.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionSnapshotsResponse {
    /// Map of symbol to snapshot.
    pub snapshots: std::collections::HashMap<String, OptionSnapshot>,
}

impl AlpacaHttpClient {
    // ========================================================================
    // Options Contract Endpoints
    // ========================================================================

    /// List option contracts with optional filters.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering contracts
    ///
    /// # Returns
    /// List of option contracts matching the filters
    pub async fn get_option_contracts(
        &self,
        params: &OptionContractParams,
    ) -> Result<OptionContractsResponse> {
        self.get_with_params("/v2/options/contracts", params).await
    }

    /// Get a specific option contract by symbol or ID.
    ///
    /// # Arguments
    /// * `symbol_or_id` - OCC symbol or contract ID
    ///
    /// # Returns
    /// The option contract details
    pub async fn get_option_contract(&self, symbol_or_id: &str) -> Result<OptionContract> {
        self.get(&format!("/v2/options/contracts/{}", symbol_or_id))
            .await
    }

    /// Exercise an option contract.
    ///
    /// # Arguments
    /// * `request` - Exercise request with symbol and quantity
    ///
    /// # Returns
    /// The resulting order from exercising the option
    pub async fn exercise_option(&self, request: &OptionExerciseRequest) -> Result<Order> {
        self.post("/v2/options/exercise", request).await
    }

    // ========================================================================
    // Options Market Data Endpoints
    // ========================================================================

    /// Get historical bars for option contracts.
    ///
    /// # Arguments
    /// * `params` - Query parameters including symbols, timeframe, and date range
    ///
    /// # Returns
    /// Historical bar data for the requested options
    pub async fn get_option_bars(&self, params: &OptionBarsParams) -> Result<OptionBarsResponse> {
        self.get_with_params("/v1beta1/options/bars", params).await
    }

    /// Get snapshots for option contracts.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of option symbols
    ///
    /// # Returns
    /// Current snapshots with latest quote, trade, and greeks
    pub async fn get_option_snapshots(&self, symbols: &str) -> Result<OptionSnapshotsResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v1beta1/options/snapshots", &Params { symbols })
            .await
    }

    /// Get the options chain for an underlying symbol.
    ///
    /// # Arguments
    /// * `underlying_symbol` - The underlying asset symbol (e.g., "AAPL")
    ///
    /// # Returns
    /// Options chain with all available contracts and snapshots
    pub async fn get_option_chain(
        &self,
        underlying_symbol: &str,
    ) -> Result<OptionSnapshotsResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            underlying_symbol: &'a str,
        }
        self.get_with_params("/v1beta1/options/snapshots", &Params { underlying_symbol })
            .await
    }
}

// ============================================================================
// Enhanced Stock Market Data Endpoints
// ============================================================================

/// Response for multi-symbol bars.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiBarsResponse {
    /// Map of symbol to bars.
    pub bars: std::collections::HashMap<String, Vec<Bar>>,
    /// Token for next page of results.
    pub next_page_token: Option<String>,
}

/// Response for multi-symbol quotes.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiQuotesResponse {
    /// Map of symbol to quotes.
    pub quotes: std::collections::HashMap<String, Vec<Quote>>,
    /// Token for next page of results.
    pub next_page_token: Option<String>,
}

/// Response for multi-symbol trades.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiTradesResponse {
    /// Map of symbol to trades.
    pub trades: std::collections::HashMap<String, Vec<Trade>>,
    /// Token for next page of results.
    pub next_page_token: Option<String>,
}

/// Response for stock snapshots.
#[derive(Debug, Serialize, Deserialize)]
pub struct StockSnapshotsResponse {
    /// Map of symbol to snapshot.
    #[serde(flatten)]
    pub snapshots: std::collections::HashMap<String, StockSnapshot>,
}

/// Response for corporate actions.
#[derive(Debug, Serialize, Deserialize)]
pub struct CorporateActionsResponse {
    /// List of corporate actions.
    pub corporate_actions: Vec<CorporateAction>,
    /// Token for next page of results.
    pub next_page_token: Option<String>,
}

/// Response for latest bars.
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestBarsResponse {
    /// Map of symbol to latest bar.
    pub bars: std::collections::HashMap<String, Bar>,
}

/// Response for latest quotes.
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestQuotesResponse {
    /// Map of symbol to latest quote.
    pub quotes: std::collections::HashMap<String, Quote>,
}

/// Response for latest trades.
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestTradesResponse {
    /// Map of symbol to latest trade.
    pub trades: std::collections::HashMap<String, Trade>,
}

impl AlpacaHttpClient {
    // ========================================================================
    // Multi-Symbol Market Data Endpoints
    // ========================================================================

    /// Get historical bars for multiple symbols.
    ///
    /// # Arguments
    /// * `params` - Query parameters including symbols, timeframe, and date range
    ///
    /// # Returns
    /// Historical bar data for all requested symbols
    pub async fn get_stock_bars(&self, params: &MultiBarsParams) -> Result<MultiBarsResponse> {
        self.get_with_params("/v2/stocks/bars", params).await
    }

    /// Get historical quotes for multiple symbols.
    ///
    /// # Arguments
    /// * `params` - Query parameters including symbols and date range
    ///
    /// # Returns
    /// Historical quote data for all requested symbols
    pub async fn get_stock_quotes(
        &self,
        params: &MultiQuotesParams,
    ) -> Result<MultiQuotesResponse> {
        self.get_with_params("/v2/stocks/quotes", params).await
    }

    /// Get historical trades for multiple symbols.
    ///
    /// # Arguments
    /// * `params` - Query parameters including symbols and date range
    ///
    /// # Returns
    /// Historical trade data for all requested symbols
    pub async fn get_stock_trades(
        &self,
        params: &MultiTradesParams,
    ) -> Result<MultiTradesResponse> {
        self.get_with_params("/v2/stocks/trades", params).await
    }

    /// Get snapshots for multiple symbols.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Current snapshots with latest trade, quote, and bars
    pub async fn get_stock_snapshots(&self, symbols: &str) -> Result<StockSnapshotsResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v2/stocks/snapshots", &Params { symbols })
            .await
    }

    // ========================================================================
    // Latest Market Data Endpoints
    // ========================================================================

    /// Get latest bars for multiple symbols.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Latest bar for each symbol
    pub async fn get_latest_bars(&self, symbols: &str) -> Result<LatestBarsResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v2/stocks/bars/latest", &Params { symbols })
            .await
    }

    /// Get latest quotes for multiple symbols.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Latest quote for each symbol
    pub async fn get_latest_quotes(&self, symbols: &str) -> Result<LatestQuotesResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v2/stocks/quotes/latest", &Params { symbols })
            .await
    }

    /// Get latest trades for multiple symbols.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Latest trade for each symbol
    pub async fn get_latest_trades(&self, symbols: &str) -> Result<LatestTradesResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v2/stocks/trades/latest", &Params { symbols })
            .await
    }

    // ========================================================================
    // Corporate Actions Endpoints
    // ========================================================================

    /// Get corporate action announcements.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering corporate actions
    ///
    /// # Returns
    /// List of corporate action announcements
    pub async fn get_corporate_actions(
        &self,
        params: &CorporateActionsParams,
    ) -> Result<CorporateActionsResponse> {
        self.get_with_params("/v1beta1/corporate-actions/announcements", params)
            .await
    }
}

// ============================================================================
// Broker API - Account Management Endpoints
// ============================================================================

impl AlpacaHttpClient {
    // ========================================================================
    // Broker Account Endpoints
    // ========================================================================

    /// Create a new broker account.
    ///
    /// # Arguments
    /// * `request` - Account creation request with KYC data
    ///
    /// # Returns
    /// The created broker account
    pub async fn create_broker_account(
        &self,
        request: &CreateBrokerAccountRequest,
    ) -> Result<BrokerAccount> {
        self.post("/v1/accounts", request).await
    }

    /// List all broker accounts.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters for filtering
    ///
    /// # Returns
    /// List of broker accounts
    pub async fn list_broker_accounts(
        &self,
        params: &ListBrokerAccountsParams,
    ) -> Result<Vec<BrokerAccount>> {
        self.get_with_params("/v1/accounts", params).await
    }

    /// Get a broker account by ID.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// The broker account
    pub async fn get_broker_account(&self, account_id: &str) -> Result<BrokerAccount> {
        self.get(&format!("/v1/accounts/{}", account_id)).await
    }

    /// Update a broker account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `request` - Update request with fields to change
    ///
    /// # Returns
    /// The updated broker account
    pub async fn update_broker_account(
        &self,
        account_id: &str,
        request: &UpdateBrokerAccountRequest,
    ) -> Result<BrokerAccount> {
        self.patch(&format!("/v1/accounts/{}", account_id), request)
            .await
    }

    /// Close a broker account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID to close
    pub async fn close_broker_account(&self, account_id: &str) -> Result<()> {
        self.delete(&format!("/v1/accounts/{}", account_id)).await
    }

    /// Get trading account details for a broker account.
    ///
    /// # Arguments
    /// * `account_id` - The broker account ID
    ///
    /// # Returns
    /// Trading account details
    pub async fn get_broker_trading_account(&self, account_id: &str) -> Result<Account> {
        self.get(&format!("/v1/accounts/{}/trading", account_id))
            .await
    }

    // ========================================================================
    // CIP (Customer Identification Program) Endpoints
    // ========================================================================

    /// Submit CIP data for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `cip_info` - CIP information to submit
    ///
    /// # Returns
    /// The submitted CIP info
    pub async fn submit_cip(&self, account_id: &str, cip_info: &CipInfo) -> Result<CipInfo> {
        self.post(&format!("/v1/accounts/{}/cip", account_id), cip_info)
            .await
    }

    /// Get CIP status for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// CIP information
    pub async fn get_cip(&self, account_id: &str) -> Result<CipInfo> {
        self.get(&format!("/v1/accounts/{}/cip", account_id)).await
    }

    // ========================================================================
    // Document Endpoints
    // ========================================================================

    /// Upload a document for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `document` - Document to upload
    ///
    /// # Returns
    /// Upload confirmation
    pub async fn upload_document(
        &self,
        account_id: &str,
        document: &Document,
    ) -> Result<DocumentUploadResponse> {
        self.post(
            &format!("/v1/accounts/{}/documents/upload", account_id),
            document,
        )
        .await
    }

    /// List documents for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// List of documents
    pub async fn list_documents(&self, account_id: &str) -> Result<Vec<DocumentInfo>> {
        self.get(&format!("/v1/accounts/{}/documents", account_id))
            .await
    }

    /// Get a specific document.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `document_id` - The document ID
    ///
    /// # Returns
    /// Document information
    pub async fn get_document(&self, account_id: &str, document_id: &str) -> Result<DocumentInfo> {
        self.get(&format!(
            "/v1/accounts/{}/documents/{}",
            account_id, document_id
        ))
        .await
    }

    /// Delete a document.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `document_id` - The document ID
    pub async fn delete_document(&self, account_id: &str, document_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/accounts/{}/documents/{}",
            account_id, document_id
        ))
        .await
    }
}

/// Response for document upload.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentUploadResponse {
    /// Document ID.
    pub id: String,
    /// Document type.
    pub document_type: DocumentType,
    /// Upload status.
    pub status: String,
}

/// Document information.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentInfo {
    /// Document ID.
    pub id: String,
    /// Document type.
    pub document_type: DocumentType,
    /// Document sub-type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_sub_type: Option<String>,
    /// Created at timestamp.
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// Broker API - Funding & Transfers Endpoints
// ============================================================================

impl AlpacaHttpClient {
    // ========================================================================
    // ACH Relationship Endpoints
    // ========================================================================

    /// Create an ACH relationship for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `request` - ACH relationship creation request
    ///
    /// # Returns
    /// The created ACH relationship
    pub async fn create_ach_relationship(
        &self,
        account_id: &str,
        request: &CreateAchRelationshipRequest,
    ) -> Result<AchRelationship> {
        self.post(
            &format!("/v1/accounts/{}/ach_relationships", account_id),
            request,
        )
        .await
    }

    /// List ACH relationships for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// List of ACH relationships
    pub async fn list_ach_relationships(&self, account_id: &str) -> Result<Vec<AchRelationship>> {
        self.get(&format!("/v1/accounts/{}/ach_relationships", account_id))
            .await
    }

    /// Delete an ACH relationship.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `relationship_id` - The relationship ID to delete
    pub async fn delete_ach_relationship(
        &self,
        account_id: &str,
        relationship_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v1/accounts/{}/ach_relationships/{}",
            account_id, relationship_id
        ))
        .await
    }

    // ========================================================================
    // Transfer Endpoints
    // ========================================================================

    /// Create a transfer for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `request` - Transfer creation request
    ///
    /// # Returns
    /// The created transfer
    pub async fn create_transfer(
        &self,
        account_id: &str,
        request: &CreateTransferRequest,
    ) -> Result<Transfer> {
        self.post(&format!("/v1/accounts/{}/transfers", account_id), request)
            .await
    }

    /// List transfers for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// List of transfers
    pub async fn list_transfers(
        &self,
        account_id: &str,
        params: &ListTransfersParams,
    ) -> Result<Vec<Transfer>> {
        self.get_with_params(&format!("/v1/accounts/{}/transfers", account_id), params)
            .await
    }

    /// Get a specific transfer.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `transfer_id` - The transfer ID
    ///
    /// # Returns
    /// The transfer
    pub async fn get_transfer(&self, account_id: &str, transfer_id: &str) -> Result<Transfer> {
        self.get(&format!(
            "/v1/accounts/{}/transfers/{}",
            account_id, transfer_id
        ))
        .await
    }

    /// Cancel a transfer.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `transfer_id` - The transfer ID to cancel
    pub async fn cancel_transfer(&self, account_id: &str, transfer_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/accounts/{}/transfers/{}",
            account_id, transfer_id
        ))
        .await
    }

    // ========================================================================
    // Wire Bank Endpoints
    // ========================================================================

    /// List recipient banks for wire transfers.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// List of wire banks
    pub async fn list_wire_banks(&self, account_id: &str) -> Result<Vec<WireBank>> {
        self.get(&format!("/v1/accounts/{}/recipient_banks", account_id))
            .await
    }

    /// Create a recipient bank for wire transfers.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `request` - Wire bank creation request
    ///
    /// # Returns
    /// The created wire bank
    pub async fn create_wire_bank(
        &self,
        account_id: &str,
        request: &CreateWireBankRequest,
    ) -> Result<WireBank> {
        self.post(
            &format!("/v1/accounts/{}/recipient_banks", account_id),
            request,
        )
        .await
    }

    /// Delete a recipient bank.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `bank_id` - The bank ID to delete
    pub async fn delete_wire_bank(&self, account_id: &str, bank_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/accounts/{}/recipient_banks/{}",
            account_id, bank_id
        ))
        .await
    }

    // ========================================================================
    // Journal Endpoints
    // ========================================================================

    /// Create a journal entry.
    ///
    /// # Arguments
    /// * `request` - Journal creation request
    ///
    /// # Returns
    /// The created journal
    pub async fn create_journal(&self, request: &CreateJournalRequest) -> Result<Journal> {
        self.post("/v1/journals", request).await
    }

    /// List journal entries.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// List of journals
    pub async fn list_journals(&self, params: &ListJournalsParams) -> Result<Vec<Journal>> {
        self.get_with_params("/v1/journals", params).await
    }

    /// Create batch journal entries.
    ///
    /// # Arguments
    /// * `request` - Batch journal creation request
    ///
    /// # Returns
    /// List of created journals
    pub async fn create_batch_journals(
        &self,
        request: &CreateBatchJournalRequest,
    ) -> Result<Vec<Journal>> {
        self.post("/v1/journals/batch", request).await
    }

    /// Delete a journal entry.
    ///
    /// # Arguments
    /// * `journal_id` - The journal ID to delete
    pub async fn delete_journal(&self, journal_id: &str) -> Result<()> {
        self.delete(&format!("/v1/journals/{}", journal_id)).await
    }
}

// ============================================================================
// Enhanced Crypto Trading Endpoints
// ============================================================================

/// Response for crypto snapshots (multi-symbol).
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiCryptoSnapshotsResponse {
    /// Map of symbol to snapshot.
    pub snapshots: std::collections::HashMap<String, CryptoSnapshot>,
}

/// Response for multi-symbol crypto bars.
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiCryptoBarsResponse {
    /// Map of symbol to bars.
    pub bars: std::collections::HashMap<String, Vec<CryptoBar>>,
    /// Next page token.
    pub next_page_token: Option<String>,
}

/// Response for latest crypto bars (multi-symbol).
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestCryptoBarsResponse {
    /// Map of symbol to latest bar.
    pub bars: std::collections::HashMap<String, CryptoBar>,
}

/// Response for latest crypto quotes (multi-symbol).
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestCryptoQuotesResponse {
    /// Map of symbol to latest quote.
    pub quotes: std::collections::HashMap<String, CryptoQuote>,
}

/// Response for latest crypto trades (multi-symbol).
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestCryptoTradesResponse {
    /// Map of symbol to latest trade.
    pub trades: std::collections::HashMap<String, CryptoTrade>,
}

/// Response for crypto orderbooks.
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoOrderbooksResponse {
    /// Map of symbol to orderbook.
    pub orderbooks: std::collections::HashMap<String, CryptoOrderbook>,
}

impl AlpacaHttpClient {
    // ========================================================================
    // Crypto Wallet Endpoints (Broker API)
    // ========================================================================

    /// List crypto wallets for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// List of crypto wallets
    pub async fn list_crypto_wallets(&self, account_id: &str) -> Result<Vec<BrokerCryptoWallet>> {
        self.get(&format!("/v1/accounts/{}/wallets", account_id))
            .await
    }

    /// Create a crypto wallet for an account.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `request` - Wallet creation request
    ///
    /// # Returns
    /// The created wallet
    pub async fn create_crypto_wallet(
        &self,
        account_id: &str,
        request: &CreateCryptoWalletRequest,
    ) -> Result<BrokerCryptoWallet> {
        self.post(&format!("/v1/accounts/{}/wallets", account_id), request)
            .await
    }

    /// Get a crypto wallet by asset.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `asset` - The asset symbol (e.g., BTC, ETH)
    ///
    /// # Returns
    /// The crypto wallet
    pub async fn get_crypto_wallet(
        &self,
        account_id: &str,
        asset: &str,
    ) -> Result<BrokerCryptoWallet> {
        self.get(&format!("/v1/accounts/{}/wallets/{}", account_id, asset))
            .await
    }

    /// List crypto wallet transfers.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// List of crypto transfers
    pub async fn list_crypto_transfers(&self, account_id: &str) -> Result<Vec<CryptoTransfer>> {
        self.get(&format!("/v1/accounts/{}/wallets/transfers", account_id))
            .await
    }

    /// Create a crypto wallet transfer.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `asset` - The asset symbol
    /// * `request` - Transfer request
    ///
    /// # Returns
    /// The created transfer
    pub async fn create_crypto_transfer(
        &self,
        account_id: &str,
        asset: &str,
        request: &CreateCryptoTransferRequest,
    ) -> Result<CryptoTransfer> {
        self.post(
            &format!("/v1/accounts/{}/wallets/{}/transfers", account_id, asset),
            request,
        )
        .await
    }

    /// List whitelisted crypto addresses.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    ///
    /// # Returns
    /// List of whitelisted addresses
    pub async fn list_crypto_whitelists(
        &self,
        account_id: &str,
    ) -> Result<Vec<CryptoWhitelistAddress>> {
        self.get(&format!("/v1/accounts/{}/wallets/whitelists", account_id))
            .await
    }

    /// Add a whitelisted crypto address.
    ///
    /// # Arguments
    /// * `account_id` - The account ID
    /// * `request` - Whitelist request
    ///
    /// # Returns
    /// The created whitelist entry
    pub async fn create_crypto_whitelist(
        &self,
        account_id: &str,
        request: &CreateCryptoWhitelistRequest,
    ) -> Result<CryptoWhitelistAddress> {
        self.post(
            &format!("/v1/accounts/{}/wallets/whitelists", account_id),
            request,
        )
        .await
    }

    // ========================================================================
    // Enhanced Crypto Market Data Endpoints
    // ========================================================================

    /// Get multi-symbol crypto bars.
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Crypto bar data for multiple symbols
    pub async fn get_multi_crypto_bars(
        &self,
        params: &CryptoBarsParams,
    ) -> Result<MultiCryptoBarsResponse> {
        self.get_with_params("/v1beta3/crypto/us/bars", params)
            .await
    }

    /// Get latest crypto bars.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Latest bar for each symbol
    pub async fn get_latest_crypto_bars(&self, symbols: &str) -> Result<LatestCryptoBarsResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v1beta3/crypto/us/latest/bars", &Params { symbols })
            .await
    }

    /// Get latest crypto quotes.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Latest quote for each symbol
    pub async fn get_latest_crypto_quotes(
        &self,
        symbols: &str,
    ) -> Result<LatestCryptoQuotesResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v1beta3/crypto/us/latest/quotes", &Params { symbols })
            .await
    }

    /// Get latest crypto trades.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Latest trade for each symbol
    pub async fn get_latest_crypto_trades(
        &self,
        symbols: &str,
    ) -> Result<LatestCryptoTradesResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v1beta3/crypto/us/latest/trades", &Params { symbols })
            .await
    }

    /// Get crypto snapshots for multiple symbols.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Snapshots for each symbol
    pub async fn get_multi_crypto_snapshots(
        &self,
        symbols: &str,
    ) -> Result<MultiCryptoSnapshotsResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v1beta3/crypto/us/snapshots", &Params { symbols })
            .await
    }

    /// Get crypto orderbooks.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    ///
    /// # Returns
    /// Orderbooks for each symbol
    pub async fn get_crypto_orderbooks(&self, symbols: &str) -> Result<CryptoOrderbooksResponse> {
        #[derive(Serialize)]
        struct Params<'a> {
            symbols: &'a str,
        }
        self.get_with_params("/v1beta3/crypto/us/latest/orderbooks", &Params { symbols })
            .await
    }
}

// ============================================================================
// News API Endpoints
// ============================================================================

/// Response for enhanced news request.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedNewsResponse {
    /// News articles.
    pub news: Vec<EnhancedNewsArticle>,
    /// Next page token.
    pub next_page_token: Option<String>,
}

impl AlpacaHttpClient {
    /// Get enhanced news articles with images and full content.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering news
    ///
    /// # Returns
    /// List of enhanced news articles with pagination
    pub async fn get_enhanced_news(
        &self,
        params: &alpaca_base::NewsParams,
    ) -> Result<EnhancedNewsResponse> {
        self.get_with_params("/v1beta1/news", params).await
    }

    /// Get enhanced news for specific symbols.
    ///
    /// # Arguments
    /// * `symbols` - Comma-separated list of symbols
    /// * `limit` - Maximum number of articles
    ///
    /// # Returns
    /// List of enhanced news articles
    pub async fn get_enhanced_news_for_symbols(
        &self,
        symbols: &str,
        limit: u32,
    ) -> Result<EnhancedNewsResponse> {
        let params = alpaca_base::NewsParams::new().symbols(symbols).limit(limit);
        self.get_enhanced_news(&params).await
    }

    /// Get latest enhanced news.
    ///
    /// # Arguments
    /// * `limit` - Maximum number of articles
    ///
    /// # Returns
    /// List of latest enhanced news articles
    pub async fn get_latest_enhanced_news(&self, limit: u32) -> Result<EnhancedNewsResponse> {
        let params = alpaca_base::NewsParams::new().sort_desc().limit(limit);
        self.get_enhanced_news(&params).await
    }
}

// ============================================================================
// OAuth 2.0 Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// Exchange authorization code for OAuth token.
    ///
    /// # Arguments
    /// * `request` - Token exchange request
    ///
    /// # Returns
    /// OAuth token
    pub async fn oauth_exchange_code(&self, request: &OAuthTokenRequest) -> Result<OAuthToken> {
        self.post("/oauth/token", request).await
    }

    /// Refresh OAuth token.
    ///
    /// # Arguments
    /// * `request` - Token refresh request
    ///
    /// # Returns
    /// New OAuth token
    pub async fn oauth_refresh_token(&self, request: &OAuthTokenRequest) -> Result<OAuthToken> {
        self.post("/oauth/token", request).await
    }

    /// Revoke OAuth token.
    ///
    /// # Arguments
    /// * `request` - Token revoke request
    pub async fn oauth_revoke_token(&self, request: &OAuthRevokeRequest) -> Result<()> {
        let _: serde_json::Value = self.post("/oauth/revoke", request).await?;
        Ok(())
    }
}

// ============================================================================
// Broker API Events (SSE) Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// Get account status events (SSE endpoint URL).
    ///
    /// Note: This returns the URL for SSE streaming. Use an SSE client to connect.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// SSE endpoint URL
    #[must_use]
    pub fn get_account_status_events_url(&self, params: &SseEventParams) -> String {
        let base = "/v1/events/accounts/status";
        self.build_sse_url(base, params)
    }

    /// Get transfer status events (SSE endpoint URL).
    ///
    /// Note: This returns the URL for SSE streaming. Use an SSE client to connect.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// SSE endpoint URL
    #[must_use]
    pub fn get_transfer_status_events_url(&self, params: &SseEventParams) -> String {
        let base = "/v1/events/transfers/status";
        self.build_sse_url(base, params)
    }

    /// Get trade events (SSE endpoint URL).
    ///
    /// Note: This returns the URL for SSE streaming. Use an SSE client to connect.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// SSE endpoint URL
    #[must_use]
    pub fn get_trade_events_url(&self, params: &SseEventParams) -> String {
        let base = "/v1/events/trades";
        self.build_sse_url(base, params)
    }

    /// Get journal status events (SSE endpoint URL).
    ///
    /// Note: This returns the URL for SSE streaming. Use an SSE client to connect.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// SSE endpoint URL
    #[must_use]
    pub fn get_journal_status_events_url(&self, params: &SseEventParams) -> String {
        let base = "/v1/events/journals/status";
        self.build_sse_url(base, params)
    }

    /// Get non-trade activity events (SSE endpoint URL).
    ///
    /// Note: This returns the URL for SSE streaming. Use an SSE client to connect.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// SSE endpoint URL
    #[must_use]
    pub fn get_nta_events_url(&self, params: &SseEventParams) -> String {
        let base = "/v2beta1/events/nta";
        self.build_sse_url(base, params)
    }

    /// Build SSE URL with query parameters.
    fn build_sse_url(&self, base: &str, params: &SseEventParams) -> String {
        let mut url = base.to_string();
        let mut query_parts = Vec::new();

        if let Some(ref account_id) = params.account_id {
            query_parts.push(format!("account_id={}", account_id));
        }
        if let Some(ref since) = params.since {
            query_parts.push(format!("since={}", since));
        }
        if let Some(ref until) = params.until {
            query_parts.push(format!("until={}", until));
        }

        if !query_parts.is_empty() {
            url.push('?');
            url.push_str(&query_parts.join("&"));
        }

        url
    }
}

// ============================================================================
// Enhanced Assets API Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// List enhanced assets with filtering.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    /// List of enhanced assets
    pub async fn list_enhanced_assets(
        &self,
        params: &ListAssetsParams,
    ) -> Result<Vec<EnhancedAsset>> {
        self.get_with_params("/v2/assets", params).await
    }

    /// Get enhanced asset by symbol or ID.
    ///
    /// # Arguments
    /// * `symbol_or_id` - Symbol or asset ID
    ///
    /// # Returns
    /// Enhanced asset details
    pub async fn get_enhanced_asset(&self, symbol_or_id: &str) -> Result<EnhancedAsset> {
        self.get(&format!("/v2/assets/{}", symbol_or_id)).await
    }

    /// Get options contracts for an underlying symbol.
    ///
    /// # Arguments
    /// * `symbol` - Underlying symbol
    ///
    /// # Returns
    /// List of option contract assets
    pub async fn get_asset_options(&self, symbol: &str) -> Result<Vec<OptionContractAsset>> {
        self.get(&format!("/v2/assets/{}/options", symbol)).await
    }

    /// List corporate action announcements.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    /// List of corporate action announcements
    pub async fn list_announcements(
        &self,
        params: &ListAnnouncementsParams,
    ) -> Result<Vec<CorporateActionAnnouncement>> {
        self.get_with_params("/v1beta1/corporate-actions/announcements", params)
            .await
    }

    /// Get a specific corporate action announcement.
    ///
    /// # Arguments
    /// * `announcement_id` - Announcement ID
    ///
    /// # Returns
    /// Corporate action announcement
    pub async fn get_announcement(
        &self,
        announcement_id: &str,
    ) -> Result<CorporateActionAnnouncement> {
        self.get(&format!(
            "/v1beta1/corporate-actions/announcements/{}",
            announcement_id
        ))
        .await
    }
}

// ============================================================================
// Enhanced Account Activities Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// List account activities with filtering.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    /// List of account activities
    pub async fn list_activities(
        &self,
        params: &ListActivitiesParams,
    ) -> Result<Vec<AccountActivity>> {
        self.get_with_params("/v2/account/activities", params).await
    }

    /// List account activities by type.
    ///
    /// # Arguments
    /// * `activity_type` - Activity type to filter by
    /// * `params` - Additional query parameters
    ///
    /// # Returns
    /// List of account activities
    pub async fn list_activities_by_type(
        &self,
        activity_type: &str,
        params: &ListActivitiesParams,
    ) -> Result<Vec<AccountActivity>> {
        self.get_with_params(&format!("/v2/account/activities/{}", activity_type), params)
            .await
    }

    /// List all broker accounts activities.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    /// List of account activities across all accounts
    pub async fn list_broker_activities(
        &self,
        params: &ListActivitiesParams,
    ) -> Result<Vec<AccountActivity>> {
        self.get_with_params("/v1/accounts/activities", params)
            .await
    }

    /// List activities for a specific broker account.
    ///
    /// # Arguments
    /// * `account_id` - Account ID
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    /// List of account activities
    pub async fn list_broker_account_activities(
        &self,
        account_id: &str,
        params: &ListActivitiesParams,
    ) -> Result<Vec<AccountActivity>> {
        self.get_with_params(&format!("/v1/accounts/{}/activities", account_id), params)
            .await
    }
}

// ============================================================================
// Portfolio Management Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// Create rebalancing portfolio.
    ///
    /// # Arguments
    /// * `request` - Portfolio creation request
    ///
    /// # Returns
    /// Created portfolio
    pub async fn create_rebalance_portfolio(
        &self,
        request: &RebalancePortfolioRequest,
    ) -> Result<RebalancePortfolio> {
        self.post("/v1/rebalancing/portfolios", request).await
    }

    /// List rebalancing portfolios.
    ///
    /// # Returns
    /// List of portfolios
    pub async fn list_rebalance_portfolios(&self) -> Result<Vec<RebalancePortfolio>> {
        self.get("/v1/rebalancing/portfolios").await
    }

    /// Get rebalancing portfolio.
    ///
    /// # Arguments
    /// * `portfolio_id` - Portfolio ID
    ///
    /// # Returns
    /// Portfolio
    pub async fn get_rebalance_portfolio(&self, portfolio_id: &str) -> Result<RebalancePortfolio> {
        self.get(&format!("/v1/rebalancing/portfolios/{}", portfolio_id))
            .await
    }

    /// Delete rebalancing portfolio.
    ///
    /// # Arguments
    /// * `portfolio_id` - Portfolio ID
    pub async fn delete_rebalance_portfolio(&self, portfolio_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .delete(&format!("/v1/rebalancing/portfolios/{}", portfolio_id))
            .await?;
        Ok(())
    }

    /// Execute rebalance run.
    ///
    /// # Arguments
    /// * `request` - Rebalance run request
    ///
    /// # Returns
    /// Rebalance run
    pub async fn execute_rebalance(&self, request: &RebalanceRunRequest) -> Result<RebalanceRun> {
        self.post("/v1/rebalancing/runs", request).await
    }

    /// List rebalance runs.
    ///
    /// # Returns
    /// List of rebalance runs
    pub async fn list_rebalance_runs(&self) -> Result<Vec<RebalanceRun>> {
        self.get("/v1/rebalancing/runs").await
    }
}

// ============================================================================
// Margin and Short Selling Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// Get locate availability for short selling.
    ///
    /// # Returns
    /// List of available locates
    pub async fn get_locates(&self) -> Result<Vec<LocateResponse>> {
        self.get("/v1/locate/stocks").await
    }

    /// Request a locate for short selling.
    ///
    /// # Arguments
    /// * `request` - Locate request
    ///
    /// # Returns
    /// Locate response
    pub async fn request_locate(&self, request: &LocateRequest) -> Result<LocateResponse> {
        self.post("/v1/locate/stocks", request).await
    }
}

// ============================================================================
// Paper Trading Endpoints
// ============================================================================

impl AlpacaHttpClient {
    /// Reset paper trading account.
    ///
    /// # Returns
    /// The reset account
    pub async fn reset_paper_account(&self) -> Result<Account> {
        self.post("/v2/account/reset", &serde_json::json!({})).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_request_market() {
        let order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "10");
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.qty, Some("10".to_string()));
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.time_in_force, TimeInForce::Day);
    }

    #[test]
    fn test_create_order_request_limit() {
        let order = CreateOrderRequest::limit("AAPL", OrderSide::Buy, "10", "150.00");
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.limit_price, Some("150.00".to_string()));
        assert_eq!(order.order_type, OrderType::Limit);
    }

    #[test]
    fn test_create_order_request_stop() {
        let order = CreateOrderRequest::stop("AAPL", OrderSide::Sell, "10", "145.00");
        assert_eq!(order.stop_price, Some("145.00".to_string()));
        assert_eq!(order.order_type, OrderType::Stop);
    }

    #[test]
    fn test_create_order_request_stop_limit() {
        let order =
            CreateOrderRequest::stop_limit("AAPL", OrderSide::Sell, "10", "145.00", "144.50");
        assert_eq!(order.stop_price, Some("145.00".to_string()));
        assert_eq!(order.limit_price, Some("144.50".to_string()));
        assert_eq!(order.order_type, OrderType::StopLimit);
    }

    #[test]
    fn test_create_order_request_trailing_stop_price() {
        let order = CreateOrderRequest::trailing_stop_price("AAPL", OrderSide::Sell, "10", "5.00");
        assert_eq!(order.trail_price, Some("5.00".to_string()));
        assert_eq!(order.order_type, OrderType::TrailingStop);
    }

    #[test]
    fn test_create_order_request_trailing_stop_percent() {
        let order = CreateOrderRequest::trailing_stop_percent("AAPL", OrderSide::Sell, "10", "2.5");
        assert_eq!(order.trail_percent, Some("2.5".to_string()));
        assert_eq!(order.order_type, OrderType::TrailingStop);
    }

    #[test]
    fn test_create_order_request_bracket() {
        let tp = TakeProfit::new("160.00");
        let sl = StopLoss::new("140.00");
        let order =
            CreateOrderRequest::bracket("AAPL", OrderSide::Buy, "10", OrderType::Market, tp, sl);

        assert_eq!(order.order_class, Some(OrderClass::Bracket));
        assert!(order.take_profit.is_some());
        assert!(order.stop_loss.is_some());
        assert_eq!(order.take_profit.unwrap().limit_price, "160.00");
        assert_eq!(order.stop_loss.unwrap().stop_price, "140.00");
    }

    #[test]
    fn test_create_order_request_oco() {
        let tp = TakeProfit::new("160.00");
        let sl = StopLoss::with_limit("140.00", "139.50");
        let order = CreateOrderRequest::oco("AAPL", OrderSide::Sell, "10", tp, sl);

        assert_eq!(order.order_class, Some(OrderClass::Oco));
        assert!(order.take_profit.is_some());
        assert!(order.stop_loss.is_some());
    }

    #[test]
    fn test_create_order_request_oto() {
        let sl = StopLoss::new("140.00");
        let order = CreateOrderRequest::oto("AAPL", OrderSide::Buy, "10", OrderType::Limit, sl);

        assert_eq!(order.order_class, Some(OrderClass::Oto));
        assert!(order.stop_loss.is_some());
    }

    #[test]
    fn test_create_order_request_builder_methods() {
        let order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "10")
            .time_in_force(TimeInForce::Gtc)
            .extended_hours(true)
            .client_order_id("my-order-123");

        assert_eq!(order.time_in_force, TimeInForce::Gtc);
        assert_eq!(order.extended_hours, Some(true));
        assert_eq!(order.client_order_id, Some("my-order-123".to_string()));
    }

    #[test]
    fn test_create_order_request_position_intent() {
        let order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "10")
            .position_intent(PositionIntent::BuyToOpen);

        assert_eq!(order.position_intent, Some(PositionIntent::BuyToOpen));
    }

    #[test]
    fn test_replace_order_request_builder() {
        let request = ReplaceOrderRequest::new()
            .qty("20")
            .limit_price("155.00")
            .time_in_force(TimeInForce::Gtc);

        assert_eq!(request.qty, Some("20".to_string()));
        assert_eq!(request.limit_price, Some("155.00".to_string()));
        assert_eq!(request.time_in_force, Some(TimeInForce::Gtc));
    }

    #[test]
    fn test_order_params_builder() {
        let params = OrderParams::new()
            .status(OrderQueryStatus::Open)
            .limit(100)
            .nested(true)
            .symbols("AAPL,GOOGL")
            .side(OrderSide::Buy)
            .direction(SortDirection::Desc);

        assert_eq!(params.status, Some(OrderQueryStatus::Open));
        assert_eq!(params.limit, Some(100));
        assert_eq!(params.nested, Some(true));
        assert_eq!(params.symbols, Some("AAPL,GOOGL".to_string()));
        assert_eq!(params.side, Some(OrderSide::Buy));
        assert_eq!(params.direction, Some(SortDirection::Desc));
    }

    #[test]
    fn test_create_order_request_serialization() {
        let order = CreateOrderRequest::limit("AAPL", OrderSide::Buy, "10", "150.00")
            .client_order_id("test-123");

        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("\"symbol\":\"AAPL\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"type\":\"limit\""));
        assert!(json.contains("\"limit_price\":\"150.00\""));
    }

    #[test]
    fn test_bracket_order_serialization() {
        let tp = TakeProfit::new("160.00");
        let sl = StopLoss::with_limit("140.00", "139.50");
        let order =
            CreateOrderRequest::bracket("AAPL", OrderSide::Buy, "10", OrderType::Limit, tp, sl)
                .with_limit_price("150.00");

        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("\"order_class\":\"bracket\""));
        assert!(json.contains("\"take_profit\""));
        assert!(json.contains("\"stop_loss\""));
    }
}
