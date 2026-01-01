//! # List Orders
//!
//! This example demonstrates how to list and filter orders.
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
//! cargo run -p alpaca-http --example http_list_orders
//! ```

use alpaca_base::{Environment, OrderQueryStatus};
use alpaca_http::{AlpacaHttpClient, OrderParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== List Orders ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // List all open orders
    println!("\n--- Open Orders ---");
    let open_params = OrderParams::new().status(OrderQueryStatus::Open).limit(10);

    match client.get_orders(&open_params).await {
        Ok(orders) => {
            if orders.is_empty() {
                println!("  No open orders found.");
            } else {
                println!("  Found {} open order(s):", orders.len());
                for order in &orders {
                    println!();
                    println!("    Order ID: {}", order.id);
                    println!("    Symbol: {}", order.symbol);
                    println!("    Side: {:?}", order.side);
                    println!("    Type: {:?}", order.order_type);
                    println!("    Qty: {:?}", order.qty);
                    println!("    Status: {:?}", order.status);
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing open orders: {}", e);
        }
    }

    // List recent closed orders
    println!("\n--- Recent Closed Orders ---");
    let closed_params = OrderParams::new().status(OrderQueryStatus::Closed).limit(5);

    match client.get_orders(&closed_params).await {
        Ok(orders) => {
            if orders.is_empty() {
                println!("  No closed orders found.");
            } else {
                println!("  Found {} closed order(s):", orders.len());
                for order in &orders {
                    println!();
                    println!("    Order ID: {}", order.id);
                    println!("    Symbol: {}", order.symbol);
                    println!("    Side: {:?}", order.side);
                    println!("    Status: {:?}", order.status);
                    println!("    Filled Qty: {}", order.filled_qty);
                    if let Some(price) = &order.filled_avg_price {
                        println!("    Filled Price: ${}", price);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing closed orders: {}", e);
        }
    }

    // List all orders (open and closed)
    println!("\n--- All Orders (last 10) ---");
    let all_params = OrderParams::new().status(OrderQueryStatus::All).limit(10);

    match client.get_orders(&all_params).await {
        Ok(orders) => {
            println!("  Total orders retrieved: {}", orders.len());

            // Count by status
            let open_count = orders
                .iter()
                .filter(|o| {
                    matches!(
                        o.status,
                        alpaca_base::OrderStatus::New
                            | alpaca_base::OrderStatus::Accepted
                            | alpaca_base::OrderStatus::PendingNew
                    )
                })
                .count();
            let filled_count = orders
                .iter()
                .filter(|o| matches!(o.status, alpaca_base::OrderStatus::Filled))
                .count();
            let canceled_count = orders
                .iter()
                .filter(|o| matches!(o.status, alpaca_base::OrderStatus::Canceled))
                .count();

            println!("  Open: {}", open_count);
            println!("  Filled: {}", filled_count);
            println!("  Canceled: {}", canceled_count);
        }
        Err(e) => {
            eprintln!("Error listing all orders: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
