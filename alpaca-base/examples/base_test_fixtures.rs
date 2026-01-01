//! # Test Fixtures
//!
//! This example demonstrates how to use test fixtures for unit testing.
//! Requires the `test-utils` feature.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_test_fixtures --features test-utils
//! ```

#[cfg(feature = "test-utils")]
use alpaca_base::test_utils::{fixtures, json_samples};

fn main() {
    println!("=== Test Fixtures ===\n");

    #[cfg(feature = "test-utils")]
    {
        // 1. Sample Account
        println!("--- Sample Account ---");
        let account = fixtures::sample_account();
        println!("  Account Number: {}", account.account_number);
        println!("  Status: {:?}", account.status);
        println!("  Buying Power: ${}", account.buying_power);
        println!("  Pattern Day Trader: {}", account.pattern_day_trader);

        // 2. Sample Asset
        println!("\n--- Sample Asset ---");
        let asset = fixtures::sample_asset("AAPL");
        println!("  Symbol: {}", asset.symbol);
        println!("  Name: {}", asset.name);
        println!("  Class: {:?}", asset.class);
        println!("  Tradable: {}", asset.tradable);

        // 3. Sample Order
        println!("\n--- Sample Order ---");
        let order = fixtures::sample_order("AAPL", alpaca_base::OrderSide::Buy, "100");
        println!("  Symbol: {}", order.symbol);
        println!("  Side: {:?}", order.side);
        println!("  Qty: {:?}", order.qty);
        println!("  Status: {:?}", order.status);

        // 4. Sample Position
        println!("\n--- Sample Position ---");
        let position = fixtures::sample_position("AAPL", "100", "150.00");
        println!("  Symbol: {}", position.symbol);
        println!("  Qty: {}", position.qty);
        println!("  Avg Entry: ${}", position.avg_entry_price);
        println!("  Unrealized P/L: ${}", position.unrealized_pl);

        // 5. Sample Market Data
        println!("\n--- Sample Market Data ---");
        let bar = fixtures::sample_bar(chrono::Utc::now());
        println!(
            "  Bar: O={:.2} H={:.2} L={:.2} C={:.2} V={}",
            bar.open, bar.high, bar.low, bar.close, bar.volume
        );

        let quote = fixtures::sample_quote(chrono::Utc::now());
        println!(
            "  Quote: Bid={:.2}x{} Ask={:.2}x{}",
            quote.bid_price, quote.bid_size, quote.ask_price, quote.ask_size
        );

        let trade = fixtures::sample_trade(chrono::Utc::now());
        println!("  Trade: Price={:.2} Size={}", trade.price, trade.size);

        // 6. Sample Clock
        println!("\n--- Sample Clock ---");
        let clock_open = fixtures::sample_clock(true);
        println!("  Market Open: is_open={}", clock_open.is_open);

        let clock_closed = fixtures::sample_clock(false);
        println!("  Market Closed: is_open={}", clock_closed.is_open);

        // 7. JSON Samples
        println!("\n--- JSON Samples ---");
        println!("  ACCOUNT_JSON: {} bytes", json_samples::ACCOUNT_JSON.len());
        println!("  ORDER_JSON: {} bytes", json_samples::ORDER_JSON.len());
        println!("  ASSET_JSON: {} bytes", json_samples::ASSET_JSON.len());
        println!("  ERROR_JSON: {} bytes", json_samples::ERROR_JSON.len());
    }

    #[cfg(not(feature = "test-utils"))]
    {
        println!("  This example requires the 'test-utils' feature.");
        println!(
            "  Run with: cargo run -p alpaca-base --example base_test_fixtures --features test-utils"
        );
    }

    println!("\n=== Example Complete ===");
}
