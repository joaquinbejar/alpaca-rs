<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/alpaca-rs/refs/heads/main/doc/images/logo.png" alt="alpaca-rs" style="width: 80%; height: 80%;">
</div>

[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alpaca-http.svg)](https://crates.io/crates/alpaca-http)
[![Downloads](https://img.shields.io/crates/d/alpaca-http.svg)](https://crates.io/crates/alpaca-http)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-http)

# alpaca-http

HTTP REST API client for the Alpaca trading platform.

## Overview

`alpaca-http` is a robust HTTP client for interacting with Alpaca's REST API. It handles authentication, request signing, and provides a clean interface for trading and market data endpoints.

## Features

- **Trading Endpoints**: Create, list, and cancel orders; manage positions and accounts.
- **Market Data**: Access historical and real-time stocks and crypto data.
- **Advanced Order Support**: Easily configure bracket, OCO, and OTO orders.
- **Broker API**: Integrated support for Broker-specific endpoints and KYC.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-http = "0.20.0"
```

## Usage

```rust
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest};
use alpaca_base::{Credentials, Environment, OrderSide};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::from_env()?;
    let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;

    let account = client.get_account().await?;
    println!("Account status: {:?}", account.status);

    Ok(())
}
```

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
- **Documentation**: <https://docs.rs/alpaca-http>

We appreciate your interest and look forward to your contributions!

**License**: MIT

## Disclaimer

This software is not officially associated with Alpaca Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a paper trading account before using in a live trading environment.
