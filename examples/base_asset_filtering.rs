//! # Asset Filtering
//!
//! This example demonstrates how to work with Asset types and
//! build asset query parameters using ListAssetsParams.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_asset_filtering
//! ```
//!
//! ## Expected Output
//!
//! Demonstrates Asset, AssetClass, AssetStatus, and ListAssetsParams.

use alpaca_base::{Asset, AssetClass, AssetStatus, ListAssetsParams};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Asset Filtering ===\n");

    // 1. Asset Classes
    println!("--- Asset Classes ---");
    demonstrate_asset_classes();

    // 2. Asset Status
    println!("\n--- Asset Status ---");
    demonstrate_asset_status();

    // 3. Asset Structure
    println!("\n--- Asset Structure ---");
    demonstrate_asset_structure();

    // 4. ListAssetsParams Builder
    println!("\n--- ListAssetsParams Builder ---");
    demonstrate_list_params()?;

    // 5. Filtering Examples
    println!("\n--- Common Filtering Patterns ---");
    demonstrate_filtering_patterns()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_asset_classes() {
    let classes = [
        (AssetClass::UsEquity, "US stocks and ETFs"),
        (AssetClass::Crypto, "Cryptocurrencies"),
    ];

    for (class, description) in classes {
        println!("  {:?}: {}", class, description);
    }

    // Serialization
    println!("\n  JSON serialization:");
    for (class, _) in classes {
        if let Ok(json) = serde_json::to_string(&class) {
            println!("    {:?} -> {}", class, json);
        }
    }
}

fn demonstrate_asset_status() {
    let statuses = [
        (AssetStatus::Active, "Asset is tradable"),
        (AssetStatus::Inactive, "Asset is not tradable"),
    ];

    for (status, description) in statuses {
        println!("  {:?}: {}", status, description);
    }
}

fn demonstrate_asset_structure() {
    // Create a sample asset (as would be returned from API)
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

    println!("  Sample Asset:");
    println!("    Symbol: {}", asset.symbol);
    println!("    Name: {}", asset.name);
    println!("    Class: {:?}", asset.class);
    println!("    Exchange: {}", asset.exchange);
    println!("    Status: {:?}", asset.status);
    println!("    Tradable: {}", asset.tradable);
    println!("    Marginable: {}", asset.marginable);
    println!("    Shortable: {}", asset.shortable);
    println!("    Easy to borrow: {}", asset.easy_to_borrow);
    println!("    Fractionable: {}", asset.fractionable);
}

fn demonstrate_list_params() -> Result<(), Box<dyn std::error::Error>> {
    // Empty params (get all assets)
    let params = ListAssetsParams::new();
    println!("  Empty params: {:?}", params);

    // Filter by status
    let params = ListAssetsParams::new().status(AssetStatus::Active);
    println!("\n  Filter by active status:");
    let json = serde_json::to_string_pretty(&params)?;
    println!("{}", json);

    // Filter by asset class
    let params = ListAssetsParams::new().asset_class("us_equity");
    println!("\n  Filter by US equity class:");
    let json = serde_json::to_string_pretty(&params)?;
    println!("{}", json);

    // Filter by exchange
    let params = ListAssetsParams::new().exchange("NASDAQ");
    println!("\n  Filter by NASDAQ exchange:");
    let json = serde_json::to_string_pretty(&params)?;
    println!("{}", json);

    Ok(())
}

fn demonstrate_filtering_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("  1. Get all active US equities:");
    let params = ListAssetsParams::new()
        .status(AssetStatus::Active)
        .asset_class("us_equity");
    println!("     {:?}", params);

    println!("\n  2. Get all crypto assets:");
    let params = ListAssetsParams::new().asset_class("crypto");
    println!("     {:?}", params);

    println!("\n  3. Get NYSE assets only:");
    let params = ListAssetsParams::new()
        .status(AssetStatus::Active)
        .exchange("NYSE");
    println!("     {:?}", params);

    println!("\n  4. Filter assets in code (example):");
    let sample_assets = vec![
        create_sample_asset("AAPL", true, true),
        create_sample_asset("MSFT", true, true),
        create_sample_asset("PENNY", true, false),  // Not shortable
        create_sample_asset("OLD", false, false),   // Not tradable
    ];

    // Filter for tradable and shortable assets
    let shortable: Vec<_> = sample_assets
        .iter()
        .filter(|a| a.tradable && a.shortable)
        .collect();
    println!("     Tradable & shortable: {:?}", 
        shortable.iter().map(|a| &a.symbol).collect::<Vec<_>>());

    // Filter for fractionable assets
    let fractionable: Vec<_> = sample_assets
        .iter()
        .filter(|a| a.fractionable)
        .collect();
    println!("     Fractionable: {:?}",
        fractionable.iter().map(|a| &a.symbol).collect::<Vec<_>>());

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
