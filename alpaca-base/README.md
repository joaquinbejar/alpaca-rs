<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/alpaca-rs/refs/heads/main/doc/images/logo.png" alt="alpaca-rs" style="width: 80%; height: 80%;">
</div>

[![License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alpaca-base.svg)](https://crates.io/crates/alpaca-base)
[![Downloads](https://img.shields.io/crates/d/alpaca-base.svg)](https://crates.io/crates/alpaca-base)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/alpaca-base)

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
alpaca-base = "0.25.0"
```

## Usage

While `alpaca-base` is primarily used internally by other crates in the workspace, you can use its types and authentication utilities directly.

> [!TIP]
> **Automated .env Loading**: `Credentials::from_env()` automatically attempts to load a `.env` file at the start of execution using the `dotenv` crate. You don't need to call `dotenv().ok()` manually in your code.

```rust
use alpaca_base::auth::Credentials;

// Automatically loads from .env
let credentials = Credentials::from_env()?;
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
- **Documentation**: <https://docs.rs/alpaca-base>

We appreciate your interest and look forward to your contributions!

**License**: MIT

## Disclaimer

This software is not officially associated with Alpaca Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a paper trading account before using in a live trading environment.
