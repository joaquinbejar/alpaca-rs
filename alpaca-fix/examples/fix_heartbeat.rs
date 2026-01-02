//! # FIX Heartbeat
//!
//! This example demonstrates heartbeat handling in FIX protocol
//! using the Alpaca FIX client.
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
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

fn main() {
    println!("=== FIX Heartbeat ===\n");

    // Heartbeat overview
    println!("--- Heartbeat Overview ---");
    println!("  Heartbeats maintain FIX session connectivity.");
    println!("  They are exchanged at regular intervals to:");
    println!("    - Verify connection is alive");
    println!("    - Detect network failures");
    println!("    - Prevent TCP timeouts");

    // Heartbeat configuration
    println!("\n--- Heartbeat Configuration ---");
    println!("  let config = FixConfig::builder()");
    println!("      .heartbeat_interval(30) // 30 seconds");
    println!("      .build();");
    println!();
    println!("  Typical intervals: 30-60 seconds");
    println!("  Negotiated during logon");

    // Heartbeat message
    println!("\n--- Heartbeat Message (MsgType 0) ---");
    println!("  MsgType (35): 0 (Heartbeat)");
    println!("  SenderCompID (49): Your sender ID");
    println!("  TargetCompID (56): ALPACA");
    println!("  MsgSeqNum (34): Sequence number");
    println!("  SendingTime (52): UTC timestamp");
    println!("  TestReqID (112): Optional, echoes TestRequest");

    // Test request message
    println!("\n--- Test Request Message (MsgType 1) ---");
    println!("  Sent when no message received within heartbeat interval.");
    println!("  MsgType (35): 1 (TestRequest)");
    println!("  TestReqID (112): Unique identifier");
    println!();
    println!("  Response: Heartbeat with matching TestReqID");

    // Automatic heartbeat handling
    println!("\n--- Automatic Heartbeat Handling ---");
    println!("  The FIX client handles heartbeats automatically:");
    println!();
    println!("  1. Background task sends heartbeats at configured interval");
    println!("  2. Responds to TestRequest with Heartbeat");
    println!("  3. Sends TestRequest if no message received");
    println!("  4. Detects connection loss if no response");

    // Heartbeat flow
    println!("\n--- Heartbeat Flow ---");
    println!("  Normal operation:");
    println!("    Client -> Heartbeat -> Server");
    println!("    Server -> Heartbeat -> Client");
    println!();
    println!("  Connection check:");
    println!("    Client -> TestRequest (ID=123) -> Server");
    println!("    Server -> Heartbeat (TestReqID=123) -> Client");

    // Connection loss detection
    println!("\n--- Connection Loss Detection ---");
    println!("  If no message received within:");
    println!("    HeartBtInt + reasonable transmission time");
    println!();
    println!("  Then:");
    println!("    1. Send TestRequest");
    println!("    2. Wait for Heartbeat response");
    println!("    3. If no response, consider connection lost");
    println!("    4. Initiate reconnection if configured");

    // Manual heartbeat (if needed)
    println!("\n--- Manual Heartbeat (Advanced) ---");
    println!("  // Usually not needed - handled automatically");
    println!("  // But available for custom implementations:");
    println!("  let heartbeat = session.create_heartbeat(None);");
    println!("  client.send_raw(&heartbeat).await?;");

    // Logon heartbeat negotiation
    println!("\n--- Logon Heartbeat Negotiation ---");
    println!("  During logon, HeartBtInt (108) is exchanged:");
    println!();
    println!("  Client Logon:");
    println!("    HeartBtInt=30 (proposed interval)");
    println!();
    println!("  Server Logon:");
    println!("    HeartBtInt=30 (confirmed interval)");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Use 30-second interval for most cases");
    println!("2. Don't set interval too low (network overhead)");
    println!("3. Don't set interval too high (slow failure detection)");
    println!("4. Monitor heartbeat failures for connection issues");
    println!("5. Implement reconnection on heartbeat timeout");

    println!("\n=== Example Complete ===");
}
