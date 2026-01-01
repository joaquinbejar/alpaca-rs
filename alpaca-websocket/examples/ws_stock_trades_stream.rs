//! # Stock Trades Stream
//!
//! This example demonstrates how to stream real-time stock trades via WebSocket.
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
//! cargo run -p alpaca-websocket --example ws_stock_trades_stream
//! ```
//!
//! **Note**: This example connects to live market data. Run during market hours
//! for best results.

use alpaca_base::Environment;
use alpaca_websocket::{AlpacaWebSocketClient, SubscribeMessage};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Stock Trades Stream ===\n");

    // Create WebSocket client for Paper trading
    let client = AlpacaWebSocketClient::from_env(Environment::Paper)?;
    println!("WebSocket client created for Paper trading environment");

    // Define symbols to subscribe to
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string(), "GOOGL".to_string()];
    println!("Subscribing to trades for: {:?}", symbols);

    // Create subscription message
    let subscription = SubscribeMessage {
        trades: Some(symbols.clone()),
        quotes: None,
        bars: None,
        trade_updates: None,
    };

    // Connect and subscribe
    println!("\nConnecting to WebSocket...");
    let stream = client.subscribe_market_data(subscription).await?;
    println!("Connected! Waiting for trades...\n");

    // Process incoming trades
    println!("--- Live Trades ---");
    println!("(Press Ctrl+C to stop)\n");

    let mut stream = stream;
    let mut trade_count = 0;

    while let Some(update) = stream.next().await {
        match update {
            alpaca_websocket::MarketDataUpdate::Trade { symbol, trade } => {
                trade_count += 1;
                println!(
                    "[{}] {} - ${:.2} x {} @ {}",
                    trade_count, symbol, trade.price, trade.size, trade.timestamp
                );

                // Stop after 10 trades for demo
                if trade_count >= 10 {
                    println!("\nReceived 10 trades, stopping demo.");
                    break;
                }
            }
            _ => {
                // Ignore non-trade updates
            }
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
