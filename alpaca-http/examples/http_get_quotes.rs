//! # Get Stock Quotes
//!
//! This example demonstrates how to fetch quote data (bid/ask).
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
//! cargo run -p alpaca-http --example http_get_quotes
//! ```

use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;
use alpaca_http::endpoints::QuotesParams;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Get Stock Quotes ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    let symbol = "AAPL";

    // Get historical quotes
    println!("\n--- Historical Quotes ---");
    let params = QuotesParams {
        start: Some(Utc::now() - Duration::minutes(30)),
        end: Some(Utc::now()),
        limit: Some(10),
        ..Default::default()
    };

    match client.get_quotes(symbol, &params).await {
        Ok(response) => {
            println!("  Symbol: {}", symbol);
            println!("  Quotes retrieved: {}", response.quotes.len());

            for quote in response.quotes.iter().take(5) {
                println!();
                println!("    Time: {}", quote.timestamp);
                println!("    Bid: ${:.2} x {}", quote.bid_price, quote.bid_size);
                println!("    Ask: ${:.2} x {}", quote.ask_price, quote.ask_size);
                let spread = quote.ask_price - quote.bid_price;
                println!("    Spread: ${:.4}", spread);
            }

            if response.quotes.len() > 5 {
                println!("\n    ... and {} more quotes", response.quotes.len() - 5);
            }
        }
        Err(e) => {
            eprintln!("Error fetching quotes: {}", e);
        }
    }

    // Get latest quote
    println!("\n--- Latest Quote ---");
    match client.get_latest_quote(symbol).await {
        Ok(response) => {
            let quote = &response.quote;
            println!("  Symbol: {}", symbol);
            println!("  Timestamp: {}", quote.timestamp);
            println!("  Bid: ${:.2} x {}", quote.bid_price, quote.bid_size);
            println!("  Ask: ${:.2} x {}", quote.ask_price, quote.ask_size);

            let spread = quote.ask_price - quote.bid_price;
            let mid = (quote.bid_price + quote.ask_price) / 2.0;
            println!("  Mid Price: ${:.2}", mid);
            println!("  Spread: ${:.4} ({:.2}%)", spread, (spread / mid) * 100.0);
        }
        Err(e) => {
            eprintln!("Error fetching latest quote: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
