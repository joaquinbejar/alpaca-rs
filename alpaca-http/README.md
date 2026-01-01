# alpaca-http

HTTP REST API client for the Alpaca trading platform.

## Overview

`alpaca-http` provides a high-level, asynchronous Rust client for Alpaca's REST API. It covers trading, market data, and account management endpoints, allowing you to build automated trading systems with ease.

## Features

- **Asynchronous API**: Built on top of `reqwest` and `tokio` for high-performance, non-blocking I/O.
- **Comprehensive Endpoint Support**:
    - **Trading**: Place, list, and cancel orders.
    - **Account**: Get account details, activities, and portfolio history.
    - **Market Data**: Fetch bars, quotes, and trades for stocks and crypto.
    - **Assets**: List and query asset information.
- **Type-Safe Requests**: Structured request and response models for all endpoints.
- **Automated Authentication**: Handles API key headers and environment-based configuration.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-http = "0.7.0"
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
- `OrderParams` - Builder for order creation requests.
- `HttpError` - HTTP-specific error wrapper.

### Main Modules

- `client` - Core HTTP client implementation.
- `endpoints` - Request and response models for all supported endpoints.

## Examples

Check the `examples/` directory in the repository for detailed usage:
- `trading_orders.rs` - Creating and managing orders.
- `market_data.rs` - Fetching real-time and historical market data.

## Changelog

### v0.7.0 (latest)
- Migrated to workspace structure.
- Improved async ergonomics.
- Added support for latest market data v2 endpoints.

## License

MIT
