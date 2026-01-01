//! # Get Market Clock
//!
//! This example demonstrates how to get the current market clock status.
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
//! cargo run -p alpaca-http --example http_get_clock
//! ```

use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Get Market Clock ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // Get market clock
    println!("\n--- Market Clock ---");
    match client.get_clock().await {
        Ok(clock) => {
            println!("  Current Time: {}", clock.timestamp);
            println!(
                "  Market Open: {}",
                if clock.is_open { "YES" } else { "NO" }
            );
            println!("  Next Open: {}", clock.next_open);
            println!("  Next Close: {}", clock.next_close);

            // Calculate time until next event
            let now = clock.timestamp;
            if clock.is_open {
                let time_to_close = clock.next_close.signed_duration_since(now);
                let hours = time_to_close.num_hours();
                let minutes = time_to_close.num_minutes() % 60;
                println!("\n  Time until close: {}h {}m", hours, minutes);
            } else {
                let time_to_open = clock.next_open.signed_duration_since(now);
                let hours = time_to_open.num_hours();
                let minutes = time_to_open.num_minutes() % 60;
                println!("\n  Time until open: {}h {}m", hours, minutes);
            }

            // Trading session info
            println!("\n  Trading Session:");
            if clock.is_open {
                println!("    Status: Regular trading hours");
                println!("    You can place orders that will execute immediately.");
            } else {
                println!("    Status: Market closed");
                println!("    Orders placed now will queue for next open.");
                println!("    Extended hours trading may be available.");
            }
        }
        Err(e) => {
            eprintln!("Error fetching clock: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
