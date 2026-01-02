//! # FIX Connect
//!
//! This example demonstrates how to connect to a FIX server
//! using the Alpaca FIX protocol client.
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
//! cargo run -p alpaca-fix --example fix_connect
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_base::Credentials;
use alpaca_fix::{FixClient, FixConfig, FixVersion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FIX Connect ===\n");

    // Load credentials from environment
    let credentials = Credentials::from_env()?;
    println!("Credentials loaded from environment");

    // Create FIX configuration
    println!("\n--- FIX Configuration ---");
    let config = FixConfig::builder()
        .version(FixVersion::Fix44)
        .sender_comp_id("YOUR_SENDER_ID")
        .target_comp_id("ALPACA")
        .host("fix.alpaca.markets")
        .port(5001)
        .heartbeat_interval_secs(30)
        .reconnect_enabled(true)
        .reconnect_max_attempts(5)
        .build();

    println!("  FIX Version: {:?}", config.version);
    println!("  Sender CompID: {}", config.sender_comp_id);
    println!("  Target CompID: {}", config.target_comp_id);
    println!("  Host: {}:{}", config.host, config.port);
    println!("  Heartbeat Interval: {}s", config.heartbeat_interval_secs);
    println!("  Reconnect Enabled: {}", config.reconnect_enabled);

    // Create FIX client
    println!("\n--- Creating FIX Client ---");
    let client = FixClient::new(credentials, config);
    println!("FIX client created: {:?}", client);

    // Show session states
    println!("\n--- Session States ---");
    println!("  Disconnected: Not connected to server");
    println!("  Connecting: TCP connection in progress");
    println!("  LoggingOn: Sending logon message");
    println!("  Active: Session established, ready for trading");
    println!("  LoggingOut: Sending logout message");

    // Connection note
    println!("\n--- Connection ---");
    println!("To actually connect, run in async context:");
    println!("  client.connect().await?;");
    println!("  let state = client.state().await;");
    println!();
    println!("FIX protocol requires:");
    println!("  1. Special access enabled by Alpaca");
    println!("  2. Valid Sender CompID assigned by Alpaca");
    println!("  3. Network access to fix.alpaca.markets:5001");

    println!("\n=== Example Complete ===");
    Ok(())
}
