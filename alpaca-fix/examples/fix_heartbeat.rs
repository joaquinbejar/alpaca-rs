//! # FIX Heartbeat
//!
//! This example demonstrates heartbeat configuration in FIX protocol.
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
//! cargo run -p alpaca-fix --example fix_heartbeat
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_fix::messages::MsgType;
use alpaca_fix::{FixConfig, FixVersion};

fn main() {
    println!("=== FIX Heartbeat ===\n");

    // Create config with heartbeat settings
    println!("--- Heartbeat Configuration ---");
    let config = FixConfig::builder()
        .version(FixVersion::Fix44)
        .sender_comp_id("YOUR_SENDER_ID")
        .target_comp_id("ALPACA")
        .host("fix.alpaca.markets")
        .port(5001)
        .heartbeat_interval_secs(30)
        .build();

    println!("  Heartbeat Interval: {}s", config.heartbeat_interval_secs);
    println!("  Typical intervals: 30-60 seconds");

    // Message types related to heartbeat
    println!("\n--- Heartbeat Message Types ---");
    let heartbeat = MsgType::Heartbeat;
    let test_request = MsgType::TestRequest;

    println!("  Heartbeat: MsgType = '{}'", heartbeat.as_str());
    println!("  TestRequest: MsgType = '{}'", test_request.as_str());

    // FIX tags
    println!("\n--- Heartbeat FIX Tags ---");
    println!("  MsgType (35): 0 (Heartbeat)");
    println!("  MsgType (35): 1 (TestRequest)");
    println!("  TestReqID (112): Echoed in Heartbeat response");
    println!("  HeartBtInt (108): Interval in seconds (in Logon)");

    // Heartbeat flow
    println!("\n--- Heartbeat Flow ---");
    println!("  Normal operation:");
    println!(
        "    Client -> Heartbeat -> Server (every {}s)",
        config.heartbeat_interval_secs
    );
    println!("    Server -> Heartbeat -> Client");
    println!();
    println!("  Connection check:");
    println!("    Client -> TestRequest (ID=123) -> Server");
    println!("    Server -> Heartbeat (TestReqID=123) -> Client");

    // Automatic handling
    println!("\n--- Automatic Handling ---");
    println!("  The FIX client handles heartbeats automatically:");
    println!("  1. Sends heartbeats at configured interval");
    println!("  2. Responds to TestRequest with Heartbeat");
    println!("  3. Detects connection loss if no response");

    println!("\n=== Example Complete ===");
}
