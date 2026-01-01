//! # Crypto Stream
//!
//! This example demonstrates how to stream real-time crypto data via WebSocket.
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
//! cargo run -p alpaca-websocket --example ws_crypto_stream
//! ```
//!
//! **Note**: Crypto markets are open 24/7, so this example should work anytime.

use alpaca_base::Environment;
use alpaca_websocket::{AlpacaWebSocketClient, SubscribeMessage};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Crypto Stream ===\n");

    // Create WebSocket client for Paper trading
    let client = AlpacaWebSocketClient::from_env(Environment::Paper)?;
    println!("WebSocket client created for Paper trading environment");

    // Define crypto symbols to subscribe to
    let symbols = vec!["BTC/USD".to_string(), "ETH/USD".to_string()];
    println!("Subscribing to crypto data for: {:?}", symbols);

    // Create subscription message for trades and quotes
    let subscription = SubscribeMessage {
        trades: Some(symbols.clone()),
        quotes: Some(symbols.clone()),
        bars: None,
        trade_updates: None,
    };

    // Connect and subscribe
    println!("\nConnecting to WebSocket...");
    let stream = client.subscribe_market_data(subscription).await?;
    println!("Connected! Waiting for crypto data...\n");

    // Process incoming data
    println!("--- Live Crypto Data ---");
    println!("(Press Ctrl+C to stop)\n");

    let mut stream = stream;
    let mut update_count = 0;

    while let Some(update) = stream.next().await {
        update_count += 1;

        match update {
            alpaca_websocket::MarketDataUpdate::Trade { symbol, trade } => {
                println!(
                    "[{}] TRADE {} - ${:.2} x {} @ {}",
                    update_count, symbol, trade.price, trade.size, trade.timestamp
                );
            }
            alpaca_websocket::MarketDataUpdate::Quote { symbol, quote } => {
                let spread = quote.ask_price - quote.bid_price;
                println!(
                    "[{}] QUOTE {} - Bid: ${:.2} | Ask: ${:.2} | Spread: ${:.2}",
                    update_count, symbol, quote.bid_price, quote.ask_price, spread
                );
            }
            alpaca_websocket::MarketDataUpdate::Bar { symbol, bar } => {
                println!(
                    "[{}] BAR {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
                    update_count, symbol, bar.open, bar.high, bar.low, bar.close
                );
            }
        }

        // Stop after 20 updates for demo
        if update_count >= 20 {
            println!("\nReceived 20 updates, stopping demo.");
            break;
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
