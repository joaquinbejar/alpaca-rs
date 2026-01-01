//! # Close Position
//!
//! This example demonstrates how to close positions.
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
//! cargo run -p alpaca-http --example http_close_position
//! ```
//!
//! **WARNING**: This example closes real positions on your Paper account!

use alpaca_base::Environment;
use alpaca_http::{AlpacaHttpClient, ClosePositionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Close Position ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // First, list positions to find one to close
    println!("\n--- Current Positions ---");
    let positions = client.get_positions().await?;

    if positions.is_empty() {
        println!("  No open positions to close.");
        println!("  Execute some orders first to create positions.");
        println!("\n=== Example Complete ===");
        return Ok(());
    }

    println!("  Found {} position(s):", positions.len());
    for position in &positions {
        println!(
            "    {} - {} shares @ ${}",
            position.symbol, position.qty, position.current_price
        );
    }

    // Close the first position
    let position_to_close = &positions[0];
    println!("\n--- Closing Position ---");
    println!("  Symbol: {}", position_to_close.symbol);
    println!("  Qty: {}", position_to_close.qty);
    println!("  Market Value: ${}", position_to_close.market_value);

    // Close entire position
    let close_request = ClosePositionRequest::new();

    match client
        .close_position(&position_to_close.symbol, &close_request)
        .await
    {
        Ok(order) => {
            println!("\n  ✓ Position close order submitted!");
            println!("    Order ID: {}", order.id);
            println!("    Side: {:?}", order.side);
            println!("    Type: {:?}", order.order_type);
            println!("    Status: {:?}", order.status);
        }
        Err(e) => {
            eprintln!("\n  ✗ Error closing position: {}", e);
        }
    }

    // Demonstrate close all positions
    println!("\n--- Close All Positions (Demo) ---");
    println!("  Skipping close_all_positions() to preserve your positions.");
    println!("  To close all positions, use:");
    println!("    client.close_all_positions(true).await?;");
    println!("  The boolean parameter cancels open orders if true.");

    // Demonstrate partial close
    println!("\n--- Partial Close (Demo) ---");
    println!("  To close only part of a position:");
    println!("    let request = ClosePositionRequest::new().qty(\"50\");");
    println!("    client.close_position(\"AAPL\", &request).await?;");

    println!("\n=== Example Complete ===");
    Ok(())
}
