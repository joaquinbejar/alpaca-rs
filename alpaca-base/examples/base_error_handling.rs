//! # Error Handling Patterns
//!
//! This example demonstrates how to work with Alpaca error types,
//! including error codes, retryable errors, and validation errors.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_error_handling
//! ```
//!
//! ## Expected Output
//!
//! Demonstrates various error types and their properties.

use alpaca_base::{AlpacaError, ApiErrorCode, ApiErrorResponse, RateLimitInfo, ValidationError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Alpaca Error Handling Patterns ===\n");

    // 1. API Error Codes
    println!("--- API Error Codes ---");
    demonstrate_error_codes();

    // 2. Retryable Errors
    println!("\n--- Retryable Errors ---");
    demonstrate_retryable_errors();

    // 3. Rate Limiting
    println!("\n--- Rate Limit Handling ---");
    demonstrate_rate_limiting();

    // 4. Validation Errors
    println!("\n--- Validation Errors ---");
    demonstrate_validation_errors();

    // 5. Error Matching
    println!("\n--- Error Pattern Matching ---");
    demonstrate_error_matching();

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_error_codes() {
    // Convert numeric codes to typed error codes
    let codes = [
        (40010000, "Malformed Request"),
        (40110000, "Invalid Credentials"),
        (40310000, "Forbidden"),
        (40410000, "Not Found"),
        (42210000, "Unprocessable Entity"),
        (42910000, "Rate Limit Exceeded"),
        (50010000, "Internal Server Error"),
    ];

    for (code, description) in codes {
        let error_code = ApiErrorCode::from_code(code);
        println!(
            "  Code {}: {} (client_error={}, server_error={})",
            code,
            description,
            error_code.is_client_error(),
            error_code.is_server_error()
        );
    }
}

fn demonstrate_retryable_errors() {
    let errors = [
        ("Rate Limit", AlpacaError::rate_limit(60)),
        (
            "Network",
            AlpacaError::Network("connection reset".to_string()),
        ),
        (
            "Timeout",
            AlpacaError::Timeout("request timed out".to_string()),
        ),
        ("Auth", AlpacaError::Auth("invalid key".to_string())),
        ("Not Found", AlpacaError::api(404, "order not found")),
        ("Server Error", AlpacaError::api(500, "internal error")),
    ];

    for (name, error) in errors {
        let retry_info = if let Some(secs) = error.retry_after() {
            format!(", retry after {} secs", secs)
        } else {
            String::new()
        };
        println!(
            "  {}: retryable={}{}",
            name,
            error.is_retryable(),
            retry_info
        );
    }
}

fn demonstrate_rate_limiting() {
    // Create rate limit info from API response headers
    let info = RateLimitInfo::new()
        .with_remaining(0)
        .with_limit(200)
        .with_retry_after(60);

    println!("  Rate limit status:");
    println!("    - Remaining: {:?}", info.remaining);
    println!("    - Limit: {:?}", info.limit);
    println!("    - Retry after: {:?} seconds", info.retry_after);
    println!("    - Is limited: {}", info.is_limited());

    // Create error with full rate limit info
    let error = AlpacaError::rate_limit_with_info(info);
    println!("  Error: {}", error);
}

fn demonstrate_validation_errors() {
    // Single validation error
    let single_error = ValidationError::new("quantity", "must be greater than 0");
    println!("  Single error: {}", single_error);

    // Multiple validation errors
    let errors = vec![
        ValidationError::new("symbol", "is required"),
        ValidationError::new("qty", "must be positive"),
        ValidationError::new("side", "must be 'buy' or 'sell'"),
    ];

    let multi_error = AlpacaError::ValidationErrors(errors);
    println!("  Multiple errors: {}", multi_error);
}

fn demonstrate_error_matching() {
    let error = AlpacaError::api_with_details(
        404,
        "order not found",
        ApiErrorCode::NotFound,
        Some("req-12345".to_string()),
    );

    // Pattern match on error type
    match &error {
        AlpacaError::Api {
            status,
            message,
            error_code,
            request_id,
        } => {
            println!("  API Error detected:");
            println!("    - Status: {}", status);
            println!("    - Message: {}", message);
            println!("    - Error code: {:?}", error_code);
            println!("    - Request ID: {:?}", request_id);
        }
        AlpacaError::RateLimit {
            retry_after_secs, ..
        } => {
            println!("  Rate limited, retry after {} seconds", retry_after_secs);
        }
        _ => {
            println!("  Other error: {}", error);
        }
    }

    // Use helper methods
    println!("\n  Helper methods:");
    println!("    - status_code(): {:?}", error.status_code());
    println!("    - request_id(): {:?}", error.request_id());
    println!("    - is_retryable(): {}", error.is_retryable());

    // API error response parsing
    let response =
        ApiErrorResponse::new(40410000, "resource not found").with_request_id("req-67890");
    println!("\n  Parsed API response:");
    println!("    - Code: {}", response.code);
    println!("    - Message: {}", response.message);
    println!("    - Typed code: {:?}", response.error_code());
}
