<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/alpaca-rs/refs/heads/main/doc/images/logo.png" alt="alpaca-rs" style="width: 80%; height: 80%;">
</div>

[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alpaca-fix.svg)](https://crates.io/crates/alpaca-fix)
[![Downloads](https://img.shields.io/crates/d/alpaca-fix.svg)](https://crates.io/crates/alpaca-fix)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-fix)

# alpaca-fix

FIX (Financial Information eXchange) protocol client for the Alpaca trading platform.

## Overview

`alpaca-fix` provides a FIX protocol implementation for high-frequency trading applications with Alpaca. It supports FIX 4.2 and 4.4 versions for order routing and market data.

## Features

- **FIX Session Management**: Session initiation, heartbeat handling, sequence number management.
- **Order Routing**: New Order Single, Cancel, Cancel/Replace requests.
- **Execution Reports**: Real-time order status updates.
- **Market Data**: Market data requests and snapshots.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-fix = "0.3.0"
```

## Examples

Run examples with `cargo run -p alpaca-fix --example <name>`:

### Session & Message Examples

| Example | Description |
|---------|-------------|
| `fix_session_setup` | Configure and set up a FIX session |
| `fix_new_order_single` | Create New Order Single messages |
| `fix_order_cancel` | Create Cancel and Cancel/Replace messages |
| `fix_execution_report` | Execution Report message structure |
| `fix_market_data_request` | Market Data Request messages |

### Connection & Trading Examples

| Example | Description |
|---------|-------------|
| `fix_connect` | Connect to FIX server |
| `fix_disconnect` | Graceful disconnect |
| `fix_market_order` | Send market order via FIX |
| `fix_limit_order` | Send limit order via FIX |
| `fix_cancel_order` | Cancel order via FIX |
| `fix_execution_reports` | Process execution reports |
| `fix_heartbeat` | Heartbeat handling |
| `fix_message_loop` | Receive and process messages |

```bash
# Session setup example
cargo run -p alpaca-fix --example fix_session_setup

# New Order Single example
cargo run -p alpaca-fix --example fix_new_order_single
```

**Note**: FIX protocol requires special access from Alpaca. Contact Alpaca support to enable FIX access for your account.

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
- **Documentation**: <https://docs.rs/alpaca-fix>

We appreciate your interest and look forward to your contributions!

**License**: MIT

## Disclaimer

This software is not officially associated with Alpaca Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a paper trading account before using in a live trading environment.
