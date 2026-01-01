//! # Credentials from Environment Variables
//!
//! This example demonstrates how to load Alpaca API credentials from
//! environment variables using the `Credentials` type.
//!
//! ## Prerequisites
//!
//! Set the following environment variables:
//! - `ALPACA_API_KEY`: Your Alpaca API key
//! - `ALPACA_SECRET_KEY`: Your Alpaca secret key
//!
//! ## Usage
//!
//! ```bash
//! export ALPACA_API_KEY="your-api-key"
//! export ALPACA_SECRET_KEY="your-secret-key"
//! cargo run --example base_credentials_from_env
//! ```
//!
//! ## Expected Output
//!
//! If credentials are set correctly:
//! ```text
//! Credentials loaded successfully!
//! API Key: PKXX...XXXX (masked)
//! Auth header generated: Basic ...
//! ```

use alpaca_base::{Credentials, Environment};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Alpaca Credentials from Environment ===\n");

    // Method 1: Load credentials from environment variables
    match Credentials::from_env() {
        Ok(credentials) => {
            println!("Credentials loaded successfully!");

            // Mask the API key for display (show first 4 and last 4 chars)
            let masked_key = if credentials.api_key.len() > 8 {
                format!(
                    "{}...{}",
                    &credentials.api_key[..4],
                    &credentials.api_key[credentials.api_key.len() - 4..]
                )
            } else {
                "****".to_string()
            };
            println!("API Key: {} (masked)", masked_key);

            // Generate auth header (useful for HTTP requests)
            let auth_header = credentials.auth_header();
            println!(
                "Auth header generated: {}...",
                &auth_header[..auth_header.len().min(20)]
            );

            // Generate request headers
            let headers = credentials.auth_headers("GET", "/v2/account", "")?;
            println!("\nGenerated {} authentication headers:", headers.len());
            for key in headers.keys() {
                println!("  - {}", key);
            }
        }
        Err(e) => {
            println!("Failed to load credentials: {}", e);
            println!("\nMake sure you have set:");
            println!("  export ALPACA_API_KEY=\"your-api-key\"");
            println!("  export ALPACA_SECRET_KEY=\"your-secret-key\"");
            return Err(e.into());
        }
    }

    // Method 2: Create credentials manually (for demonstration)
    println!("\n--- Manual Credential Creation ---");
    let manual_creds = Credentials::new("demo_key".to_string(), "demo_secret".to_string());
    println!(
        "Manual credentials created with key: {}",
        manual_creds.api_key
    );

    // Demonstrate Environment enum
    println!("\n--- Trading Environments ---");
    println!("Paper trading URL: {}", Environment::Paper.base_url());
    println!("Live trading URL: {}", Environment::Live.base_url());

    println!("\n=== Example Complete ===");
    Ok(())
}
