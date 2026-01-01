//! # Get Stock Bars
//!
//! This example demonstrates how to fetch historical bar (OHLCV) data.
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
//! cargo run -p alpaca-http --example http_get_bars
//! ```

use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;
use alpaca_http::endpoints::BarsParams;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Get Stock Bars ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    let symbol = "AAPL";

    // Get bars for the last 5 days
    println!("\n--- Daily Bars (Last 5 Days) ---");
    let end = Utc::now();
    let start = end - Duration::days(7);

    let params = BarsParams {
        start: Some(start),
        end: Some(end),
        timeframe: Some("1Day".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    match client.get_bars(symbol, &params).await {
        Ok(response) => {
            println!("  Symbol: {}", symbol);
            println!("  Bars retrieved: {}", response.bars.len());

            for bar in &response.bars {
                println!();
                println!("    Date: {}", bar.timestamp);
                println!("    Open: ${:.2}", bar.open);
                println!("    High: ${:.2}", bar.high);
                println!("    Low: ${:.2}", bar.low);
                println!("    Close: ${:.2}", bar.close);
                println!("    Volume: {}", bar.volume);
                if let Some(vwap) = bar.vwap {
                    println!("    VWAP: ${:.2}", vwap);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching bars: {}", e);
        }
    }

    // Get hourly bars
    println!("\n--- Hourly Bars (Last 10 Hours) ---");
    let params = BarsParams {
        start: Some(Utc::now() - Duration::hours(12)),
        end: Some(Utc::now()),
        timeframe: Some("1Hour".to_string()),
        limit: Some(10),
        ..Default::default()
    };

    match client.get_bars(symbol, &params).await {
        Ok(response) => {
            println!("  Hourly bars retrieved: {}", response.bars.len());
            for bar in response.bars.iter().take(3) {
                println!(
                    "    {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
                    bar.timestamp, bar.open, bar.high, bar.low, bar.close
                );
            }
            if response.bars.len() > 3 {
                println!("    ... and {} more", response.bars.len() - 3);
            }
        }
        Err(e) => {
            eprintln!("Error fetching hourly bars: {}", e);
        }
    }

    // Get latest bar
    println!("\n--- Latest Bar ---");
    match client.get_latest_bar(symbol).await {
        Ok(response) => {
            println!("  Symbol: {}", symbol);
            println!("  Timestamp: {}", response.bar.timestamp);
            println!("  Close: ${:.2}", response.bar.close);
            println!("  Volume: {}", response.bar.volume);
        }
        Err(e) => {
            eprintln!("Error fetching latest bar: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
