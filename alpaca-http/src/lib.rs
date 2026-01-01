//! # Alpaca HTTP Client
//!
//! HTTP REST API client for Alpaca trading platform.
//! This crate provides a comprehensive client for interacting with Alpaca's REST API endpoints.

pub mod client;
pub mod endpoints;
pub mod error;

pub use alpaca_base::*;
pub use client::AlpacaHttpClient;
pub use error::HttpError;
