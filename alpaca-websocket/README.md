<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/alpaca-rs/refs/heads/main/doc/images/logo.png" alt="alpaca-rs" style="width: 80%; height: 80%;">
</div>

[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alpaca-websocket.svg)](https://crates.io/crates/alpaca-websocket)
[![Downloads](https://img.shields.io/crates/d/alpaca-websocket.svg)](https://crates.io/crates/alpaca-websocket)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-websocket)

# alpaca-websocket

WebSocket client for the Alpaca trading platform real-time data and account updates.

## Overview

`alpaca-websocket` provides a real-time streaming interface for Alpaca's market data and account events. It's built on top of `tokio-tungstenite` for high-performance asynchronous streaming.

## Features

- **Real-time Market Data**: Stream trades, quotes, and bars for stocks and crypto.
- **Account Updates**: Receive real-time notifications about order fills and account changes.
- **Automatic Reconnection**: Built-in logic to handle network disruptions.
- **Easy Subscription**: Clean API for subscribing to multiple symbols.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-websocket = "0.3.1"
```

## Examples

Run examples with `cargo run -p alpaca-websocket --example <name>`:

| Example | Description |
|---------|-------------|
| `ws_stock_trades_stream` | Stream real-time stock trades |
| `ws_stock_quotes_stream` | Stream real-time stock quotes |
| `ws_stock_bars_stream` | Stream real-time stock bars |
| `ws_crypto_stream` | Stream real-time crypto data |
| `ws_trade_updates` | Stream order/trade updates |
| `ws_websocket_config` | Configure WebSocket client |
| `ws_reconnection` | Handle disconnections and reconnect |
| `ws_stream_news` | Subscribe to real-time news |

```bash
# Stream stock trades
cargo run -p alpaca-websocket --example ws_stock_trades_stream

# Stream trade updates (order fills)
cargo run -p alpaca-websocket --example ws_trade_updates
```

**Note**: Examples require `ALPACA_API_KEY` and `ALPACA_API_SECRET` environment variables.

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
- **Documentation**: <https://docs.rs/alpaca-websocket>

We appreciate your interest and look forward to your contributions!

**License**: MIT

## Disclaimer

This software is not officially associated with Alpaca Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a paper trading account before using in a live trading environment.
