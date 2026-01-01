//! # Stock Quotes Stream
//!
//! This example demonstrates how to stream real-time stock quotes via WebSocket.
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
//! cargo run -p alpaca-websocket --example ws_stock_quotes_stream
//! ```
//!
//! **Note**: This example connects to live market data. Run during market hours
//! for best results.

use alpaca_base::Environment;
use alpaca_websocket::{AlpacaWebSocketClient, SubscribeMessage};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Stock Quotes Stream ===\n");

    // Create WebSocket client for Paper trading
    let client = AlpacaWebSocketClient::from_env(Environment::Paper)?;
    println!("WebSocket client created for Paper trading environment");

    // Define symbols to subscribe to
    let symbols = vec!["AAPL".to_string(), "TSLA".to_string()];
    println!("Subscribing to quotes for: {:?}", symbols);

    // Create subscription message
    let subscription = SubscribeMessage {
        trades: None,
        quotes: Some(symbols.clone()),
        bars: None,
        trade_updates: None,
    };

    // Connect and subscribe
    println!("\nConnecting to WebSocket...");
    let stream = client.subscribe_market_data(subscription).await?;
    println!("Connected! Waiting for quotes...\n");

    // Process incoming quotes
    println!("--- Live Quotes ---");
    println!("(Press Ctrl+C to stop)\n");

    let mut stream = stream;
    let mut quote_count = 0;

    while let Some(update) = stream.next().await {
        match update {
            alpaca_websocket::MarketDataUpdate::Quote { symbol, quote } => {
                quote_count += 1;
                let spread = quote.ask_price - quote.bid_price;
                let mid = (quote.bid_price + quote.ask_price) / 2.0;

                println!(
                    "[{}] {} - Bid: ${:.2} x {} | Ask: ${:.2} x {} | Spread: ${:.4} ({:.2}%)",
                    quote_count,
                    symbol,
                    quote.bid_price,
                    quote.bid_size,
                    quote.ask_price,
                    quote.ask_size,
                    spread,
                    (spread / mid) * 100.0
                );

                // Stop after 10 quotes for demo
                if quote_count >= 10 {
                    println!("\nReceived 10 quotes, stopping demo.");
                    break;
                }
            }
            _ => {
                // Ignore non-quote updates
            }
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
