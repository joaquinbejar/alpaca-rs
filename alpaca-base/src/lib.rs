//! # Alpaca Base
//!
//! Base library with common structs, traits, and logic for Alpaca API clients.
//! This crate provides shared types, error handling, and utilities used across
//! all Alpaca API client implementations.

/// Authentication types and utilities.
pub mod auth;
/// Error types and handling.
pub mod error;
/// Test utilities and fixtures (requires `test-utils` feature).
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
/// Core API types and data structures.
pub mod types;
/// Utility functions and helpers.
pub mod utils;

pub use auth::*;
pub use error::{
    AlpacaError, ApiErrorCode, ApiErrorResponse, RateLimitInfo, Result, ValidationError,
};
pub use types::*;
pub use utils::*;
