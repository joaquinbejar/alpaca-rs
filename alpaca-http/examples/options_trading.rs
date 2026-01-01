//! Options Trading Example
//!
//! This example demonstrates how to use the Alpaca Options Trading API.
//!
//! # Prerequisites
//! - Alpaca account with options trading enabled
//! - API credentials in `.env` file
//!
//! # Environment Variables
//! - `ALPACA_API_KEY`: Your Alpaca API key
//! - `ALPACA_API_SECRET`: Your Alpaca API secret
//!
//! # Usage
//! ```bash
//! cargo run -p alpaca-http --example options_trading
//! ```

use alpaca_base::{Credentials, Environment, OptionContractParams, OptionType};
use alpaca_http::AlpacaHttpClient;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get credentials from environment
    let api_key = env::var("ALPACA_API_KEY").expect("ALPACA_API_KEY must be set");
    let api_secret = env::var("ALPACA_API_SECRET").expect("ALPACA_API_SECRET must be set");

    // Create credentials and client
    let credentials = Credentials::new(api_key, api_secret);
    let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;

    println!("=== Alpaca Options Trading Example ===\n");

    // Get account info
    let account = client.get_account().await?;
    println!("Account ID: {}", account.id);
    println!("Buying Power: ${}", account.buying_power);
    println!();

    // Example 1: List option contracts for AAPL
    println!("--- Listing AAPL Option Contracts ---");

    let params = OptionContractParams::new()
        .underlying_symbol("AAPL")
        .option_type(OptionType::Call)
        .limit(5);

    match client.get_option_contracts(&params).await {
        Ok(response) => {
            println!("Found {} contracts", response.option_contracts.len());
            for contract in response.option_contracts.iter().take(5) {
                println!(
                    "  {} - {} ${} exp {} ({:?})",
                    contract.symbol,
                    contract.underlying_symbol,
                    contract.strike_price,
                    contract.expiration_date,
                    contract.option_type
                );
            }
        }
        Err(e) => {
            println!("Error fetching contracts: {}", e);
            println!("Note: Options trading may not be enabled on this account");
        }
    }
    println!();

    // Example 2: Get a specific option contract (if we have a symbol)
    println!("--- Option Contract Details ---");
    // Example OCC symbol format: AAPL240315C00150000
    // (AAPL, March 15 2024, Call, $150 strike)
    let example_symbol = "AAPL240315C00150000";
    println!("Looking up contract: {}", example_symbol);

    match client.get_option_contract(example_symbol).await {
        Ok(contract) => {
            println!("Contract found:");
            println!("  Symbol: {}", contract.symbol);
            println!("  Name: {}", contract.name);
            println!("  Underlying: {}", contract.underlying_symbol);
            println!("  Strike: ${}", contract.strike_price);
            println!("  Expiration: {}", contract.expiration_date);
            println!("  Type: {:?}", contract.option_type);
            println!("  Style: {:?}", contract.style);
            println!("  Tradable: {}", contract.tradable);
        }
        Err(e) => {
            println!("Contract not found or error: {}", e);
        }
    }
    println!();

    // Example 3: Creating an options order
    println!("--- Options Order Example ---");
    println!("To create an options order, use CreateOrderRequest with position_intent:");
    println!();
    println!("  use alpaca_base::{{PositionIntent, OrderSide, OrderType}};");
    println!("  use alpaca_http::CreateOrderRequest;");
    println!();
    println!("  // Buy to open a call option");
    println!(
        "  let order = CreateOrderRequest::market(\"AAPL240315C00150000\", OrderSide::Buy, \"1\")"
    );
    println!("      .position_intent(PositionIntent::BuyToOpen);");
    println!();
    println!("  // Sell to close");
    println!(
        "  let order = CreateOrderRequest::limit(\"AAPL240315C00150000\", OrderSide::Sell, \"1\", \"5.00\")"
    );
    println!("      .position_intent(PositionIntent::SellToClose);");
    println!();

    println!("=== Example completed ===");
    Ok(())
}
