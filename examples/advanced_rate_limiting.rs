//! # Advanced Rate Limiting
//!
//! This example demonstrates rate limiting patterns for Alpaca API requests.
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
//! cargo run --example advanced_rate_limiting
//! ```

use alpaca_base::{Credentials, Environment};
use alpaca_http::AlpacaHttpClient;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

/// Simple token bucket rate limiter
struct TokenBucket {
    tokens: AtomicU64,
    max_tokens: u64,
    refill_rate: u64, // tokens per second
    last_refill: std::sync::Mutex<Instant>,
}

impl TokenBucket {
    fn new(max_tokens: u64, refill_rate: u64) -> Self {
        Self {
            tokens: AtomicU64::new(max_tokens),
            max_tokens,
            refill_rate,
            last_refill: std::sync::Mutex::new(Instant::now()),
        }
    }

    fn try_acquire(&self) -> bool {
        self.refill();
        let current = self.tokens.load(Ordering::SeqCst);
        if current > 0 {
            self.tokens.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    fn refill(&self) {
        let mut last = self.last_refill.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs();
        
        if elapsed > 0 {
            let new_tokens = elapsed * self.refill_rate;
            let current = self.tokens.load(Ordering::SeqCst);
            let updated = (current + new_tokens).min(self.max_tokens);
            self.tokens.store(updated, Ordering::SeqCst);
            *last = now;
        }
    }

    async fn acquire(&self) {
        while !self.try_acquire() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced Rate Limiting ===\n");

    let credentials = Credentials::from_env()?;
    let http_client = Arc::new(AlpacaHttpClient::new(credentials, Environment::Paper));

    // Alpaca rate limits
    println!("--- Alpaca API Rate Limits ---");
    println!("  Trading API: 200 requests/minute");
    println!("  Data API: Varies by subscription tier");
    println!("  WebSocket: No explicit limit (connection-based)");

    // Pattern 1: Token bucket
    println!("\n--- Pattern 1: Token Bucket ---");
    let bucket = TokenBucket::new(200, 3); // 200 max, 3 per second refill
    println!("  Max tokens: 200");
    println!("  Refill rate: 3/second");
    println!("  Current tokens: {}", bucket.tokens.load(Ordering::SeqCst));
    
    // Simulate some requests
    for i in 1..=5 {
        if bucket.try_acquire() {
            println!("  Request {} acquired token", i);
        } else {
            println!("  Request {} rate limited", i);
        }
    }
    println!("  Remaining tokens: {}", bucket.tokens.load(Ordering::SeqCst));

    // Pattern 2: Semaphore for concurrent requests
    println!("\n--- Pattern 2: Concurrent Request Limiting ---");
    let semaphore = Arc::new(Semaphore::new(5)); // Max 5 concurrent requests
    println!("  Max concurrent requests: 5");
    println!("  Available permits: {}", semaphore.available_permits());
    
    println!("  Usage:");
    println!("    let permit = semaphore.acquire().await?;");
    println!("    let result = client.get_account().await;");
    println!("    drop(permit); // Release permit");

    // Pattern 3: Sliding window
    println!("\n--- Pattern 3: Sliding Window ---");
    println!("  Track requests in a time window");
    println!();
    println!("  struct SlidingWindow {{");
    println!("      requests: VecDeque<Instant>,");
    println!("      window_size: Duration,");
    println!("      max_requests: usize,");
    println!("  }}");
    println!();
    println!("  impl SlidingWindow {{");
    println!("      fn can_proceed(&mut self) -> bool {{");
    println!("          let now = Instant::now();");
    println!("          // Remove old requests outside window");
    println!("          while let Some(&t) = self.requests.front() {{");
    println!("              if now.duration_since(t) > self.window_size {{");
    println!("                  self.requests.pop_front();");
    println!("              }} else {{ break; }}");
    println!("          }}");
    println!("          self.requests.len() < self.max_requests");
    println!("      }}");
    println!("  }}");

    // Pattern 4: Adaptive rate limiting
    println!("\n--- Pattern 4: Adaptive Rate Limiting ---");
    println!("  Adjust rate based on API response headers");
    println!();
    println!("  Headers to watch:");
    println!("    X-RateLimit-Limit: Maximum requests allowed");
    println!("    X-RateLimit-Remaining: Requests remaining");
    println!("    X-RateLimit-Reset: When limit resets (Unix timestamp)");
    println!();
    println!("  if remaining < limit * 0.1 {{");
    println!("      // Less than 10% remaining, slow down");
    println!("      tokio::time::sleep(Duration::from_millis(500)).await;");
    println!("  }}");

    // Pattern 5: Request queuing
    println!("\n--- Pattern 5: Request Queuing ---");
    println!("  Queue requests and process at controlled rate");
    println!();
    println!("  struct RequestQueue {{");
    println!("      queue: mpsc::Sender<Request>,");
    println!("      rate: Duration, // Time between requests");
    println!("  }}");
    println!();
    println!("  // Worker processes queue at fixed rate");
    println!("  async fn worker(rx: mpsc::Receiver<Request>, rate: Duration) {{");
    println!("      let mut interval = tokio::time::interval(rate);");
    println!("      while let Some(req) = rx.recv().await {{");
    println!("          interval.tick().await;");
    println!("          process(req).await;");
    println!("      }}");
    println!("  }}");

    // Live demo with rate limiting
    println!("\n--- Live Rate-Limited Requests ---");
    let rate_limiter = Arc::new(TokenBucket::new(10, 2));
    
    let start = Instant::now();
    for i in 1..=3 {
        rate_limiter.acquire().await;
        let client = Arc::clone(&http_client);
        match client.get_clock().await {
            Ok(clock) => {
                println!(
                    "  Request {} at {:?}: market open = {}",
                    i,
                    start.elapsed(),
                    clock.is_open
                );
            }
            Err(e) => {
                println!("  Request {} failed: {}", i, e);
            }
        }
    }

    // Best practices
    println!("\n--- Rate Limiting Best Practices ---");
    println!("1. Implement client-side rate limiting proactively");
    println!("2. Use token bucket for bursty traffic patterns");
    println!("3. Use sliding window for strict rate enforcement");
    println!("4. Monitor X-RateLimit headers and adapt");
    println!("5. Queue non-urgent requests during high load");
    println!("6. Implement backoff when rate limited (429)");
    println!("7. Use separate limits for different API endpoints");
    println!("8. Log rate limit events for capacity planning");

    // Alpaca-specific recommendations
    println!("\n--- Alpaca-Specific Recommendations ---");
    println!("1. Stay well under 200 req/min for trading API");
    println!("2. Batch market data requests when possible");
    println!("3. Use WebSocket for real-time data (no rate limit)");
    println!("4. Cache static data (assets, calendar)");
    println!("5. Implement exponential backoff on 429 responses");

    println!("\n=== Example Complete ===");
    Ok(())
}
