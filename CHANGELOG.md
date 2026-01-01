# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-01-01

### Added
- Testing infrastructure with test utilities module (`test_utils`)
- Test fixtures for Account, Asset, Order, Position, Bar, Quote, Trade, Clock
- Assertion helpers for common test patterns
- JSON sample data for deserialization tests
- `test-utils` feature flag for downstream crate usage
- GitHub Actions CI/CD workflow with coverage reporting

## [0.3.0] - 2026-01-01

### Added
- Comprehensive error handling with `ApiErrorCode` enum
- `ApiErrorResponse` struct for structured error parsing
- `RateLimitInfo` struct with rate limit header parsing
- `ValidationError` struct for field-level validation errors
- Helper methods on `AlpacaError`: `is_retryable()`, `retry_after()`, `request_id()`, `status_code()`

### Changed
- Enhanced HTTP client to parse structured error responses
- HTTP client now extracts request ID from headers
- HTTP client now parses rate limit headers

## [0.2.0] - 2026-01-01

### Added
- Advanced order types support (bracket, OCO, OTO)
- `PositionIntent` enum for options orders
- `TakeProfit` and `StopLoss` structs with helper constructors
- `Gtd` (Good Till Date) to `TimeInForce` enum
- `SortDirection` and `OrderQueryStatus` enums
- Builder patterns for `CreateOrderRequest`, `ReplaceOrderRequest`, `OrderParams`
- Factory methods for order creation: `market()`, `limit()`, `stop()`, `bracket()`, etc.
- Example: `trading_bracket_order.rs`

### Changed
- Added `Default` implementations for `OrderSide`, `OrderType`, `TimeInForce`

## [0.1.0] - 2026-01-01

### Added
- Initial release
- Core types: Account, Asset, Order, Position, Watchlist, Calendar, Clock
- Market data types: Bar, Quote, Trade
- HTTP client for Trading API and Market Data API
- WebSocket client for real-time streaming
- Authentication with API keys
- Paper and live trading environment support
