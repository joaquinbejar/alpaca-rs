//! HTTP API endpoints for Alpaca trading platform.
//!
//! This module provides implementations for all Alpaca REST API endpoints
//! including account management, order operations, position management,
//! market data, and more.

use crate::client::AlpacaHttpClient;
use alpaca_base::{Result, types::*};
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
