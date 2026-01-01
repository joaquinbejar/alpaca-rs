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

## Usage

While `alpaca-base` is primarily used internally by other crates in the workspace, you can use its types and authentication utilities directly.

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
