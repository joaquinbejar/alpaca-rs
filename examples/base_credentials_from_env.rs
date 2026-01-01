//! # Credentials from Environment Example
//!
//! This example demonstrates how to load Alpaca API credentials from environment variables
//! using the `alpaca_base::auth::Credentials` and `alpaca_base::types::Environment` types.
//!
//! **Note**: The library now automatically loads `.env` files when calling `Credentials::from_env()`.
//!
//! ## Prerequisites
//!
//! - Set `ALPACA_API_KEY` environment variable.
//! - Set `ALPACA_API_SECRET` environment variable.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_credentials_from_env
//! ```

use alpaca_base::auth::Credentials;
use alpaca_base::types::Environment;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Alpaca Credentials Loading Example ---");

    // Method 1: Loading from environment variables automatically (Now with automatic .env loading!)
    println!("\nMethod 1: Credentials::from_env()");
    match Credentials::from_env() {
        Ok(creds) => {
            println!("✅ Successfully loaded credentials from environment (via .env or system env).");
            println!("API Key: {}...", &creds.api_key[..5]); // Only show prefix for security
        }
        Err(e) => {
            println!("❌ Failed to load credentials: {}", e);
            println!("   Make sure ALPACA_API_KEY and ALPACA_API_SECRET are set in your .env file or environment.");
        }
    }

    // Method 2: Manual construction
    println!("\nMethod 2: Manual construction");
    let api_key = env::var("ALPACA_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let secret_key = env::var("ALPACA_API_SECRET").unwrap_or_else(|_| "your_secret_key".to_string());
    
    let creds = Credentials::new(api_key, secret_key);
    println!("✅ Manually created credentials object.");

    // Demonstrating environment selection
    println!("\n--- Environment Selection ---");
    
    let paper = Environment::Paper;
    println!("Paper Base URL: {}", paper.base_url());
    let live = Environment::Live;
    println!("Live Base URL: {}", live.base_url());

    println!("\nDone.");
    Ok(())
}
