//! # List Assets
//!
//! This example demonstrates how to list and filter assets.
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
//! cargo run -p alpaca-http --example http_list_assets
//! ```

use alpaca_base::{AssetClass, AssetStatus, Environment};
use alpaca_http::AlpacaHttpClient;
use alpaca_http::endpoints::AssetParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== List Assets ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // List active US equities
    println!("\n--- Active US Equities ---");
    let params = AssetParams {
        status: Some(AssetStatus::Active),
        asset_class: Some(AssetClass::UsEquity),
        ..Default::default()
    };

    match client.get_assets(&params).await {
        Ok(assets) => {
            println!("  Total active US equities: {}", assets.len());

            // Show some statistics
            let tradable = assets.iter().filter(|a| a.tradable).count();
            let shortable = assets.iter().filter(|a| a.shortable).count();
            let fractionable = assets.iter().filter(|a| a.fractionable).count();

            println!("\n  Statistics:");
            println!("    Tradable: {}", tradable);
            println!("    Shortable: {}", shortable);
            println!("    Fractionable: {}", fractionable);

            // Show first 5 assets
            println!("\n  Sample Assets:");
            for asset in assets.iter().take(5) {
                println!("    {} - {} ({})", asset.symbol, asset.name, asset.exchange);
            }
        }
        Err(e) => {
            eprintln!("Error listing assets: {}", e);
        }
    }

    // Get a specific asset by symbol
    println!("\n--- Get Asset by Symbol ---");
    let symbol = "AAPL";
    match client.get_asset_by_symbol(symbol).await {
        Ok(asset) => {
            println!("  Symbol: {}", asset.symbol);
            println!("  Name: {}", asset.name);
            println!("  Class: {:?}", asset.class);
            println!("  Exchange: {}", asset.exchange);
            println!("  Status: {:?}", asset.status);
            println!("  Tradable: {}", asset.tradable);
            println!("  Marginable: {}", asset.marginable);
            println!("  Shortable: {}", asset.shortable);
            println!("  Easy to Borrow: {}", asset.easy_to_borrow);
            println!("  Fractionable: {}", asset.fractionable);
        }
        Err(e) => {
            eprintln!("Error fetching asset: {}", e);
        }
    }

    // List crypto assets
    println!("\n--- Crypto Assets ---");
    let crypto_params = AssetParams {
        status: Some(AssetStatus::Active),
        asset_class: Some(AssetClass::Crypto),
        ..Default::default()
    };

    match client.get_assets(&crypto_params).await {
        Ok(assets) => {
            println!("  Total crypto assets: {}", assets.len());
            println!("\n  Available Crypto:");
            for asset in assets.iter().take(10) {
                println!("    {} - {}", asset.symbol, asset.name);
            }
            if assets.len() > 10 {
                println!("    ... and {} more", assets.len() - 10);
            }
        }
        Err(e) => {
            eprintln!("Error listing crypto assets: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
