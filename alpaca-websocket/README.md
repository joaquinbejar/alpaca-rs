# alpaca-websocket

[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alpaca-websocket.svg)](https://crates.io/crates/alpaca-websocket)
[![Downloads](https://img.shields.io/crates/d/alpaca-websocket.svg)](https://crates.io/crates/alpaca-websocket)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-websocket)

WebSocket client for the Alpaca trading platform real-time data and account updates.

## Overview

`alpaca-websocket` provides a robust, asynchronous interface for Alpaca's streaming APIs. It supports real-time market data (trades, quotes, bars) and account-related event streams (order updates, position changes).

## Features

- **Real-Time Market Data**: Subscribe to stock and crypto streams for sub-second updates.
- **Account Updates**: Stream trading events to react instantly to order fills and cancellations.
- **Automatic Reconnection**: Intelligent connection management with configurable retry logic.
- **Multiple Stream Support**: Connect to Market Data v2 and Trading streams simultaneously.
- **Type-Safe Messages**: Strongly typed representations for all WebSocket message types.
- **Configurable**: Flexible configuration for timeouts, reconnection, and buffer sizes.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-websocket = "0.2.0"
```

## Usage

```rust
use alpaca_websocket::AlpacaWebSocketClient;
use alpaca_base::auth::AlpacaCredentials;
use alpaca_websocket::config::StreamType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = AlpacaCredentials::from_env()?;
    let mut client = AlpacaWebSocketClient::new(credentials, StreamType::MarketData);

    client.connect().await?;
    client.subscribe_trades(&["AAPL", "TSLA"]).await?;

    while let Some(msg) = client.next_message().await? {
        println!("Received: {:?}", msg);
    }

    Ok(())
}
```

## API Reference

### Main Types

- `AlpacaWebSocketClient` - The primary client for managing WebSocket connections.
- `WebSocketConfig` - Configuration options for timeouts and reconnection behavior.
- `StreamType` - Enum specifying which Alpaca stream to connect to.

### Stream Types

- `StreamType::MarketData` - Real-time stock market data (IEX or SIP)
- `StreamType::CryptoData` - Real-time cryptocurrency data
- `StreamType::Trading` - Account and order updates
- `StreamType::News` - Real-time news feed

### Main Modules

- `client` - WebSocket client implementation.
- `config` - Configuration types and builders.
- `messages` - Message types for all streaming data.

### Message Types

#### Market Data
- `TradeMessage` - Real-time trade executions
- `QuoteMessage` - Real-time bid/ask quotes
- `BarMessage` - Real-time OHLCV bars

#### Trading Updates
- `TradeUpdateEvent` - Order status changes (fill, cancel, etc.)
- `ConnectionStatus` - Connection state notifications

#### Subscriptions
- `Subscription` - Builder for managing stream subscriptions

## Configuration

```rust
use alpaca_websocket::config::WebSocketConfig;

let config = WebSocketConfig::builder()
    .reconnect_enabled(true)
    .reconnect_max_attempts(5)
    .reconnect_delay_ms(1000)
    .ping_interval_secs(30)
    .build();
```

## Examples

See the `examples/` directory for detailed streaming usage:
- `stream_market_data.rs` - Subscribing to trade and quote updates.
- `stream_trade_updates.rs` - Listening for order execution events.

## Changelog

### v0.2.0 (latest)
- Integrated into the workspace structure.
- Improved subscription management with builder pattern.
- Added support for enhanced Crypto Market Data.
- Added News stream support.
- Improved connection status handling.
- Added configurable reconnection logic.

## License

MIT
