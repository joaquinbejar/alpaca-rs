//! # FIX Disconnect
//!
//! This example demonstrates how to gracefully disconnect from a FIX server
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
//! cargo run -p alpaca-fix --example fix_disconnect
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_base::Credentials;
use alpaca_fix::session::SessionState;
use alpaca_fix::{FixClient, FixConfig, FixVersion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FIX Disconnect ===\n");

    // Load credentials
    let credentials = Credentials::from_env()?;
    println!("Credentials loaded from environment");

    // Create FIX configuration
    let config = FixConfig::builder()
        .version(FixVersion::Fix44)
        .sender_comp_id("YOUR_SENDER_ID")
        .target_comp_id("ALPACA")
        .host("fix.alpaca.markets")
        .port(5001)
        .heartbeat_interval_secs(30)
        .build();

    // Create FIX client
    let client = FixClient::new(credentials, config);
    println!("FIX client created: {:?}", client);

    // Show session states
    println!("\n--- Session States ---");
    let states = [
        SessionState::Disconnected,
        SessionState::Connecting,
        SessionState::LoggingOn,
        SessionState::Active,
        SessionState::LoggingOut,
    ];
    for state in states {
        println!("  {:?}", state);
    }

    // Disconnect process
    println!("\n--- Disconnect Process ---");
    println!("In async context:");
    println!("  // Connect first");
    println!("  client.connect().await?;");
    println!("  assert_eq!(client.state().await, SessionState::Active);");
    println!();
    println!("  // Graceful disconnect");
    println!("  client.disconnect().await?;");
    println!("  assert_eq!(client.state().await, SessionState::Disconnected);");

    // Logout message
    println!("\n--- Logout Message (MsgType 5) ---");
    println!("  MsgType (35): 5");
    println!("  SenderCompID (49): Your sender ID");
    println!("  TargetCompID (56): ALPACA");
    println!("  MsgSeqNum (34): Next sequence number");
    println!("  SendingTime (52): UTC timestamp");
    println!("  Text (58): Optional logout reason");

    // State transitions
    println!("\n--- State Transitions ---");
    println!("  Active -> LoggingOut -> Disconnected");

    println!("\n=== Example Complete ===");
    Ok(())
}
