//! # Create Limit Order
//!
//! This example demonstrates how to place a limit order.
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
//! cargo run -p alpaca-http --example http_create_limit_order
//! ```
//!
//! **WARNING**: This example places a real order on your Paper account!

use alpaca_base::{Environment, OrderSide, TimeInForce};
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Create Limit Order ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // Create a limit order to buy 1 share of AAPL at $150
    let order_request = CreateOrderRequest::limit("AAPL", OrderSide::Buy, "1", "150.00")
        .time_in_force(TimeInForce::Gtc);

    println!("\n--- Order Request ---");
    println!("  Symbol: AAPL");
    println!("  Side: Buy");
    println!("  Type: Limit");
    println!("  Qty: 1");
    println!("  Limit Price: $150.00");
    println!("  Time in Force: GTC (Good 'til Canceled)");

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
            println!("  Limit Price: {:?}", order.limit_price);
            println!("  Status: {:?}", order.status);
            println!("  Time in Force: {:?}", order.time_in_force);
            println!("  Created At: {}", order.created_at);

            // Note: Limit orders may not fill immediately
            println!("\n  Note: Limit orders fill when market price reaches limit price.");
            println!("  Check order status later or use WebSocket for real-time updates.");
        }
        Err(e) => {
            eprintln!("Error creating order: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
