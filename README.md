# alpaca-rs

A comprehensive Rust client library for the [Alpaca Markets](https://alpaca.markets/) trading API.

## Overview

This workspace provides a complete Rust implementation for interacting with Alpaca's trading platform, including:

- **alpaca-base**: Core types, error handling, and utilities
- **alpaca-http**: HTTP REST API client for trading and market data
- **alpaca-websocket**: WebSocket client for real-time streaming data

## Features

- Full Trading API support (accounts, orders, positions, watchlists)
- Market Data API (stocks, crypto bars, quotes, trades)
- Advanced order types (bracket, OCO, OTO, trailing stop)
- Real-time WebSocket streaming
- Paper and live trading environments
- Async/await support with Tokio

## Installation

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
alpaca-base = "0.2"
alpaca-http = "0.2"
alpaca-websocket = "0.1"
```

## Quick Start

```rust
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest};
use alpaca_base::{Credentials, Environment, OrderSide, TakeProfit, StopLoss, OrderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials and client
    let credentials = Credentials::new(
        "your_api_key".to_string(),
        "your_api_secret".to_string(),
    );
    let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;

    // Get account information
    let account = client.get_account().await?;
    println!("Buying Power: ${}", account.buying_power);

    // Create a simple market order
    let order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "1");
    // let result = client.create_order(&order).await?;

    // Create a bracket order with take-profit and stop-loss
    let bracket_order = CreateOrderRequest::bracket(
        "AAPL",
        OrderSide::Buy,
        "10",
        OrderType::Limit,
        TakeProfit::new("160.00"),
        StopLoss::with_limit("140.00", "139.50"),
    )
    .with_limit_price("150.00")
    .client_order_id("my-bracket-order");
    // let result = client.create_order(&bracket_order).await?;

    Ok(())
}
```

## Order Types

The library supports all Alpaca order types:

- **Market**: Execute immediately at current market price
- **Limit**: Execute at specified price or better
- **Stop**: Trigger market order when stop price is reached
- **Stop-Limit**: Trigger limit order when stop price is reached
- **Trailing Stop**: Dynamic stop that follows the market

### Advanced Order Classes

- **Bracket**: Primary order with take-profit and stop-loss legs
- **OCO (One-Cancels-Other)**: Two orders where filling one cancels the other
- **OTO (One-Triggers-Other)**: Primary order that triggers a secondary order

## Configuration

Set up your API credentials using environment variables:

```bash
export ALPACA_API_KEY=your_api_key
export ALPACA_API_SECRET=your_api_secret
```

Or use a `.env` file (see `.env.example`).

## Examples

Run the examples with:

```bash
cargo run --example trading_bracket_order
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please see the issues in `.issues/` for planned features.
