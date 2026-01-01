//! # Get Stock Trades
//!
//! This example demonstrates how to fetch trade data.
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
//! cargo run -p alpaca-http --example http_get_trades
//! ```

use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;
use alpaca_http::endpoints::TradesParams;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Get Stock Trades ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    let symbol = "AAPL";

    // Get historical trades
    println!("\n--- Historical Trades ---");
    let params = TradesParams {
        start: Some(Utc::now() - Duration::minutes(15)),
        end: Some(Utc::now()),
        limit: Some(20),
        ..Default::default()
    };

    match client.get_trades(symbol, &params).await {
        Ok(response) => {
            println!("  Symbol: {}", symbol);
            println!("  Trades retrieved: {}", response.trades.len());

            if response.trades.is_empty() {
                println!("  No trades in the specified time range.");
                println!("  (Market may be closed)");
            } else {
                // Calculate some statistics
                let total_volume: u64 = response.trades.iter().map(|t| t.size as u64).sum();
                let avg_price: f64 = response.trades.iter().map(|t| t.price).sum::<f64>()
                    / response.trades.len() as f64;

                println!("\n  Statistics:");
                println!("    Total Volume: {} shares", total_volume);
                println!("    Average Price: ${:.2}", avg_price);

                println!("\n  Recent Trades:");
                for trade in response.trades.iter().take(5) {
                    println!(
                        "    {} - ${:.2} x {} @ {}",
                        trade.timestamp, trade.price, trade.size, trade.exchange
                    );
                }

                if response.trades.len() > 5 {
                    println!("    ... and {} more trades", response.trades.len() - 5);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching trades: {}", e);
        }
    }

    // Get latest trade
    println!("\n--- Latest Trade ---");
    match client.get_latest_trade(symbol).await {
        Ok(response) => {
            let trade = &response.trade;
            println!("  Symbol: {}", symbol);
            println!("  Timestamp: {}", trade.timestamp);
            println!("  Price: ${:.2}", trade.price);
            println!("  Size: {} shares", trade.size);
            println!("  Exchange: {}", trade.exchange);
        }
        Err(e) => {
            eprintln!("Error fetching latest trade: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
