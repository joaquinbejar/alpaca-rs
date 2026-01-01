//! Enhanced Stock Market Data Example
//!
//! This example demonstrates how to use the enhanced stock market data API
//! including multi-symbol queries, snapshots, and corporate actions.
//!
//! # Prerequisites
//! - Alpaca account with market data access
//! - API credentials in `.env` file
//!
//! # Environment Variables
//! - `ALPACA_API_KEY`: Your Alpaca API key
//! - `ALPACA_API_SECRET`: Your Alpaca API secret
//!
//! # Usage
//! ```bash
//! cargo run -p alpaca-http --example market_data
//! ```

use alpaca_base::{CorporateActionsParams, Credentials, DataFeed, Environment, MultiBarsParams};
use alpaca_http::AlpacaHttpClient;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get credentials from environment
    let api_key = env::var("ALPACA_API_KEY").expect("ALPACA_API_KEY must be set");
    let api_secret = env::var("ALPACA_API_SECRET").expect("ALPACA_API_SECRET must be set");

    // Create credentials and client
    let credentials = Credentials::new(api_key, api_secret);
    let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;

    println!("=== Enhanced Stock Market Data Example ===\n");

    // Example 1: Get multi-symbol bars
    println!("--- Multi-Symbol Historical Bars ---");

    let params = MultiBarsParams::new("AAPL,MSFT,GOOGL")
        .timeframe("1Day")
        .feed(DataFeed::Iex)
        .limit(5);

    match client.get_stock_bars(&params).await {
        Ok(response) => {
            for (symbol, bars) in &response.bars {
                println!("{}:", symbol);
                for bar in bars.iter().take(3) {
                    println!(
                        "  {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}",
                        bar.timestamp, bar.open, bar.high, bar.low, bar.close, bar.volume
                    );
                }
            }
        }
        Err(e) => println!("Error fetching bars: {}", e),
    }
    println!();

    // Example 2: Get stock snapshots
    println!("--- Stock Snapshots ---");

    match client.get_stock_snapshots("AAPL,MSFT").await {
        Ok(response) => {
            for (symbol, snapshot) in &response.snapshots {
                println!("{}:", symbol);
                if let Some(trade) = &snapshot.latest_trade {
                    println!("  Latest Trade: ${:.2} x {}", trade.price, trade.size);
                }
                if let Some(quote) = &snapshot.latest_quote {
                    println!(
                        "  Latest Quote: Bid ${:.2} x {} / Ask ${:.2} x {}",
                        quote.bid_price, quote.bid_size, quote.ask_price, quote.ask_size
                    );
                }
                if let Some(bar) = &snapshot.daily_bar {
                    println!(
                        "  Daily Bar: O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
                        bar.open, bar.high, bar.low, bar.close
                    );
                }
            }
        }
        Err(e) => println!("Error fetching snapshots: {}", e),
    }
    println!();

    // Example 3: Get latest quotes
    println!("--- Latest Quotes ---");

    match client.get_latest_quotes("AAPL,MSFT,GOOGL").await {
        Ok(response) => {
            for (symbol, quote) in &response.quotes {
                println!(
                    "{}: Bid ${:.2} x {} / Ask ${:.2} x {}",
                    symbol, quote.bid_price, quote.bid_size, quote.ask_price, quote.ask_size
                );
            }
        }
        Err(e) => println!("Error fetching latest quotes: {}", e),
    }
    println!();

    // Example 4: Get corporate actions
    println!("--- Corporate Actions ---");

    let params = CorporateActionsParams::new()
        .symbols("AAPL")
        .types("dividend")
        .limit(5);

    match client.get_corporate_actions(&params).await {
        Ok(response) => {
            println!(
                "Found {} corporate actions",
                response.corporate_actions.len()
            );
            for action in response.corporate_actions.iter().take(5) {
                println!(
                    "  {:?} - Ex-date: {:?}, Cash: {:?}",
                    action.action_type, action.ex_date, action.cash
                );
            }
        }
        Err(e) => println!("Error fetching corporate actions: {}", e),
    }

    println!("\n=== Example completed ===");
    Ok(())
}
