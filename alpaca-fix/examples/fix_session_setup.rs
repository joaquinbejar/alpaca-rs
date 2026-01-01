//! # FIX Session Setup
//!
//! This example demonstrates how to configure and set up a FIX session.
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
//! cargo run -p alpaca-fix --example fix_session_setup
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca. Contact Alpaca
//! support to enable FIX access for your account.

use alpaca_base::Credentials;
use alpaca_fix::{FixClient, FixConfig, FixVersion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FIX Session Setup ===\n");

    // Load credentials from environment
    let credentials = Credentials::from_env()?;
    println!("Credentials loaded from environment");

    // Create FIX configuration using builder pattern
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
        .reconnect_delay_ms(1000)
        .message_logging(true)
        .reset_on_logon(false)
        .build();

    println!("  Version: {}", config.version);
    println!("  Sender CompID: {}", config.sender_comp_id);
    println!("  Target CompID: {}", config.target_comp_id);
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  Heartbeat Interval: {}s", config.heartbeat_interval_secs);
    println!("  Reconnect Enabled: {}", config.reconnect_enabled);
    println!(
        "  Max Reconnect Attempts: {}",
        config.reconnect_max_attempts
    );
    println!("  Reconnect Delay: {}ms", config.reconnect_delay_ms);
    println!("  Message Logging: {}", config.message_logging);
    println!("  Reset on Logon: {}", config.reset_on_logon);

    // Create FIX client
    println!("\n--- Creating FIX Client ---");
    let client = FixClient::new(credentials, config);
    println!("  FIX client created: {:?}", client);

    // Demonstrate FIX versions
    println!("\n--- Supported FIX Versions ---");
    println!("  FIX 4.2: {}", FixVersion::Fix42.begin_string());
    println!("  FIX 4.4: {}", FixVersion::Fix44.begin_string());

    // Note about connection
    println!("\n--- Connection Notes ---");
    println!("  To connect to the FIX server, use:");
    println!("    client.connect().await?;");
    println!();
    println!("  FIX protocol requires:");
    println!("    1. Special access enabled by Alpaca");
    println!("    2. Valid Sender CompID assigned by Alpaca");
    println!("    3. Network access to fix.alpaca.markets:5001");

    println!("\n=== Example Complete ===");
    Ok(())
}
