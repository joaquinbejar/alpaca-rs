//! # HTTP + WebSocket Integration
//!
//! This example demonstrates how to combine HTTP and WebSocket clients
//! for a complete trading workflow.
//!
//! ## Prerequisites
//!
//! Set environment variables:
//! - `ALPACA_API_KEY`: Your Alpaca API key
//! - `ALPACA_API_SECRET`: Your Alpaca secret key
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example integration_http_websocket
//! ```
//!
//! **Note**: This example demonstrates the integration pattern but does not
//! execute real trades.

use alpaca_base::{Credentials, Environment};
use alpaca_http::AlpacaHttpClient;
use alpaca_websocket::{AlpacaWebSocketClient, SubscribeMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HTTP + WebSocket Integration ===\n");

    // Load credentials once, use for both clients
    let credentials = Credentials::from_env()?;
    let environment = Environment::Paper;
    println!("Environment: {:?}", environment);

    // Create HTTP client for REST operations
    println!("\n--- HTTP Client Setup ---");
    let http_client = AlpacaHttpClient::new(credentials.clone(), environment);
    println!("HTTP client created");

    // Get account info via HTTP
    println!("\n--- Account Info (HTTP) ---");
    match http_client.get_account().await {
        Ok(account) => {
            println!("  Account ID: {}", account.id);
            println!("  Status: {:?}", account.status);
            println!("  Buying Power: ${}", account.buying_power);
            println!("  Cash: ${}", account.cash);
        }
        Err(e) => {
            println!("  Error getting account: {}", e);
        }
    }

    // Check market status via HTTP
    println!("\n--- Market Clock (HTTP) ---");
    match http_client.get_clock().await {
        Ok(clock) => {
            println!("  Is Open: {}", clock.is_open);
            println!("  Timestamp: {}", clock.timestamp);
            if let Some(next_open) = clock.next_open {
                println!("  Next Open: {}", next_open);
            }
            if let Some(next_close) = clock.next_close {
                println!("  Next Close: {}", next_close);
            }
        }
        Err(e) => {
            println!("  Error getting clock: {}", e);
        }
    }

    // Create WebSocket client for real-time data
    println!("\n--- WebSocket Client Setup ---");
    let ws_client = AlpacaWebSocketClient::from_env(environment)?;
    println!("WebSocket client created");

    // Define symbols to monitor
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string()];
    println!("Symbols to monitor: {:?}", symbols);

    // Create subscription for real-time quotes
    let subscription = SubscribeMessage {
        trades: Some(symbols.clone()),
        quotes: Some(symbols.clone()),
        bars: None,
        trade_updates: None,
    };

    // Integration workflow description
    println!("\n--- Integration Workflow ---");
    println!("1. Use HTTP client to:");
    println!("   - Check account status and buying power");
    println!("   - Verify market is open");
    println!("   - Get current positions");
    println!("   - Submit orders");
    println!();
    println!("2. Use WebSocket client to:");
    println!("   - Stream real-time quotes for entry signals");
    println!("   - Monitor trade executions");
    println!("   - React to price movements");
    println!();
    println!("3. Combined workflow:");
    println!("   - WebSocket detects price signal");
    println!("   - HTTP submits order");
    println!("   - WebSocket confirms execution");

    // Example: Get positions before trading
    println!("\n--- Current Positions (HTTP) ---");
    match http_client.get_positions().await {
        Ok(positions) => {
            if positions.is_empty() {
                println!("  No open positions");
            } else {
                for pos in positions.iter().take(5) {
                    println!(
                        "  {} - {} shares @ ${} (P&L: ${})",
                        pos.symbol, pos.qty, pos.avg_entry_price, pos.unrealized_pl
                    );
                }
            }
        }
        Err(e) => {
            println!("  Error getting positions: {}", e);
        }
    }

    // Note about WebSocket connection
    println!("\n--- WebSocket Streaming ---");
    println!("To connect and stream data:");
    println!("  let stream = ws_client.subscribe_market_data(subscription).await?;");
    println!("  while let Some(update) = stream.next().await {{");
    println!("      // Process real-time updates");
    println!("      // Trigger HTTP orders based on signals");
    println!("  }}");

    println!("\n=== Example Complete ===");
    Ok(())
}
