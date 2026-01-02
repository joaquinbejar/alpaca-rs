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
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

fn main() {
    println!("=== FIX Disconnect ===\n");

    // Graceful disconnect process
    println!("--- Graceful Disconnect ---");
    println!("  1. Check current session state");
    println!("     let state = client.state().await;");
    println!("     if state == SessionState::Active {{");
    println!();
    println!("  2. Send logout message");
    println!("     client.disconnect().await?;");
    println!();
    println!("  3. Wait for logout confirmation");
    println!("     // Client waits up to 5 seconds for server response");
    println!();
    println!("  4. Close TCP connection");
    println!("     // Connection is closed automatically");

    // Logout message fields
    println!("\n--- Logout Message Fields ---");
    println!("  MsgType (35): 5 (Logout)");
    println!("  SenderCompID (49): Your sender ID");
    println!("  TargetCompID (56): ALPACA");
    println!("  MsgSeqNum (34): Next sequence number");
    println!("  SendingTime (52): UTC timestamp");
    println!("  Text (58): Optional logout reason");

    // Disconnect scenarios
    println!("\n--- Disconnect Scenarios ---");
    println!("  Normal logout:");
    println!("    - Send Logout message");
    println!("    - Receive Logout confirmation");
    println!("    - Close connection");
    println!();
    println!("  Server-initiated logout:");
    println!("    - Receive Logout from server");
    println!("    - Send Logout response");
    println!("    - Close connection");
    println!();
    println!("  Connection lost:");
    println!("    - Detect TCP disconnect");
    println!("    - Update session state");
    println!("    - Optionally attempt reconnect");

    // Session state transitions
    println!("\n--- State Transitions on Disconnect ---");
    println!("  Active -> LoggingOut -> Disconnected");
    println!();
    println!("  if client.state().await == SessionState::Active {{");
    println!("      client.disconnect().await?;");
    println!("      assert_eq!(client.state().await, SessionState::Disconnected);");
    println!("  }}");

    // Cleanup
    println!("\n--- Cleanup Tasks ---");
    println!("  1. Stop heartbeat background task");
    println!("  2. Stop message receiver task");
    println!("  3. Flush pending messages");
    println!("  4. Close message channels");
    println!("  5. Close TCP socket");

    // Reconnection after disconnect
    println!("\n--- Reconnection After Disconnect ---");
    println!("  // After disconnect, you can reconnect");
    println!("  client.disconnect().await?;");
    println!("  // ... some time later ...");
    println!("  client.connect().await?;");
    println!("  // Session is re-established with new sequence numbers");

    // Error handling
    println!("\n--- Disconnect Errors ---");
    println!("  Timeout: Server didn't respond to logout");
    println!("  Already disconnected: Session not active");
    println!("  Network error: Connection already broken");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Always disconnect gracefully when possible");
    println!("2. Handle server-initiated logouts");
    println!("3. Implement shutdown hooks for clean exit");
    println!("4. Log disconnect reasons for debugging");
    println!("5. Reset state before reconnecting");

    println!("\n=== Example Complete ===");
}
