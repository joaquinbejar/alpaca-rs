# alpaca-rs

[![CI](https://github.com/joaquinbejar/alpaca-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/joaquinbejar/alpaca-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/joaquinbejar/alpaca-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/joaquinbejar/alpaca-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust client library for the [Alpaca Markets](https://alpaca.markets/) trading API.

## Overview

This workspace provides a complete Rust implementation for interacting with Alpaca's trading platform:

| Crate | Description | Version |
|-------|-------------|---------|
| `alpaca-base` | Core types, error handling, and utilities | 0.24.0 |
| `alpaca-http` | HTTP REST API client for trading and market data | 0.20.0 |
| `alpaca-websocket` | WebSocket client for real-time streaming data | 0.2.0 |
| `alpaca-fix` | FIX protocol client for high-frequency trading | 0.1.0 |

## Features

- **Trading API**: Accounts, orders, positions, watchlists
- **Market Data API**: Stocks, crypto, options bars, quotes, trades
- **Advanced Orders**: Bracket, OCO, OTO, trailing stop
- **Options Trading**: Options contracts and market data
- **Broker API**: Account management, funding, journals, compliance
- **Corporate Actions**: Dividends, splits, and other events
- **Calendar & Clock**: Market hours and trading calendar
- **Local Currency Trading**: Exchange rates and FX support
- **IRA Accounts**: Contributions, distributions, beneficiaries
- **OAuth 2.0**: Full OAuth authentication support
- **FIX Protocol**: High-frequency trading via FIX 4.2/4.4
- **Real-time Streaming**: WebSocket for live market data
- **Error Handling**: Typed errors with retry support
- **Testing Utilities**: Fixtures and helpers for testing
- **Async/Await**: Built on Tokio for async operations

## Installation

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
alpaca-base = "0.24"
alpaca-http = "0.20"
alpaca-websocket = "0.2"
alpaca-fix = "0.1"  # For FIX protocol (optional)
```

## Quick Start

```rust
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest};
use alpaca_base::{Credentials, Environment, OrderSide};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials and client
    let credentials = Credentials::new(
        std::env::var("ALPACA_API_KEY")?,
        std::env::var("ALPACA_API_SECRET")?,
    );
    let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;

    // Get account information
    let account = client.get_account().await?;
    println!("Buying Power: ${}", account.buying_power);
    println!("Portfolio Value: ${}", account.portfolio_value);

    // Create a market order
    let order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "1");
    let result = client.create_order(&order).await?;
    println!("Order ID: {}", result.id);

    Ok(())
}
```

## Order Types

### Simple Orders

```rust
// Market order
let order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "10");

// Limit order
let order = CreateOrderRequest::limit("AAPL", OrderSide::Buy, "10", "150.00");

// Stop order
let order = CreateOrderRequest::stop("AAPL", OrderSide::Sell, "10", "145.00");

// Stop-limit order
let order = CreateOrderRequest::stop_limit("AAPL", OrderSide::Sell, "10", "145.00", "144.50");

// Trailing stop (by price)
let order = CreateOrderRequest::trailing_stop_price("AAPL", OrderSide::Sell, "10", "5.00");

// Trailing stop (by percent)
let order = CreateOrderRequest::trailing_stop_percent("AAPL", OrderSide::Sell, "10", "2.5");
```

### Advanced Order Classes

```rust
use alpaca_base::{TakeProfit, StopLoss, OrderType};

// Bracket order: primary order with take-profit and stop-loss legs
let order = CreateOrderRequest::bracket(
    "AAPL",
    OrderSide::Buy,
    "10",
    OrderType::Limit,
    TakeProfit::new("160.00"),
    StopLoss::with_limit("140.00", "139.50"),
)
.with_limit_price("150.00");

// OCO (One-Cancels-Other): two orders where filling one cancels the other
let order = CreateOrderRequest::oco(
    "AAPL",
    OrderSide::Sell,
    "10",
    TakeProfit::new("160.00"),
    StopLoss::new("140.00"),
);

// OTO (One-Triggers-Other): primary order that triggers a secondary order
let order = CreateOrderRequest::oto(
    "AAPL",
    OrderSide::Buy,
    "10",
    OrderType::Limit,
    StopLoss::new("140.00"),
);
```

## Error Handling

The library provides comprehensive error handling with typed errors:

```rust
use alpaca_base::{AlpacaError, ApiErrorCode};

match client.create_order(&order).await {
    Ok(order) => println!("Order created: {}", order.id),
    Err(AlpacaError::Api { status, message, error_code, request_id }) => {
        println!("API error {}: {}", status, message);
        if let Some(code) = error_code {
            if code.is_retryable() {
                println!("This error is retryable");
            }
        }
    }
    Err(AlpacaError::RateLimit { retry_after_secs, .. }) => {
        println!("Rate limited, retry after {} seconds", retry_after_secs);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Configuration

### Environment Variables

Set up your API credentials:

```bash
export ALPACA_API_KEY=your_api_key
export ALPACA_API_SECRET=your_api_secret
```

### Using .env File

Create a `.env` file (see `.env.example`):

```env
ALPACA_API_KEY=your_api_key
ALPACA_API_SECRET=your_api_secret
ALPACA_BASE_URL=https://paper-api.alpaca.markets
```

### Environments

```rust
use alpaca_base::Environment;

// Paper trading (recommended for testing)
let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;

// Live trading
let client = AlpacaHttpClient::new(credentials, Environment::Live)?;
```

## Examples

Run the examples with:

```bash
# Bracket order example
cargo run --example trading_bracket_order
```

## Testing

The library includes test utilities for your own tests:

```rust
// Enable test-utils feature
// alpaca-base = { version = "0.24", features = ["test-utils"] }

use alpaca_base::test_utils::{fixtures, assertions};

let account = fixtures::sample_account();
assertions::assert_account_active(&account);

let order = fixtures::sample_order("AAPL", OrderSide::Buy, "10");
assertions::assert_order_basics(&order, "AAPL", OrderSide::Buy, OrderType::Market);
```

## API Documentation

- [alpaca-base docs](https://docs.rs/alpaca-base)
- [alpaca-http docs](https://docs.rs/alpaca-http)
- [alpaca-websocket docs](https://docs.rs/alpaca-websocket)
- [alpaca-fix docs](https://docs.rs/alpaca-fix)
- [Alpaca API Documentation](https://docs.alpaca.markets/)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please see:
- [CHANGELOG.md](CHANGELOG.md) for version history
- [.issues/](/.issues/) for planned features and roadmap

## Disclaimer

This library is not affiliated with Alpaca Markets. Use at your own risk. Always test with paper trading before using real money.
