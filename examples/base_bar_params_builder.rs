//! # Bar Params Builder Example
//!
//! This example demonstrates how to build query parameters for historical
//! market data (bars) using the builder pattern in `alpaca-base`.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_bar_params_builder
//! ```

use alpaca_base::{MultiBarsParams, Timeframe};
use chrono::{Utc, Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Bar Params Builder ===\n");

    let end = Utc::now();
    let start = end - Duration::days(7);

    // Build Parameters for multiple symbols
    let params = MultiBarsParams::new()
        .symbols(vec!["AAPL", "MSFT", "GOOGL"])
        .timeframe(Timeframe::OneDay)
        .start(start)
        .end(end)
        .limit(100);

    println!("--- Multi-Symbol Bar Request ---");
    println!("  Symbols: AAPL, MSFT, GOOGL");
    println!("  Timeframe: {:?}", params.timeframe);
    println!("  Period: {} to {}", start.date_naive(), end.date_naive());
    println!("  Limit per symbol: {:?}", params.limit);

    // Serialization (as it would be sent over HTTP)
    println!("\n  Encoded query string (simulated):");
    // In actual use, alpaca-http handles the mapping of these params to the URL
    println!("  ?symbols=AAPL,MSFT,GOOGL&timeframe=1Day&limit=100&start={}", start.to_rfc3339());

    println!("\n=== Example Complete ===");
    Ok(())
}
