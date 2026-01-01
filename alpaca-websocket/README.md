# alpaca-websocket

WebSocket client for the Alpaca trading platform real-time data and account updates.

## Overview

`alpaca-websocket` provides a robust, asynchronous interface for Alpaca's streaming APIs. It supports real-time market data (trades, quotes, bars) and account-related event streams (order updates, position changes).

## Features

- **Real-Time Market Data**: Subscribe to stock and crypto streams for sub-second updates.
- **Account Updates**: Stream trading events to react instantly to order fills and cancellations.
- **Automatic Reconnection**: Intelligent connection management with configurable retry logic.
- **Multiple Stream Support**: Connect to Market Data v2 and Trading streams simultaneously.
- **Type-Safe Messages**: Strongly typed representations for all WebSocket message types.

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
- `StreamType` - Enum specifying which Alpaca stream to connect to (MarketData or Trading).

### Main Modules

- `client` - WebSocket client implementation.
- `messages` - Message types for all streaming data.
- `streams` - Stream handling and subscription logic.

## Examples

See the `examples/` directory for detailed streaming usage:
- `stream_market_data.rs` - Subscribing to trade and quote updates.
- `stream_trade_updates.rs` - Listening for order execution events.

## Changelog

### v0.2.0 (latest)
- Integrated into the workspace structure.
- Improved subscription management.
- Added support for enhanced Crypto Market Data.

## License

MIT
