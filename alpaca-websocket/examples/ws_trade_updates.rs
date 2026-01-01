//! # Trade Updates Stream
//!
//! This example demonstrates how to stream real-time trade/order updates via WebSocket.
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
//! cargo run -p alpaca-websocket --example ws_trade_updates
//! ```
//!
//! **Note**: This example streams order execution updates. Place orders via the
//! HTTP API to see updates here.

use alpaca_base::Environment;
use alpaca_websocket::AlpacaWebSocketClient;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Trade Updates Stream ===\n");

    // Create trading WebSocket client
    let credentials = alpaca_base::Credentials::from_env()?;
    let client = AlpacaWebSocketClient::trading(credentials, Environment::Paper);
    println!("Trading WebSocket client created for Paper trading environment");

    // Connect and subscribe to trade updates
    println!("\nConnecting to trading WebSocket...");
    let stream = client.subscribe_trading_updates().await?;
    println!("Connected! Waiting for trade updates...\n");

    // Process incoming trade updates
    println!("--- Trade Updates ---");
    println!("(Place orders via HTTP API to see updates here)");
    println!("(Press Ctrl+C to stop)\n");

    let mut stream = stream;
    let mut update_count = 0;

    while let Some(update) = stream.next().await {
        update_count += 1;

        println!("[{}] Trade Update Received:", update_count);
        println!("    Event: {:?}", update.event);
        println!("    Order ID: {}", update.order.id);
        println!("    Symbol: {}", update.order.symbol);
        println!("    Side: {:?}", update.order.side);
        println!("    Type: {:?}", update.order.order_type);
        println!("    Status: {:?}", update.order.status);

        if let Some(qty) = &update.order.qty {
            println!("    Qty: {}", qty);
        }
        println!("    Filled Qty: {}", update.order.filled_qty);

        if let Some(price) = &update.order.filled_avg_price {
            println!("    Filled Avg Price: ${}", price);
        }

        println!("    Timestamp: {}", update.timestamp);

        println!();

        // Stop after 10 updates for demo
        if update_count >= 10 {
            println!("Received 10 updates, stopping demo.");
            break;
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
