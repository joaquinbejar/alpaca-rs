//! # Bar Parameters Builder
//!
//! This example demonstrates how to build market data query parameters
//! using the builder pattern for stocks, crypto, and options.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_bar_params_builder
//! ```
//!
//! ## Expected Output
//!
//! Demonstrates MultiBarsParams, CryptoBarsParams, and OptionBarsParams builders.

use alpaca_base::{CryptoBarsParams, DataFeed, MultiBarsParams, OptionBarsParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Market Data Parameters Builder ===\n");

    // 1. Stock Bars Parameters
    println!("--- Stock Bars (MultiBarsParams) ---");
    demonstrate_stock_bars()?;

    // 2. Crypto Bars Parameters
    println!("\n--- Crypto Bars (CryptoBarsParams) ---");
    demonstrate_crypto_bars()?;

    // 3. Option Bars Parameters
    println!("\n--- Option Bars (OptionBarsParams) ---");
    demonstrate_option_bars()?;

    // 4. Data Feeds
    println!("\n--- Data Feeds ---");
    demonstrate_data_feeds();

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_stock_bars() -> Result<(), Box<dyn std::error::Error>> {
    // Basic usage - single symbol
    let params = MultiBarsParams::new("AAPL");
    println!("  Single symbol: {:?}", params.symbols);

    // Multiple symbols with all options
    let params = MultiBarsParams::new("AAPL,MSFT,GOOGL")
        .timeframe("1Day")
        .time_range("2024-01-01", "2024-03-01")
        .feed(DataFeed::Sip)
        .limit(1000);

    println!("\n  Full configuration:");
    println!("    Symbols: {:?}", params.symbols);
    println!("    Timeframe: {:?}", params.timeframe);
    println!("    Start: {:?}", params.start);
    println!("    End: {:?}", params.end);
    println!("    Feed: {:?}", params.feed);
    println!("    Limit: {:?}", params.limit);

    // Serialize to see query parameters
    let json = serde_json::to_string_pretty(&params)?;
    println!("\n  JSON representation:\n{}", json);

    Ok(())
}

fn demonstrate_crypto_bars() -> Result<(), Box<dyn std::error::Error>> {
    // Crypto uses different symbol format (e.g., BTC/USD)
    let params = CryptoBarsParams::new("BTC/USD,ETH/USD")
        .timeframe("1Hour")
        .limit(100);

    println!("  Crypto configuration:");
    println!("    Symbols: {:?}", params.symbols);
    println!("    Timeframe: {:?}", params.timeframe);
    println!("    Limit: {:?}", params.limit);

    // Common crypto timeframes
    println!("\n  Common crypto timeframes:");
    let timeframes = ["1Min", "5Min", "15Min", "1Hour", "4Hour", "1Day"];
    for tf in timeframes {
        println!("    - {}", tf);
    }

    Ok(())
}

fn demonstrate_option_bars() -> Result<(), Box<dyn std::error::Error>> {
    // Option symbols use OCC format
    // Format: SYMBOL + YYMMDD + C/P + Strike (8 digits with leading zeros)
    // Example: AAPL240315C00150000 = AAPL March 15, 2024 $150 Call
    let option_symbol = "AAPL240315C00150000";

    let params = OptionBarsParams::new(option_symbol)
        .timeframe("1Day")
        .time_range("2024-01-01", "2024-03-01")
        .limit(100);

    println!("  Option configuration:");
    println!("    Symbol: {:?}", params.symbols);
    println!("    Timeframe: {:?}", params.timeframe);
    println!("    Start: {:?}", params.start);
    println!("    End: {:?}", params.end);
    println!("    Limit: {:?}", params.limit);

    // Explain OCC symbol format
    println!("\n  OCC Symbol Format:");
    println!("    AAPL240315C00150000");
    println!("    ├── AAPL      = Underlying symbol");
    println!("    ├── 240315    = Expiration (YYMMDD)");
    println!("    ├── C         = Call (P for Put)");
    println!("    └── 00150000  = Strike price ($150.00)");

    Ok(())
}

fn demonstrate_data_feeds() {
    let feeds = [
        (DataFeed::Iex, "IEX Exchange - free, delayed data"),
        (DataFeed::Sip, "SIP - consolidated feed, real-time (paid)"),
    ];

    for (feed, description) in &feeds {
        println!("  {:?}: {}", feed, description);
    }

    // Serialize feeds
    println!("\n  JSON serialization:");
    for (feed, _) in &feeds {
        if let Ok(json) = serde_json::to_string(&feed) {
            println!("    {:?} -> {}", feed, json);
        }
    }
}
