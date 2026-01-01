//! # Alpaca WebSocket Client
//!
//! WebSocket client for Alpaca trading platform real-time data.
//! This crate provides real-time market data and trading updates via WebSocket connections.

pub mod client;
pub mod config;
pub mod error;
pub mod messages;
pub mod streams;

pub use alpaca_base::*;
pub use client::AlpacaWebSocketClient;
pub use config::{ConnectionState, StreamType, WebSocketConfig};
pub use error::WebSocketError;
pub use messages::*;
pub use streams::*;
