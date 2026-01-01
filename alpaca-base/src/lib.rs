//! # Alpaca Base
//!
//! Base library with common structs, traits, and logic for Alpaca API clients.
//! This crate provides shared types, error handling, and utilities used across
//! all Alpaca API client implementations.

pub mod auth;
pub mod error;
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
pub mod types;
pub mod utils;

pub use auth::*;
pub use error::{AlpacaError, ApiErrorCode, ApiErrorResponse, RateLimitInfo, Result, ValidationError};
pub use types::*;
pub use utils::*;
