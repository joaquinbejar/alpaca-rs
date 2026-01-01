//! Test utilities and fixtures for Alpaca API testing.
//!
//! This module provides helper functions, fixtures, and builders
//! for creating test data in unit and integration tests.

use crate::types::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Test fixtures for common Alpaca types.
pub mod fixtures {
    use super::*;

    /// Creates a sample Account for testing.
    #[must_use]
    pub fn sample_account() -> Account {
        Account {
            id: Uuid::new_v4(),
            account_number: "PA1234567890".to_string(),
            status: AccountStatus::Active,
            currency: "USD".to_string(),
            buying_power: "100000.00".to_string(),
            regt_buying_power: "100000.00".to_string(),
            daytrading_buying_power: "400000.00".to_string(),
            cash: "100000.00".to_string(),
            portfolio_value: "100000.00".to_string(),
            pattern_day_trader: false,
            trading_blocked: false,
            transfers_blocked: false,
            account_blocked: false,
            created_at: Utc::now(),
            trade_suspended_by_user: false,
            multiplier: "4".to_string(),
            shorting_enabled: true,
            equity: "100000.00".to_string(),
            last_equity: "99500.00".to_string(),
            long_market_value: "0.00".to_string(),
            short_market_value: "0.00".to_string(),
            initial_margin: "0.00".to_string(),
            maintenance_margin: "0.00".to_string(),
            last_maintenance_margin: "0.00".to_string(),
            sma: "0.00".to_string(),
            daytrade_count: 0,
        }
    }

    /// Creates a sample Asset for testing.
    #[must_use]
    pub fn sample_asset(symbol: &str) -> Asset {
        Asset {
            id: Uuid::new_v4(),
            class: AssetClass::UsEquity,
            exchange: "NASDAQ".to_string(),
            symbol: symbol.to_string(),
            name: format!("{} Inc.", symbol),
            status: AssetStatus::Active,
            tradable: true,
            marginable: true,
            shortable: true,
            easy_to_borrow: true,
            fractionable: true,
        }
    }

    /// Creates a sample Order for testing.
    #[must_use]
    pub fn sample_order(symbol: &str, side: OrderSide, qty: &str) -> Order {
        Order {
            id: Uuid::new_v4(),
            client_order_id: format!("test-order-{}", Uuid::new_v4()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            submitted_at: Some(Utc::now()),
            filled_at: None,
            expired_at: None,
            canceled_at: None,
            failed_at: None,
            replaced_at: None,
            replaced_by: None,
            replaces: None,
            asset_id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            asset_class: AssetClass::UsEquity,
            notional: None,
            qty: Some(qty.to_string()),
            filled_qty: "0".to_string(),
            filled_avg_price: None,
            order_class: OrderClass::Simple,
            order_type: OrderType::Market,
            side,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: None,
            status: OrderStatus::New,
            extended_hours: false,
            legs: None,
            trail_percent: None,
            trail_price: None,
            hwm: None,
        }
    }

    /// Creates a sample Position for testing.
    #[must_use]
    pub fn sample_position(symbol: &str, qty: &str, avg_price: &str) -> Position {
        Position {
            asset_id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            exchange: "NASDAQ".to_string(),
            asset_class: AssetClass::UsEquity,
            avg_entry_price: avg_price.to_string(),
            qty: qty.to_string(),
            side: PositionSide::Long,
            market_value: "15000.00".to_string(),
            cost_basis: "14500.00".to_string(),
            unrealized_pl: "500.00".to_string(),
            unrealized_plpc: "0.0345".to_string(),
            unrealized_intraday_pl: "100.00".to_string(),
            unrealized_intraday_plpc: "0.0067".to_string(),
            current_price: "150.00".to_string(),
            lastday_price: "149.00".to_string(),
            change_today: "0.0067".to_string(),
        }
    }

    /// Creates a sample Bar for testing.
    #[must_use]
    pub fn sample_bar(timestamp: DateTime<Utc>) -> Bar {
        Bar {
            timestamp,
            open: 150.00,
            high: 152.50,
            low: 149.25,
            close: 151.75,
            volume: 1_000_000,
            trade_count: Some(5000),
            vwap: Some(151.00),
        }
    }

    /// Creates a sample Quote for testing.
    #[must_use]
    pub fn sample_quote(timestamp: DateTime<Utc>) -> Quote {
        Quote {
            timestamp,
            timeframe: "1Min".to_string(),
            bid_price: 150.00,
            bid_size: 100,
            ask_price: 150.05,
            ask_size: 200,
            bid_exchange: "Q".to_string(),
            ask_exchange: "Q".to_string(),
        }
    }

    /// Creates a sample Trade for testing.
    #[must_use]
    pub fn sample_trade(timestamp: DateTime<Utc>) -> Trade {
        Trade {
            timestamp,
            price: 150.02,
            size: 50,
            exchange: "Q".to_string(),
            conditions: vec!["@".to_string()],
            id: 123456789,
        }
    }

    /// Creates a sample Clock for testing.
    #[must_use]
    pub fn sample_clock(is_open: bool) -> Clock {
        let now = Utc::now();
        Clock {
            timestamp: now,
            is_open,
            next_open: now + chrono::Duration::hours(if is_open { 24 } else { 1 }),
            next_close: now + chrono::Duration::hours(if is_open { 6 } else { 24 }),
        }
    }
}

/// Assertion helpers for testing.
pub mod assertions {
    use super::*;

    /// Asserts that an order has the expected basic properties.
    pub fn assert_order_basics(order: &Order, symbol: &str, side: OrderSide, order_type: OrderType) {
        assert_eq!(order.symbol, symbol);
        assert_eq!(order.side, side);
        assert_eq!(order.order_type, order_type);
    }

    /// Asserts that a position has the expected symbol and side.
    pub fn assert_position_basics(position: &Position, symbol: &str, side: PositionSide) {
        assert_eq!(position.symbol, symbol);
        assert_eq!(position.side, side);
    }

    /// Asserts that an account is active and not blocked.
    pub fn assert_account_active(account: &Account) {
        assert_eq!(account.status, AccountStatus::Active);
        assert!(!account.trading_blocked);
        assert!(!account.account_blocked);
    }
}

/// JSON test data for deserialization tests.
pub mod json_samples {
    /// Sample account JSON response.
    pub const ACCOUNT_JSON: &str = r#"{
        "id": "904837e3-3b76-47ec-b432-046db621571b",
        "account_number": "PA1234567890",
        "status": "ACTIVE",
        "currency": "USD",
        "buying_power": "100000.00",
        "regt_buying_power": "100000.00",
        "daytrading_buying_power": "400000.00",
        "cash": "100000.00",
        "portfolio_value": "100000.00",
        "pattern_day_trader": false,
        "trading_blocked": false,
        "transfers_blocked": false,
        "account_blocked": false,
        "created_at": "2021-01-01T00:00:00Z",
        "trade_suspended_by_user": false,
        "multiplier": "4",
        "shorting_enabled": true,
        "equity": "100000.00",
        "last_equity": "99500.00",
        "long_market_value": "0.00",
        "short_market_value": "0.00",
        "initial_margin": "0.00",
        "maintenance_margin": "0.00",
        "last_maintenance_margin": "0.00",
        "sma": "0.00",
        "daytrade_count": 0
    }"#;

    /// Sample order JSON response.
    pub const ORDER_JSON: &str = r#"{
        "id": "904837e3-3b76-47ec-b432-046db621571b",
        "client_order_id": "test-order-123",
        "created_at": "2021-01-01T10:00:00Z",
        "updated_at": "2021-01-01T10:00:00Z",
        "submitted_at": "2021-01-01T10:00:00Z",
        "filled_at": null,
        "expired_at": null,
        "canceled_at": null,
        "failed_at": null,
        "replaced_at": null,
        "replaced_by": null,
        "replaces": null,
        "asset_id": "904837e3-3b76-47ec-b432-046db621571c",
        "symbol": "AAPL",
        "asset_class": "us_equity",
        "notional": null,
        "qty": "10",
        "filled_qty": "0",
        "filled_avg_price": null,
        "order_class": "simple",
        "order_type": "market",
        "side": "buy",
        "time_in_force": "day",
        "limit_price": null,
        "stop_price": null,
        "status": "new",
        "extended_hours": false,
        "legs": null,
        "trail_percent": null,
        "trail_price": null,
        "hwm": null
    }"#;

    /// Sample asset JSON response.
    pub const ASSET_JSON: &str = r#"{
        "id": "904837e3-3b76-47ec-b432-046db621571b",
        "class": "us_equity",
        "exchange": "NASDAQ",
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true
    }"#;

    /// Sample error JSON response.
    pub const ERROR_JSON: &str = r#"{
        "code": 40410000,
        "message": "order not found"
    }"#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_account() {
        let account = fixtures::sample_account();
        assert_eq!(account.status, AccountStatus::Active);
        assert!(!account.trading_blocked);
    }

    #[test]
    fn test_sample_asset() {
        let asset = fixtures::sample_asset("AAPL");
        assert_eq!(asset.symbol, "AAPL");
        assert!(asset.tradable);
    }

    #[test]
    fn test_sample_order() {
        let order = fixtures::sample_order("AAPL", OrderSide::Buy, "10");
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, OrderSide::Buy);
    }

    #[test]
    fn test_sample_position() {
        let position = fixtures::sample_position("AAPL", "100", "145.00");
        assert_eq!(position.symbol, "AAPL");
        assert_eq!(position.qty, "100");
    }

    #[test]
    fn test_account_json_deserialization() {
        let account: Account = serde_json::from_str(json_samples::ACCOUNT_JSON).unwrap();
        assert_eq!(account.account_number, "PA1234567890");
        assert_eq!(account.status, AccountStatus::Active);
    }

    #[test]
    fn test_order_json_deserialization() {
        let order: Order = serde_json::from_str(json_samples::ORDER_JSON).unwrap();
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Market);
    }

    #[test]
    fn test_asset_json_deserialization() {
        let asset: Asset = serde_json::from_str(json_samples::ASSET_JSON).unwrap();
        assert_eq!(asset.symbol, "AAPL");
        assert_eq!(asset.class, AssetClass::UsEquity);
    }

    #[test]
    fn test_assertion_helpers() {
        let order = fixtures::sample_order("AAPL", OrderSide::Buy, "10");
        assertions::assert_order_basics(&order, "AAPL", OrderSide::Buy, OrderType::Market);

        let account = fixtures::sample_account();
        assertions::assert_account_active(&account);
    }
}
