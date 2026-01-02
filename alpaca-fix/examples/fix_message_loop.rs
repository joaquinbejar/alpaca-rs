//! # FIX Message Loop
//!
//! This example demonstrates FIX message types and processing patterns.
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
//! cargo run -p alpaca-fix --example fix_message_loop
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_fix::messages::MsgType;

fn main() {
    println!("=== FIX Message Loop ===\n");

    // Show all message types
    println!("--- FIX Message Types ---");
    let msg_types = [
        (MsgType::Heartbeat, "Session heartbeat"),
        (MsgType::TestRequest, "Connection test"),
        (MsgType::ResendRequest, "Request message resend"),
        (MsgType::Reject, "Message rejected"),
        (MsgType::SequenceReset, "Reset sequence numbers"),
        (MsgType::Logout, "Session logout"),
        (MsgType::Logon, "Session logon"),
        (MsgType::NewOrderSingle, "New order"),
        (MsgType::ExecutionReport, "Order status update"),
        (MsgType::OrderCancelRequest, "Cancel order"),
        (MsgType::OrderCancelReplaceRequest, "Modify order"),
        (MsgType::MarketDataRequest, "Request market data"),
    ];

    for (msg_type, description) in msg_types {
        println!(
            "  {} = '{}': {}",
            format!("{:?}", msg_type)
                .chars()
                .take(20)
                .collect::<String>(),
            msg_type.as_str(),
            description
        );
    }

    // Message categories
    println!("\n--- Message Categories ---");
    println!("  Session messages: Heartbeat, TestRequest, Logon, Logout");
    println!("  Order messages: NewOrderSingle, ExecutionReport");
    println!("  Cancel messages: OrderCancelRequest, OrderCancelReplaceRequest");
    println!("  Admin messages: ResendRequest, Reject, SequenceReset");

    // Message loop pattern
    println!("\n--- Message Loop Pattern ---");
    println!("  loop {{");
    println!("      let msg = client.next_message().await?;");
    println!("      if let Some(msg_type) = msg.msg_type() {{");
    println!("          match msg_type {{");
    println!("              \"0\" => {{ /* Heartbeat */ }}");
    println!("              \"8\" => {{ /* ExecutionReport */ }}");
    println!("              \"5\" => break, // Logout");
    println!("              _ => {{}}");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Processing methods
    println!("\n--- Processing Methods ---");
    println!("  client.process_message(&msg).await?  // Handle session messages");
    println!("  client.parse_execution_report(&msg)? // Parse execution report");

    println!("\n=== Example Complete ===");
}
