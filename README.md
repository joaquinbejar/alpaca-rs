<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/alpaca-rs/refs/heads/main/doc/images/logo.png" alt="alpaca-rs" style="width: 80%; height: 80%;">
</div>

[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alpaca-rs.svg)](https://crates.io/crates/alpaca-rs)
[![Downloads](https://img.shields.io/crates/d/alpaca-rs.svg)](https://crates.io/crates/alpaca-rs)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/alpaca-rs.svg)](https://github.com/joaquinbejar/alpaca-rs/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/alpaca-rs.svg)](https://github.com/joaquinbejar/alpaca-rs/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/alpaca-rs.svg)](https://github.com/joaquinbejar/alpaca-rs/pulls)

[![Build Status](https://github.com/joaquinbejar/alpaca-rs/workflows/CI/badge.svg)](https://github.com/joaquinbejar/alpaca-rs/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/alpaca-rs)](https://codecov.io/gh/joaquinbejar/alpaca-rs)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/alpaca-rs)](https://libraries.io/github/joaquinbejar/alpaca-rs)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-rs)

A comprehensive Rust client library for the [Alpaca Markets](https://alpaca.markets/) trading API.

## Overview

This workspace provides a complete Rust implementation for interacting with Alpaca's trading platform:

| Crate | Description | Version |
|-------|-------------|---------|
| `alpaca-base` | Core types, error handling, and utilities | 0.24.0 |
| `alpaca-http` | HTTP REST API client for trading and market data | 0.20.0 |
| `alpaca-websocket` | WebSocket client for real-time streaming data | 0.2.0 |
| `alpaca-fix` | FIX protocol client for high-frequency trading | 0.2.0 |

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
alpaca-fix = "0.2"
```

## Quick Start

```rust
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest};
use alpaca_base::{Credentials, Environment, OrderSide};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials and client
    // Note: Credentials::from_env() automatically loads .env files
    let credentials = Credentials::from_env()?;
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

## Examples

Detailed examples are available in the `examples/` directory:
- `base_credentials_from_env.rs`: Loading API keys and automated .env support
- `base_error_handling.rs`: Handling API errors and retryable status
- `base_order_types.rs`: Order configuration (Market, Limit, etc.)
- `base_bar_params_builder.rs`: Market data query builders
- `base_asset_filtering.rs`: Querying and filtering available assets

## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

### **Contact Information**
- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **Repository**: <https://github.com/joaquinbejar/alpaca-rs>
- **Documentation**: <https://docs.rs/alpaca-rs>

We appreciate your interest and look forward to your contributions!

**License**: MIT

## Disclaimer

This software is not officially associated with Alpaca Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a paper trading account before using in a live trading environment.
