//! # Advanced Error Handling
//!
//! This example demonstrates comprehensive error handling patterns
//! for the Alpaca API clients.
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
//! cargo run --example advanced_error_handling
//! ```

use alpaca_base::{Credentials, Environment};
use alpaca_http::AlpacaHttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced Error Handling ===\n");

    // Pattern 1: Credential validation
    println!("--- Pattern 1: Credential Validation ---");
    match Credentials::from_env() {
        Ok(creds) => {
            println!("  Credentials loaded successfully");
            println!("  API Key length: {} chars", creds.api_key.len());
        }
        Err(e) => {
            println!("  Error loading credentials: {}", e);
            println!("  Ensure ALPACA_API_KEY and ALPACA_API_SECRET are set");
            return Ok(());
        }
    }

    let credentials = Credentials::from_env()?;
    let http_client = AlpacaHttpClient::new(credentials, Environment::Paper);

    // Pattern 2: API request error handling
    println!("\n--- Pattern 2: API Request Errors ---");
    
    // Handle account request errors
    match http_client.get_account().await {
        Ok(account) => {
            println!("  Account retrieved: {}", account.id);
        }
        Err(e) => {
            println!("  Account error: {}", e);
            // Categorize the error
            let error_str = e.to_string();
            if error_str.contains("401") || error_str.contains("403") {
                println!("  -> Authentication error: Check API credentials");
            } else if error_str.contains("429") {
                println!("  -> Rate limit exceeded: Implement backoff");
            } else if error_str.contains("500") || error_str.contains("503") {
                println!("  -> Server error: Retry with exponential backoff");
            } else {
                println!("  -> Unknown error: {}", error_str);
            }
        }
    }

    // Pattern 3: Order validation errors
    println!("\n--- Pattern 3: Order Validation ---");
    println!("  Common order errors:");
    println!("  - insufficient_balance: Not enough buying power");
    println!("  - invalid_qty: Quantity must be positive");
    println!("  - symbol_not_found: Unknown or delisted symbol");
    println!("  - market_closed: Cannot trade outside market hours");
    println!("  - account_restricted: Account has trading restrictions");

    // Pattern 4: Position errors
    println!("\n--- Pattern 4: Position Errors ---");
    match http_client.get_position("INVALID_SYMBOL_XYZ").await {
        Ok(pos) => {
            println!("  Position found: {} shares", pos.qty);
        }
        Err(e) => {
            println!("  Expected error for invalid symbol: {}", e);
            println!("  -> Handle gracefully: No position exists");
        }
    }

    // Pattern 5: Result combinators
    println!("\n--- Pattern 5: Result Combinators ---");
    
    // Using map_err to add context
    let account_result = http_client
        .get_account()
        .await
        .map_err(|e| format!("Failed to fetch account: {}", e));
    
    match account_result {
        Ok(acc) => println!("  Account status: {:?}", acc.status),
        Err(e) => println!("  Contextualized error: {}", e),
    }

    // Using ok() to convert to Option
    let maybe_clock = http_client.get_clock().await.ok();
    if let Some(clock) = maybe_clock {
        println!("  Market is open: {}", clock.is_open);
    } else {
        println!("  Could not determine market status");
    }

    // Pattern 6: Error recovery strategies
    println!("\n--- Pattern 6: Error Recovery ---");
    println!("  Strategy 1: Retry with backoff");
    println!("    for attempt in 1..=3 {{");
    println!("        match client.get_account().await {{");
    println!("            Ok(acc) => return Ok(acc),");
    println!("            Err(_) if attempt < 3 => {{");
    println!("                tokio::time::sleep(Duration::from_secs(attempt)).await;");
    println!("            }}");
    println!("            Err(e) => return Err(e),");
    println!("        }}");
    println!("    }}");
    println!();
    println!("  Strategy 2: Fallback values");
    println!("    let positions = client.get_positions().await.unwrap_or_default();");
    println!();
    println!("  Strategy 3: Circuit breaker");
    println!("    Track consecutive failures and stop retrying after threshold");

    // Pattern 7: Logging errors
    println!("\n--- Pattern 7: Error Logging ---");
    println!("  Use tracing for structured error logging:");
    println!("    tracing::error!(error = %e, \"API request failed\");");
    println!("    tracing::warn!(symbol = %sym, \"Position not found\");");

    // Pattern 8: Custom error types
    println!("\n--- Pattern 8: Custom Error Types ---");
    println!("  #[derive(Debug, thiserror::Error)]");
    println!("  enum TradingError {{");
    println!("      #[error(\"API error: {{0}}\")]");
    println!("      Api(#[from] alpaca_http::Error),");
    println!("      #[error(\"Insufficient funds: need ${{needed}}, have ${{available}}\")]");
    println!("      InsufficientFunds {{ needed: f64, available: f64 }},");
    println!("      #[error(\"Market closed\")]");
    println!("      MarketClosed,");
    println!("  }}");

    // Summary
    println!("\n--- Error Handling Best Practices ---");
    println!("1. Always validate credentials at startup");
    println!("2. Categorize errors (auth, rate limit, server, validation)");
    println!("3. Use Result combinators for clean error handling");
    println!("4. Implement retry with exponential backoff for transient errors");
    println!("5. Log errors with context for debugging");
    println!("6. Define custom error types for domain-specific errors");
    println!("7. Fail fast for unrecoverable errors");

    println!("\n=== Example Complete ===");
    Ok(())
}
