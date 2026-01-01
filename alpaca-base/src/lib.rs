//! # Alpaca Base
//!
//! Base library with common structs, traits, and logic for Alpaca API clients.
//! This crate provides shared types, error handling, and utilities used across
//! all Alpaca API client implementations.

pub mod auth;
pub mod error;
pub mod types;
pub mod utils;

pub use auth::*;
pub use error::{AlpacaError, Result};
pub use types::*;
pub use utils::*;
