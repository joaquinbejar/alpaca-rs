//! # Asset Filtering Example
//!
//! This example demonstrates how to work with Asset types and
//! build asset query parameters using ListAssetsParams.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_asset_filtering
//! ```

use alpaca_base::{Asset, AssetClass, AssetStatus, ListAssetsParams};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Asset Filtering ===\n");

    // 1. Asset Classes
    println!("--- Asset Classes ---");
    let classes = [AssetClass::UsEquity, AssetClass::Crypto];
    for class in &classes {
        println!("  {:?}", class);
    }

    // 2. Asset Structure
    println!("\n--- Asset Structure ---");
    let asset = Asset {
        id: Uuid::new_v4(),
        class: AssetClass::UsEquity,
        exchange: "NASDAQ".to_string(),
        symbol: "AAPL".to_string(),
        name: "Apple Inc.".to_string(),
        status: AssetStatus::Active,
        tradable: true,
        marginable: true,
        shortable: true,
        easy_to_borrow: true,
        fractionable: true,
    };
    println!("  Sample: {} ({}) - Tradable: {}", asset.name, asset.symbol, asset.tradable);

    // 3. ListAssetsParams Builder
    println!("\n--- ListAssetsParams Builder ---");
    let params = ListAssetsParams::new()
        .status(AssetStatus::Active)
        .asset_class("us_equity");
    
    println!("  Query for active US equities: {:?}", params);

    // 4. Filtering In-Memory
    println!("\n--- In-Memory Filtering Logic ---");
    let sample_assets = [
        create_sample_asset("AAPL", true, true),
        create_sample_asset("MSFT", true, true),
        create_sample_asset("PENNY", true, false), // Not shortable
        create_sample_asset("OLD", false, false),  // Not tradable
    ];

    let shortable: Vec<_> = sample_assets
        .iter()
        .filter(|a| a.tradable && a.shortable)
        .collect();
        
    println!("  Shortable assets found: {:?}", 
        shortable.iter().map(|a| &a.symbol).collect::<Vec<_>>()
    );

    println!("\n=== Example Complete ===");
    Ok(())
}

fn create_sample_asset(symbol: &str, tradable: bool, shortable: bool) -> Asset {
    Asset {
        id: Uuid::new_v4(),
        class: AssetClass::UsEquity,
        exchange: "NASDAQ".to_string(),
        symbol: symbol.to_string(),
        name: format!("{} Inc.", symbol),
        status: if tradable { AssetStatus::Active } else { AssetStatus::Inactive },
        tradable,
        marginable: tradable,
        shortable,
        easy_to_borrow: shortable,
        fractionable: tradable,
    }
}
