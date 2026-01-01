//! # Get Positions
//!
//! This example demonstrates how to list and get positions.
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
//! cargo run -p alpaca-http --example http_get_positions
//! ```

use dotenvy::dotenv;
use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    println!("=== Get Positions ===\n");
    
    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // Get all positions
    println!("\n--- All Positions ---");
    match client.get_positions().await {
        Ok(positions) => {
            if positions.is_empty() {
                println!("  No open positions.");
                println!("  Execute some orders to create positions.");
            } else {
                println!("  Found {} position(s):", positions.len());

                let mut total_market_value = 0.0;
                let mut total_unrealized_pl = 0.0;

                for position in &positions {
                    println!();
                    println!("  --- {} ---", position.symbol);
                    println!("    Side: {:?}", position.side);
                    println!("    Qty: {}", position.qty);
                    println!("    Avg Entry Price: ${}", position.avg_entry_price);
                    println!("    Current Price: ${}", position.current_price);
                    println!("    Market Value: ${}", position.market_value);
                    println!("    Cost Basis: ${}", position.cost_basis);
                    println!("    Unrealized P/L: ${}", position.unrealized_pl);
                    println!(
                        "    Unrealized P/L %%: {}%",
                        (position.unrealized_plpc.parse::<f64>().unwrap_or(0.0) * 100.0)
                    );
                    println!(
                        "    Today's Change: {}%",
                        (position.change_today.parse::<f64>().unwrap_or(0.0) * 100.0)
                    );

                    total_market_value += position.market_value.parse::<f64>().unwrap_or(0.0);
                    total_unrealized_pl += position.unrealized_pl.parse::<f64>().unwrap_or(0.0);
                }

                println!("\n  --- Portfolio Summary ---");
                println!("    Total Market Value: ${:.2}", total_market_value);
                println!("    Total Unrealized P/L: ${:.2}", total_unrealized_pl);
            }
        }
        Err(e) => {
            eprintln!("Error fetching positions: {}", e);
            return Err(e.into());
        }
    }

    // Get a specific position (if we have any)
    println!("\n--- Get Specific Position ---");
    println!("  To get a specific position by symbol:");
    println!("    let position = client.get_position(\"AAPL\").await?;");

    println!("\n=== Example Complete ===");
    Ok(())
}
