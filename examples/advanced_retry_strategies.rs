//! # Advanced Retry Strategies
//!
//! This example demonstrates retry patterns and strategies for handling
//! transient failures in API requests.
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
//! cargo run --example advanced_retry_strategies
//! ```

use alpaca_base::{Credentials, Environment};
use alpaca_http::AlpacaHttpClient;
use std::time::Duration;

/// Simple retry configuration
struct RetryConfig {
    max_attempts: u32,
    initial_delay_ms: u64,
    max_delay_ms: u64,
    backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let delay = self.initial_delay_ms as f64 
            * self.backoff_multiplier.powi(attempt as i32 - 1);
        let delay_ms = delay.min(self.max_delay_ms as f64) as u64;
        Duration::from_millis(delay_ms)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced Retry Strategies ===\n");

    let credentials = Credentials::from_env()?;
    let http_client = AlpacaHttpClient::new(credentials, Environment::Paper);

    // Strategy 1: Simple retry with fixed delay
    println!("--- Strategy 1: Fixed Delay Retry ---");
    println!("  Retry with constant delay between attempts");
    println!("  Best for: Simple cases, predictable failures");
    println!();
    println!("  async fn retry_fixed<T, F, Fut>(f: F, max_attempts: u32) -> Result<T, E>");
    println!("  where F: Fn() -> Fut {{");
    println!("      for attempt in 1..=max_attempts {{");
    println!("          match f().await {{");
    println!("              Ok(v) => return Ok(v),");
    println!("              Err(e) if attempt < max_attempts => {{");
    println!("                  tokio::time::sleep(Duration::from_secs(1)).await;");
    println!("              }}");
    println!("              Err(e) => return Err(e),");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Strategy 2: Exponential backoff
    println!("\n--- Strategy 2: Exponential Backoff ---");
    let config = RetryConfig::default();
    println!("  Configuration:");
    println!("    Max attempts: {}", config.max_attempts);
    println!("    Initial delay: {}ms", config.initial_delay_ms);
    println!("    Max delay: {}ms", config.max_delay_ms);
    println!("    Backoff multiplier: {}", config.backoff_multiplier);
    println!();
    println!("  Delay progression:");
    for attempt in 1..=5 {
        let delay = config.delay_for_attempt(attempt);
        println!("    Attempt {}: {:?}", attempt, delay);
    }

    // Strategy 3: Exponential backoff with jitter
    println!("\n--- Strategy 3: Exponential Backoff with Jitter ---");
    println!("  Add randomness to prevent thundering herd");
    println!();
    println!("  fn delay_with_jitter(base_delay: Duration) -> Duration {{");
    println!("      let jitter = rand::random::<f64>() * 0.3; // 0-30% jitter");
    println!("      let jittered = base_delay.as_millis() as f64 * (1.0 + jitter);");
    println!("      Duration::from_millis(jittered as u64)");
    println!("  }}");

    // Strategy 4: Retry with condition
    println!("\n--- Strategy 4: Conditional Retry ---");
    println!("  Only retry on specific error types");
    println!();
    println!("  fn should_retry(error: &Error) -> bool {{");
    println!("      match error {{");
    println!("          Error::RateLimit => true,   // 429");
    println!("          Error::ServerError => true, // 5xx");
    println!("          Error::Timeout => true,");
    println!("          Error::AuthError => false,  // Don't retry auth failures");
    println!("          Error::ValidationError => false,");
    println!("          _ => false,");
    println!("      }}");
    println!("  }}");

    // Strategy 5: Circuit breaker pattern
    println!("\n--- Strategy 5: Circuit Breaker ---");
    println!("  Stop retrying after too many failures");
    println!();
    println!("  struct CircuitBreaker {{");
    println!("      failure_count: AtomicU32,");
    println!("      failure_threshold: u32,");
    println!("      reset_timeout: Duration,");
    println!("      last_failure: Mutex<Option<Instant>>,");
    println!("  }}");
    println!();
    println!("  States: Closed -> Open -> Half-Open -> Closed");
    println!("    Closed: Normal operation, track failures");
    println!("    Open: Fail fast, don't attempt requests");
    println!("    Half-Open: Allow one test request");

    // Demonstrate actual retry
    println!("\n--- Live Retry Demo ---");
    let result = retry_with_backoff(|| async {
        http_client.get_clock().await
    }, &RetryConfig::default()).await;

    match result {
        Ok(clock) => {
            println!("  Success! Market is open: {}", clock.is_open);
        }
        Err(e) => {
            println!("  Failed after retries: {}", e);
        }
    }

    // Strategy 6: Retry budget
    println!("\n--- Strategy 6: Retry Budget ---");
    println!("  Limit total retries across all requests");
    println!();
    println!("  struct RetryBudget {{");
    println!("      tokens: AtomicU32,");
    println!("      refill_rate: u32, // tokens per second");
    println!("  }}");
    println!();
    println!("  impl RetryBudget {{");
    println!("      fn try_acquire(&self) -> bool {{");
    println!("          self.tokens.fetch_sub(1, Ordering::SeqCst) > 0");
    println!("      }}");
    println!("  }}");

    // Best practices summary
    println!("\n--- Retry Best Practices ---");
    println!("1. Use exponential backoff for rate limits");
    println!("2. Add jitter to prevent thundering herd");
    println!("3. Set maximum retry attempts (3-5 typical)");
    println!("4. Only retry transient/recoverable errors");
    println!("5. Implement circuit breaker for cascading failures");
    println!("6. Log retry attempts for debugging");
    println!("7. Consider retry budgets for high-volume systems");
    println!("8. Make retries idempotent when possible");

    println!("\n=== Example Complete ===");
    Ok(())
}

/// Retry a future with exponential backoff
async fn retry_with_backoff<T, E, F, Fut>(
    f: F,
    config: &RetryConfig,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error = None;
    
    for attempt in 1..=config.max_attempts {
        match f().await {
            Ok(value) => {
                if attempt > 1 {
                    println!("  Succeeded on attempt {}", attempt);
                }
                return Ok(value);
            }
            Err(e) => {
                println!("  Attempt {} failed: {}", attempt, e);
                last_error = Some(e);
                
                if attempt < config.max_attempts {
                    let delay = config.delay_for_attempt(attempt);
                    println!("  Retrying in {:?}...", delay);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}
