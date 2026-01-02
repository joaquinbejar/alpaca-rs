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
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

use alpaca_fix::{FixConfig, FixVersion};

fn main() {
    println!("=== FIX Connect ===\n");

    // Create FIX configuration
    println!("--- FIX Configuration ---");
    let config = FixConfig::builder()
        .version(FixVersion::Fix44)
        .sender_comp_id("YOUR_SENDER_ID")
        .target_comp_id("ALPACA")
        .host("fix.alpaca.markets")
        .port(5001)
        .heartbeat_interval_secs(30)
        .build();

    println!("  FIX Version: {:?}", config.version);
    println!("  Sender CompID: {}", config.sender_comp_id);
    println!("  Target CompID: {}", config.target_comp_id);
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  Heartbeat Interval: {}s", config.heartbeat_interval_secs);

    // Connection process
    println!("\n--- Connection Process ---");
    println!("  1. Create credentials from environment");
    println!("     let credentials = Credentials::from_env()?;");
    println!();
    println!("  2. Create FIX client");
    println!("     let client = FixClient::new(credentials, config);");
    println!();
    println!("  3. Connect to server");
    println!("     client.connect().await?;");
    println!();
    println!("  4. Check session state");
    println!("     let state = client.state().await;");
    println!("     println!(\"Session state: {{:?}}\", state);");

    // Session states
    println!("\n--- Session States ---");
    println!("  Disconnected: Not connected to server");
    println!("  Connecting: TCP connection in progress");
    println!("  LoggingOn: Sending logon message");
    println!("  Active: Session established, ready for trading");
    println!("  LoggingOut: Sending logout message");
    println!("  Reconnecting: Attempting to reconnect");

    // Logon message fields
    println!("\n--- Logon Message Fields ---");
    println!("  MsgType (35): A (Logon)");
    println!("  SenderCompID (49): Your sender ID");
    println!("  TargetCompID (56): ALPACA");
    println!("  MsgSeqNum (34): Message sequence number");
    println!("  SendingTime (52): UTC timestamp");
    println!("  EncryptMethod (98): 0 (None)");
    println!("  HeartBtInt (108): Heartbeat interval in seconds");
    println!("  Username (553): API key");
    println!("  Password (554): API secret");

    // Configuration options
    println!("\n--- Configuration Options ---");
    println!("  version: FIX 4.2 or FIX 4.4");
    println!("  sender_comp_id: Your unique identifier");
    println!("  target_comp_id: Server identifier (ALPACA)");
    println!("  host: FIX server hostname");
    println!("  port: FIX server port");
    println!("  heartbeat_interval_secs: Heartbeat frequency");
    println!("  reconnect_enabled: Auto-reconnect on disconnect");
    println!("  max_reconnect_attempts: Max reconnection tries");
    println!("  log_messages: Enable message logging");

    // Error handling
    println!("\n--- Connection Errors ---");
    println!("  Connection refused: Server not available");
    println!("  Authentication failed: Invalid credentials");
    println!("  Timeout: Server not responding");
    println!("  Sequence gap: Message sequence mismatch");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Use environment variables for credentials");
    println!("2. Set appropriate heartbeat interval (30s typical)");
    println!("3. Implement reconnection logic");
    println!("4. Log all FIX messages for debugging");
    println!("5. Handle session state changes");

    println!("\n=== Example Complete ===");
}
