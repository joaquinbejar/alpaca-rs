//! # FIX Market Data Request
//!
//! This example demonstrates FIX Market Data Request messages.
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
//! cargo run -p alpaca-fix --example fix_market_data_request
//! ```
//!
//! **Note**: This example demonstrates message structures.
//! FIX protocol requires special access from Alpaca.

use alpaca_fix::{MarketDataEntry, MarketDataRequest, MarketDataSnapshot};

fn main() {
    println!("=== FIX Market Data Request ===\n");

    // Create a snapshot request
    println!("--- Snapshot Request ---");
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string(), "GOOGL".to_string()];
    let snapshot_request = MarketDataRequest::snapshot(symbols.clone());

    println!("  Request ID: {}", snapshot_request.md_req_id);
    println!(
        "  Subscription Type: {} (Snapshot)",
        snapshot_request.subscription_request_type
    );
    println!("  Market Depth: {}", snapshot_request.market_depth);
    println!("  Symbols: {:?}", snapshot_request.symbols);

    // Create a subscription request
    println!("\n--- Subscription Request ---");
    let subscribe_request = MarketDataRequest::subscribe(symbols.clone());

    println!("  Request ID: {}", subscribe_request.md_req_id);
    println!(
        "  Subscription Type: {} (Subscribe)",
        subscribe_request.subscription_request_type
    );
    println!("  Market Depth: {}", subscribe_request.market_depth);
    println!("  Symbols: {:?}", subscribe_request.symbols);

    // Create an unsubscribe request
    println!("\n--- Unsubscribe Request ---");
    let unsubscribe_request = MarketDataRequest::unsubscribe(symbols);

    println!("  Request ID: {}", unsubscribe_request.md_req_id);
    println!(
        "  Subscription Type: {} (Unsubscribe)",
        unsubscribe_request.subscription_request_type
    );

    // Simulate a market data snapshot response
    println!("\n--- Market Data Snapshot Response ---");
    let snapshot = MarketDataSnapshot {
        md_req_id: snapshot_request.md_req_id.clone(),
        symbol: "AAPL".to_string(),
        entries: vec![
            MarketDataEntry {
                md_entry_type: '0', // Bid
                md_entry_px: 175.50,
                md_entry_size: 100.0,
            },
            MarketDataEntry {
                md_entry_type: '1', // Offer
                md_entry_px: 175.55,
                md_entry_size: 200.0,
            },
            MarketDataEntry {
                md_entry_type: '2', // Trade
                md_entry_px: 175.52,
                md_entry_size: 50.0,
            },
        ],
    };

    println!("  Request ID: {}", snapshot.md_req_id);
    println!("  Symbol: {}", snapshot.symbol);
    println!("  Entries:");
    for entry in &snapshot.entries {
        let entry_type_name = match entry.md_entry_type {
            '0' => "Bid",
            '1' => "Offer",
            '2' => "Trade",
            _ => "Unknown",
        };
        println!(
            "    {} - Price: ${:.2}, Size: {}",
            entry_type_name, entry.md_entry_px, entry.md_entry_size
        );
    }

    // Subscription type reference
    println!("\n--- Subscription Request Type (Tag 263) ---");
    println!("  0 = Snapshot");
    println!("  1 = Snapshot + Updates (Subscribe)");
    println!("  2 = Disable previous Snapshot + Updates (Unsubscribe)");

    // Market data entry type reference
    println!("\n--- MD Entry Type (Tag 269) ---");
    println!("  0 = Bid");
    println!("  1 = Offer");
    println!("  2 = Trade");
    println!("  4 = Opening Price");
    println!("  5 = Closing Price");
    println!("  7 = Trading Session High Price");
    println!("  8 = Trading Session Low Price");

    println!("\n=== Example Complete ===");
}
