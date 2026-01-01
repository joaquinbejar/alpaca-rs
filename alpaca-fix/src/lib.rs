//! # alpaca-fix
//!
//! FIX (Financial Information eXchange) protocol client for the Alpaca trading platform.
//!
//! This crate provides a FIX protocol implementation for high-frequency trading
//! applications with Alpaca. It supports FIX 4.2 and 4.4 versions.
//!
//! ## Features
//!
//! - FIX session management with heartbeat and sequence numbers
//! - Order routing (New Order Single, Cancel, Cancel/Replace)
//! - Execution reports
//! - Market data requests and snapshots
//! - Session recovery
//!
//! ## Example
//!
//! ```rust,ignore
//! use alpaca_fix::{FixClient, FixConfig, FixVersion};
//!
//! let config = FixConfig::builder()
//!     .version(FixVersion::Fix44)
//!     .sender_comp_id("YOUR_SENDER_ID")
//!     .target_comp_id("ALPACA")
//!     .build();
//!
//! let client = FixClient::new(credentials, config);
//! ```

pub mod client;
pub mod codec;
pub mod config;
pub mod error;
pub mod messages;
pub mod session;

pub use client::FixClient;
pub use config::{FixConfig, FixVersion};
pub use error::FixError;
pub use messages::*;
