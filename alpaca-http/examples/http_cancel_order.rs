//! # Cancel Order
//!
//! This example demonstrates how to cancel orders.
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
//! cargo run -p alpaca-http --example http_cancel_order
//! ```
//!
//! **WARNING**: This example cancels real orders on your Paper account!

use alpaca_base::{Environment, OrderQueryStatus};
use alpaca_http::{AlpacaHttpClient, OrderParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cancel Order ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // First, list open orders to find one to cancel
    println!("\n--- Finding Open Orders ---");
    let params = OrderParams::new().status(OrderQueryStatus::Open).limit(5);

    let orders = client.get_orders(&params).await?;

    if orders.is_empty() {
        println!("  No open orders to cancel.");
        println!("  Run http_create_limit_order first to create an order.");
        println!("\n=== Example Complete ===");
        return Ok(());
    }

    println!("  Found {} open order(s)", orders.len());

    // Cancel the first open order
    let order_to_cancel = &orders[0];
    println!("\n--- Canceling Order ---");
    println!("  Order ID: {}", order_to_cancel.id);
    println!("  Symbol: {}", order_to_cancel.symbol);
    println!("  Side: {:?}", order_to_cancel.side);
    println!("  Status: {:?}", order_to_cancel.status);

    match client.cancel_order(&order_to_cancel.id).await {
        Ok(()) => {
            println!("\n  ✓ Order canceled successfully!");
        }
        Err(e) => {
            eprintln!("\n  ✗ Error canceling order: {}", e);

            // Check specific error types
            if let Some(status) = e.status_code() {
                match status {
                    404 => eprintln!("    Order not found (may already be filled or canceled)"),
                    422 => eprintln!("    Order cannot be canceled (may be in terminal state)"),
                    _ => {}
                }
            }
        }
    }

    // Demonstrate cancel all orders
    println!("\n--- Cancel All Orders (Demo) ---");
    println!("  Skipping cancel_all_orders() to preserve your orders.");
    println!("  To cancel all orders, use:");
    println!("    client.cancel_all_orders().await?;");

    // Verify the order was canceled
    println!("\n--- Verifying Cancellation ---");
    match client.get_order(&order_to_cancel.id).await {
        Ok(order) => {
            println!("  Order Status: {:?}", order.status);
            if let Some(canceled_at) = order.canceled_at {
                println!("  Canceled At: {}", canceled_at);
            }
        }
        Err(e) => {
            eprintln!("  Error fetching order: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
