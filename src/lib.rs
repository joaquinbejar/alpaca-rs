//! # Alpaca Markets API Client
//!
//! This crate provides a Rust client for the Alpaca Markets API.
//!
//! ## Example
//!
//! ```rust
//! use alpaca_rs::Client;
//!
//! // Basic usage example will be added as the API is implemented
//! ```

pub mod client;
pub mod error;
pub mod types;

pub use client::Client;
pub use error::{AlpacaError, Result};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
