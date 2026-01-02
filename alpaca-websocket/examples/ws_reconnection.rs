//! # WebSocket Reconnection Handling
//!
//! This example demonstrates how to handle WebSocket disconnections
//! and implement reconnection strategies.
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
//! cargo run -p alpaca-websocket --example ws_reconnection
//! ```

use alpaca_base::Environment;
use alpaca_websocket::{AlpacaWebSocketClient, ConnectionState, WebSocketConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== WebSocket Reconnection Handling ===\n");

    // Create client
    let environment = Environment::Paper;
    println!("WebSocket client created for {:?} environment", environment);
    let client = AlpacaWebSocketClient::from_env(environment)?;

    // Reconnection configuration
    println!("\n--- Reconnection Configuration ---");
    let config = WebSocketConfig::new()
        .max_reconnect_attempts(5)
        .reconnect_base_delay(1000);

    println!("  Max attempts: {}", config.reconnect_max_attempts);
    println!("  Base delay: {}ms", config.reconnect_base_delay_ms);
    println!("  Max delay: {}ms", config.reconnect_max_delay_ms);

    // Exponential backoff calculation
    println!("\n--- Exponential Backoff Strategy ---");
    println!("  Delay = base_delay * 2^(attempt - 1)");
    println!();
    let base_delay = config.reconnect_base_delay_ms;
    let max_delay = config.reconnect_max_delay_ms;
    for attempt in 1..=5 {
        let delay = (base_delay * 2u64.pow(attempt - 1)).min(max_delay);
        println!("  Attempt {}: {}ms delay", attempt, delay);
    }

    // Connection state machine
    println!("\n--- Connection State Machine ---");
    println!("  Disconnected -> Connecting -> Connected");
    println!("       ^              |             |");
    println!("       |              v             v");
    println!("       +-------- Failed      Reconnecting");
    println!("                                    |");
    println!("                                    v");
    println!("                              Connecting");

    // State transitions
    println!("\n--- State Transitions ---");
    let transitions = [
        (
            ConnectionState::Disconnected,
            ConnectionState::Connecting,
            "connect() called",
        ),
        (
            ConnectionState::Connecting,
            ConnectionState::Connected,
            "authentication successful",
        ),
        (
            ConnectionState::Connecting,
            ConnectionState::Failed,
            "connection error",
        ),
        (
            ConnectionState::Connected,
            ConnectionState::Reconnecting,
            "connection lost",
        ),
        (
            ConnectionState::Reconnecting,
            ConnectionState::Connecting,
            "retry attempt",
        ),
        (
            ConnectionState::Reconnecting,
            ConnectionState::Failed,
            "max attempts exceeded",
        ),
    ];

    for (from, to, trigger) in transitions {
        println!("  {:?} -> {:?}: {}", from, to, trigger);
    }

    // Reconnection with the client
    println!("\n--- Using connect_with_reconnect ---");
    println!("  let stream = client.connect_with_reconnect(5).await?;");
    println!();
    println!("  This method will:");
    println!("  1. Attempt initial connection");
    println!("  2. On failure, wait with exponential backoff");
    println!("  3. Retry up to max_retries times");
    println!("  4. Return error if all attempts fail");

    // Demo connection with reconnect
    println!("\n--- Live Connection Demo ---");
    println!("Attempting connection with reconnect (max 3 attempts)...");

    match client.connect_with_reconnect(3).await {
        Ok(_stream) => {
            println!("  Connected successfully!");
            println!("  Stream is ready to receive messages");
        }
        Err(e) => {
            println!("  Connection failed: {}", e);
            println!("  (This is expected if credentials are not set)");
        }
    }

    // Error handling patterns
    println!("\n--- Error Handling Patterns ---");
    println!("  Pattern 1: Simple retry");
    println!("    loop {{");
    println!("        match client.connect().await {{");
    println!("            Ok(stream) => break stream,");
    println!("            Err(e) => {{");
    println!("                eprintln!(\"Connection failed: {{}}\", e);");
    println!("                tokio::time::sleep(Duration::from_secs(5)).await;");
    println!("            }}");
    println!("        }}");
    println!("    }}");
    println!();
    println!("  Pattern 2: Built-in reconnect");
    println!("    let stream = client.connect_with_reconnect(10).await?;");
    println!();
    println!("  Pattern 3: Circuit breaker");
    println!("    if consecutive_failures > threshold {{");
    println!("        return Err(\"Circuit breaker open\");");
    println!("    }}");

    // Best practices
    println!("\n--- Reconnection Best Practices ---");
    println!("1. Always use exponential backoff");
    println!("2. Set reasonable max attempts (5-10)");
    println!("3. Log reconnection attempts for debugging");
    println!("4. Handle partial state on reconnect");
    println!("5. Re-subscribe to streams after reconnect");
    println!("6. Consider circuit breaker for persistent failures");

    println!("\n=== Example Complete ===");
    Ok(())
}
