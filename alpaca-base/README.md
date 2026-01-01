# alpaca-base

Base library with common structs, traits, and logic for Alpaca API clients.

## Overview

`alpaca-base` provides the foundational building blocks for the Alpaca Market API Rust clients. It includes shared data models, authentication utilities, custom error types, and common helper functions used by both `alpaca-http` and `alpaca-websocket`.

## Features

- **Core Data Models**: Comprehensive Rust representations of Alpaca API objects (Orders, Positions, Assets, etc.).
- **Authentication**: Utilities for managing API keys and generating authentication headers.
- **Robust Error Handling**: Centralized error types for API errors, validation errors, and rate limits.
- **Utility Functions**: Helpers for URL encoding, timestamp parsing, and more.
- **Test Utilities**: Fixtures and helpers for testing Alpaca integrations (available with `test-utils` feature).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-base = "0.8.0"
```

## Usage

While `alpaca-base` is primarily used internally by other crates in the workspace, you can use its types and authentication utilities directly.

```rust
use alpaca_base::auth::AlpacaCredentials;

let credentials = AlpacaCredentials::new(
    "your_api_key".to_string(),
    "your_api_secret".to_string(),
    false, // paper trading
);
```

## API Reference

### Main Modules

- `auth` - Authentication types and credentials management.
- `error` - `AlpacaError` and related API error types.
- `types` - Core data structures (Account, Order, Position, etc.).
- `utils` - Common helper functions.

### Main Types

- `AlpacaCredentials` - Stores and manages API credentials.
- `AlpacaError` - Unified error type for the entire library.
- `Order` - Representation of an Alpaca order.
- `Account` - Representation of an Alpaca account.

## Changelog

### v0.8.0 (latest)
- Initial workspace consolidation.
- Enhanced error handling with `thiserror`.
- Comprehensive API models updated to latest Alpaca specification.

## License

MIT
