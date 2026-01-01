//! # Error Handling Example
//!
//! This example demonstrates the different error types in the `alpaca-rs` library
//! and how to handle them effectively, including retry logic.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_error_handling
//! ```

use alpaca_base::{AlpacaError, ApiErrorCode};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Alpaca Error Handling ===\n");

    // 1. Simulating an API Error (e.g., from Alpaca server)
    println!("--- API Error Case ---");
    let api_error = AlpacaError::Api("insufficient buying power".to_string());
    handle_error(&api_error);

    // 2. Simulating a Rate Limit Error
    println!("\n--- Rate Limit Case ---");
    let rate_limit_error = AlpacaError::RateLimit("too many requests".to_string());
    handle_error(&rate_limit_error);
    
    // Check if retryable
    if rate_limit_error.is_retryable() {
        println!("  💡 Pro-tip: This error is retryable. You should implement an exponential backoff.");
    }

    // 3. Simulating a Validation Error
    println!("\n--- Validation Error Case ---");
    let validation_error = AlpacaError::Validation("qty must be greater than 0".to_string());
    handle_error(&validation_error);

    // 4. Working with ApiErrorCode
    println!("\n--- Specific API Error Codes ---");
    let forbidden_code = ApiErrorCode::Forbidden;
    println!("  Error Code: {:?} (Integer: {})", forbidden_code, forbidden_code as u16);
    
    let order_not_found = ApiErrorCode::OrderNotFound;
    println!("  Error Code: {:?} (Integer: {})", order_not_found, order_not_found as u16);

    println!("\n=== Example Complete ===");
    Ok(())
}

fn handle_error(err: &AlpacaError) {
    match err {
        AlpacaError::Api(msg) => println!("  Caught API Error: {}", msg),
        AlpacaError::RateLimit(msg) => println!("  Caught Rate Limit: {}. Wait before retrying.", msg),
        AlpacaError::Validation(msg) => println!("  Caught Validation Error: {}", msg),
        AlpacaError::Auth(msg) => println!("  Caught Authentication Error: {}", msg),
        _ => println!("  Caught Other Error: {:?}", err),
    }
}
