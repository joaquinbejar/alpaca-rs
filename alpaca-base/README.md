# alpaca-base

Base library with common structs, traits, and logic for Alpaca API clients.

## Overview

`alpaca-base` provides the foundational building blocks for the Alpaca Market API Rust clients. It includes shared data models, authentication utilities, custom error types, and common helper functions used by both `alpaca-http` and `alpaca-websocket`.

## Features

- **Core Data Models**: Comprehensive Rust representations of Alpaca API objects (Orders, Positions, Assets, etc.).
- **Authentication**: Utilities for managing API keys and generating authentication headers.
- **Robust Error Handling**: Centralized error types for API errors, validation errors, and rate limits.
- **Utility Functions**: Helpers for URL encoding, timestamp parsing, and more.
- **Test Utilities**: Fixtures and helpers for testing Alpaca integrations (available with `test-utils` feature).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-base = "0.24.0"
```

While `alpaca-base` is primarily used internally by other crates in the workspace, you can use its types and authentication utilities directly.

> [!TIP]
> **Automated .env Loading**: `Credentials::from_env()` automatically attempts to load a `.env` file at the start of execution using the `dotenv` crate. You don't need to call `dotenv().ok()` manually in your code.

```rust
use alpaca_base::auth::AlpacaCredentials;

let credentials = AlpacaCredentials::new(
    "your_api_key".to_string(),
    "your_api_secret".to_string(),
    false, // paper trading
);
```

## API Reference

### Main Modules

- `auth` - Authentication types and credentials management.
- `error` - `AlpacaError` and related API error types.
- `types` - Core data structures (Account, Order, Position, etc.).
- `utils` - Common helper functions.

### Core Types

#### Trading
- `Order`, `Position`, `Account` - Core trading entities
- `CreateOrderRequest` - Order creation with builder pattern
- `TakeProfit`, `StopLoss` - Advanced order legs
- `OrderSide`, `OrderType`, `TimeInForce` - Order enums

#### Market Data
- `Bar`, `Quote`, `Trade` - Market data types
- `Asset`, `AssetClass`, `AssetStatus` - Asset information
- `CryptoBarsParams`, `MultiBarsParams` - Data query builders

#### Options Trading
- `OptionContract`, `OptionType`, `OptionStyle` - Options types
- `OptionContractParams`, `OptionBarsParams` - Options query builders
- `OptionsApprovalLevel` - Account approval levels

#### Broker API
- `BrokerAccount`, `BrokerAccountStatus` - Broker account types
- `Contact`, `Identity`, `Disclosures` - KYC types
- `AchRelationship`, `Transfer`, `Journal` - Money movement
- `TrustedContact`, `Agreement` - Compliance types

#### Corporate Actions
- `CorporateAction`, `CorporateActionType` - Corporate action types
- `CorporateActionsParams` - Query builder

#### Calendar & Clock
- `MarketClock`, `CalendarDay` - Market timing
- `MarketSession` - Session types (pre-market, regular, post-market)
- `CalendarParams`, `TradingDay` - Calendar utilities

#### FIX Protocol
- `FixVersion`, `FixSessionConfig` - FIX configuration
- `FixMsgType`, `FixSessionState` - FIX message types
- `FixSequenceNumbers` - Sequence management

#### Statements & Documents
- `StatementType`, `TaxFormType` - Document types
- `AccountDocument`, `TradeConfirmation`, `TaxDocument` - Document structs
- `DocumentParams` - Query builder

#### Local Currency Trading
- `Currency` - Supported currencies (USD, EUR, GBP, etc.)
- `ExchangeRate`, `CurrencyPair` - FX types
- `LctPosition` - Local currency positions

#### IRA Accounts
- `IraAccountType` - IRA types (Traditional, Roth, SEP, SIMPLE)
- `IraContribution`, `IraDistribution` - IRA transactions
- `IraBeneficiary` - Beneficiary management

#### OAuth
- `OAuthConfig`, `OAuthScope`, `OAuthToken` - OAuth 2.0 support

#### Utilities
- `RateLimitConfig`, `RateLimitStatus` - Rate limiting
- `TradingEnvironment`, `EnvironmentGuard` - Environment management

## Examples

Run examples with `cargo run -p alpaca-base --example <name>`:

| Example | Description |
|---------|-------------|
| `base_credentials_from_env` | Load credentials from environment variables |
| `base_error_handling` | Error types and handling patterns |
| `base_order_types` | Order type configurations (OrderType, OrderSide, TimeInForce) |
| `base_bar_params_builder` | Build market data query parameters |
| `base_asset_filtering` | Filter and query assets |
| `base_bracket_order_config` | Configure bracket orders with TakeProfit/StopLoss |
| `base_option_contract_params` | Build options contract query parameters |
| `base_broker_account_types` | Broker account KYC types (Contact, Identity, etc.) |
| `base_oauth_config` | OAuth 2.0 configuration and token handling |
| `base_rate_limit_config` | Rate limiting configuration and status |
| `base_calendar_types` | Market calendar and clock types |
| `base_currency_conversion` | Currency types and exchange rates |
| `base_ira_types` | IRA account types and transactions |
| `base_test_fixtures`* | Test fixtures for unit testing |
| `base_test_assertions`* | Assertion helpers for testing |

*Requires `--features test-utils`

```bash
# Example: Load credentials
cargo run -p alpaca-base --example base_credentials_from_env

# Example: Error handling patterns
cargo run -p alpaca-base --example base_error_handling
```

## Changelog

### v0.24.0 (latest)
- Added IRA Account Support (IraAccountType, IraContribution, IraDistribution, IraBeneficiary)
- Added Local Currency Trading (Currency, ExchangeRate, CurrencyPair, LctPosition)
- Added Statements & Confirmations (StatementType, AccountDocument, TaxDocument)
- Added FIX Protocol types (FixVersion, FixSessionConfig, FixMsgType)
- Added Calendar & Clock (MarketClock, CalendarDay, MarketSession)
- Added OAuth 2.0 support
- Added Corporate Actions types
- Added Options Trading support
- Added Broker API types
- Enhanced error handling with `thiserror`
- Comprehensive API models updated to latest Alpaca specification

## License

MIT
