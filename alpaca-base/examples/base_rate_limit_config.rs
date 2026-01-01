//! # Rate Limit Configuration
//!
//! This example demonstrates rate limiting configuration and status tracking.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_rate_limit_config
//! ```

use alpaca_base::{RateLimitConfig, RateLimitStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rate Limit Configuration ===\n");

    // 1. Default Configuration
    println!("--- Default Configuration ---");
    demonstrate_default_config();

    // 2. Custom Configuration
    println!("\n--- Custom Configuration ---");
    demonstrate_custom_config();

    // 3. Rate Limit Status
    println!("\n--- Rate Limit Status ---");
    demonstrate_status();

    // 4. Rate Limiting Strategy
    println!("\n--- Rate Limiting Strategy ---");
    demonstrate_strategy();

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_default_config() {
    let config = RateLimitConfig::default();

    println!("  Default Rate Limit Config:");
    println!("    Requests per minute: {}", config.requests_per_minute);
    println!("    Burst limit: {}", config.burst_limit);
    println!("    Max retries: {}", config.max_retries);
    println!("    Base delay: {} ms", config.base_delay_ms);
}

fn demonstrate_custom_config() {
    let config = RateLimitConfig::new()
        .requests_per_minute(100)
        .burst_limit(25)
        .max_retries(5)
        .base_delay_ms(500);

    println!("  Custom Rate Limit Config:");
    println!("    Requests per minute: {}", config.requests_per_minute);
    println!("    Burst limit: {}", config.burst_limit);
    println!("    Max retries: {}", config.max_retries);
    println!("    Base delay: {} ms", config.base_delay_ms);

    // Calculate effective rate
    let requests_per_second = config.requests_per_minute as f64 / 60.0;
    println!(
        "\n  Effective rate: {:.2} requests/second",
        requests_per_second
    );
}

fn demonstrate_status() {
    // Normal status - plenty of requests remaining
    let normal = RateLimitStatus::new(150, 200, 1704067200);
    println!("  Normal Status:");
    println!("    Remaining: {}/{}", normal.remaining, normal.limit);
    println!("    Rate limited: {}", normal.is_rate_limited());

    // Low status - running low
    let low = RateLimitStatus::new(10, 200, 1704067200);
    println!("\n  Low Status:");
    println!("    Remaining: {}/{}", low.remaining, low.limit);
    println!("    Rate limited: {}", low.is_rate_limited());
    println!(
        "    Usage: {:.1}%",
        (1.0 - low.remaining as f64 / low.limit as f64) * 100.0
    );

    // Rate limited - no requests remaining
    let limited = RateLimitStatus::new(0, 200, 1704067260);
    println!("\n  Rate Limited Status:");
    println!("    Remaining: {}/{}", limited.remaining, limited.limit);
    println!("    Rate limited: {}", limited.is_rate_limited());
    println!("    Reset at: {} (Unix timestamp)", limited.reset_at);
}

fn demonstrate_strategy() {
    println!("  Rate Limiting Best Practices:");
    println!();
    println!("    1. Monitor remaining requests from response headers");
    println!("       X-RateLimit-Remaining: 150");
    println!("       X-RateLimit-Limit: 200");
    println!("       X-RateLimit-Reset: 1704067200");
    println!();
    println!("    2. Implement exponential backoff on 429 errors");
    println!("       Retry 1: wait 1s");
    println!("       Retry 2: wait 2s");
    println!("       Retry 3: wait 4s");
    println!("       ...");
    println!();
    println!("    3. Use burst limit for initial requests");
    println!("       Then throttle to stay under rate limit");
    println!();
    println!("    4. Queue requests when approaching limit");
    println!("       Process queue when limit resets");
}
