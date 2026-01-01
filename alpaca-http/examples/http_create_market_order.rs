//! # Create Market Order
//!
//! This example demonstrates how to place a simple market order.
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
//! cargo run -p alpaca-http --example http_create_market_order
//! ```
//!
//! **WARNING**: This example places a real order on your Paper account!

use alpaca_base::{Environment, OrderSide};
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Create Market Order ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // Create a market order to buy 1 share of AAPL
    let order_request = CreateOrderRequest::market("AAPL", OrderSide::Buy, "1");

    println!("\n--- Order Request ---");
    println!("  Symbol: AAPL");
    println!("  Side: Buy");
    println!("  Type: Market");
    println!("  Qty: 1");

    println!("\nSubmitting order...");
    match client.create_order(&order_request).await {
        Ok(order) => {
            println!("\n--- Order Created ---");
            println!("  Order ID: {}", order.id);
            println!("  Client Order ID: {}", order.client_order_id);
            println!("  Symbol: {}", order.symbol);
            println!("  Side: {:?}", order.side);
            println!("  Type: {:?}", order.order_type);
            println!("  Qty: {:?}", order.qty);
            println!("  Status: {:?}", order.status);
            println!("  Time in Force: {:?}", order.time_in_force);
            println!("  Created At: {}", order.created_at);

            if let Some(filled_at) = order.filled_at {
                println!("  Filled At: {}", filled_at);
            }
            if let Some(filled_avg_price) = &order.filled_avg_price {
                println!("  Filled Avg Price: ${}", filled_avg_price);
            }
        }
        Err(e) => {
            eprintln!("Error creating order: {}", e);

            // Check if it's a retryable error
            if e.is_retryable() {
                eprintln!("This error is retryable. Consider implementing retry logic.");
            }

            return Err(e.into());
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
