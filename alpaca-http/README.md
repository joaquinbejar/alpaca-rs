# alpaca-http

HTTP REST API client for the Alpaca trading platform.

## Overview

`alpaca-http` provides a high-level, asynchronous Rust client for Alpaca's REST API. It covers trading, market data, broker API, and account management endpoints, allowing you to build automated trading systems with ease.

## Features

- **Asynchronous API**: Built on top of `reqwest` and `tokio` for high-performance, non-blocking I/O.
- **Comprehensive Endpoint Support**:
    - **Trading**: Place, list, and cancel orders (market, limit, stop, bracket, OCO, OTO).
    - **Account**: Get account details, activities, portfolio history, and configurations.
    - **Market Data**: Fetch bars, quotes, and trades for stocks, crypto, and options.
    - **Assets**: List and query asset information.
    - **Broker API**: Account management, funding, journals, and compliance.
    - **Options**: Options contracts and market data.
    - **Corporate Actions**: Dividends, splits, and other corporate events.
    - **Calendar & Clock**: Market hours and trading calendar.
    - **Local Currency Trading**: Exchange rates and FX support.
    - **IRA Accounts**: Contributions, distributions, and beneficiaries.
- **Type-Safe Requests**: Structured request and response models for all endpoints.
- **Automated Authentication**: Handles API key headers and environment-based configuration.
- **Rate Limiting**: Built-in rate limit handling with retry support.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-http = "0.20.0"
```

## Usage

```rust
use alpaca_http::AlpacaHttpClient;
use alpaca_base::auth::AlpacaCredentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = AlpacaCredentials::from_env()?;
    let client = AlpacaHttpClient::new(credentials);

    let account = client.get_account().await?;
    println!("Account ID: {}", account.id);

    Ok(())
}
```

## API Reference

### Main Types

- `AlpacaHttpClient` - The primary entry point for making REST API calls.
- `CreateOrderRequest` - Builder for order creation requests.
- `ReplaceOrderRequest` - Builder for order modification requests.

### Main Modules

- `client` - Core HTTP client implementation.
- `endpoints` - Request and response models for all supported endpoints.

### Endpoint Categories

#### Trading API
- `get_account()` - Get account information
- `create_order()`, `get_order()`, `list_orders()`, `cancel_order()` - Order management
- `replace_order()` - Modify existing orders
- `get_position()`, `list_positions()`, `close_position()` - Position management
- `list_activities()` - Account activities
- `get_portfolio_history()` - Historical portfolio data

#### Market Data API
- `get_bars()`, `get_multi_bars()` - Stock bars (OHLCV)
- `get_quotes()`, `get_trades()` - Quotes and trades
- `get_crypto_bars()` - Cryptocurrency data
- `get_option_bars()` - Options market data
- `get_latest_quote()`, `get_latest_trade()` - Real-time data

#### Assets & Options
- `list_assets()`, `get_asset()` - Asset information
- `list_option_contracts()`, `get_option_contract()` - Options contracts

#### Broker API
- `create_broker_account()`, `get_broker_account()` - Account management
- `create_ach_relationship()`, `list_ach_relationships()` - ACH funding
- `create_transfer()`, `list_transfers()` - Wire/ACH transfers
- `create_journal()`, `list_journals()` - Journal entries
- `list_documents()`, `get_document()` - Account documents

#### Calendar & Clock
- `get_calendar()` - Trading calendar
- `get_clock()` - Market clock status

#### Corporate Actions
- `list_corporate_actions()` - Dividends, splits, etc.

#### Local Currency Trading
- `get_exchange_rates()` - All exchange rates
- `get_exchange_rate()` - Specific currency pair rate

#### IRA Accounts
- `list_ira_contributions()`, `create_ira_contribution()` - IRA contributions
- `list_ira_distributions()` - IRA distributions
- `list_ira_beneficiaries()` - IRA beneficiaries

## Examples

Run examples with `cargo run -p alpaca-http --example <name>`:

| Example | Description |
|---------|-------------|
| `http_get_account` | Get account information |
| `http_create_market_order` | Place a market order |
| `http_create_limit_order` | Place a limit order with GTC |
| `http_list_orders` | List and filter orders |
| `http_cancel_order` | Cancel an open order |
| `http_get_positions` | List all positions |
| `http_close_position` | Close a position |
| `market_data` | Fetch real-time and historical market data |
| `options_trading` | Options trading examples |
| `trading_bracket_order` | Advanced bracket orders |

```bash
# Get account info
cargo run -p alpaca-http --example http_get_account

# Place a market order (Paper trading)
cargo run -p alpaca-http --example http_create_market_order
```

**Note**: Examples require `ALPACA_API_KEY` and `ALPACA_API_SECRET` environment variables.

## Changelog

### v0.20.0 (latest)
- Added IRA Account endpoints (contributions, distributions, beneficiaries)
- Added Local Currency Trading endpoints (exchange rates)
- Added Calendar & Clock endpoints
- Added Corporate Actions endpoints
- Added Options Trading endpoints
- Added Broker API endpoints (accounts, funding, journals)
- Added OAuth 2.0 support
- Improved async ergonomics
- Added support for latest market data v2 endpoints

## License

MIT
