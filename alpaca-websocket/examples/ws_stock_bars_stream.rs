//! # Stock Bars Stream
//!
//! This example demonstrates how to stream real-time stock bars via WebSocket.
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
//! cargo run -p alpaca-websocket --example ws_stock_bars_stream
//! ```
//!
//! **Note**: This example connects to live market data. Bars are typically
//! delivered at minute intervals during market hours.

use alpaca_base::Environment;
use alpaca_websocket::{AlpacaWebSocketClient, SubscribeMessage};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Stock Bars Stream ===\n");

    // Create WebSocket client for Paper trading
    let client = AlpacaWebSocketClient::from_env(Environment::Paper)?;
    println!("WebSocket client created for Paper trading environment");

    // Define symbols to subscribe to
    let symbols = vec!["AAPL".to_string(), "SPY".to_string()];
    println!("Subscribing to bars for: {:?}", symbols);

    // Create subscription message
    let subscription = SubscribeMessage {
        trades: None,
        quotes: None,
        bars: Some(symbols.clone()),
        trade_updates: None,
    };

    // Connect and subscribe
    println!("\nConnecting to WebSocket...");
    let stream = client.subscribe_market_data(subscription).await?;
    println!("Connected! Waiting for bars...\n");

    // Process incoming bars
    println!("--- Live Bars (1-minute) ---");
    println!("(Press Ctrl+C to stop)\n");
    println!("Note: Bars are delivered at the end of each minute interval.");

    let mut stream = stream;
    let mut bar_count = 0;

    while let Some(update) = stream.next().await {
        match update {
            alpaca_websocket::MarketDataUpdate::Bar { symbol, bar } => {
                bar_count += 1;
                println!("[{}] {} @ {}", bar_count, symbol, bar.timestamp);
                println!(
                    "    O: ${:.2} | H: ${:.2} | L: ${:.2} | C: ${:.2} | V: {}",
                    bar.open, bar.high, bar.low, bar.close, bar.volume
                );
                if let Some(vwap) = bar.vwap {
                    println!("    VWAP: ${:.2}", vwap);
                }
                println!();

                // Stop after 5 bars for demo
                if bar_count >= 5 {
                    println!("Received 5 bars, stopping demo.");
                    break;
                }
            }
            _ => {
                // Ignore non-bar updates
            }
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
