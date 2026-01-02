//! # FIX Cancel Order
//!
//! This example demonstrates how to cancel an order via FIX protocol
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
//! cargo run -p alpaca-fix --example fix_cancel_order
//! ```
//!
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

use alpaca_fix::messages::{OrderCancelRequest, Side};

fn main() {
    println!("=== FIX Cancel Order ===\n");

    // Create cancel request
    println!("--- Create Cancel Request ---");
    let cancel = OrderCancelRequest::new(
        "original-order-001", // Original client order ID
        "AAPL",               // Symbol
        Side::Buy,            // Original order side
    );

    println!("  Original ClOrdID: {}", cancel.orig_cl_ord_id);
    println!("  Cancel ClOrdID: {}", cancel.cl_ord_id);
    println!("  Symbol: {}", cancel.symbol);
    println!("  Side: {:?}", cancel.side);

    // Send cancel request
    println!("\n--- Send Cancel Request ---");
    println!("  let cl_ord_id = client.cancel_order(&cancel).await?;");
    println!("  println!(\"Cancel request sent: {{}}\", cl_ord_id);");

    // Order Cancel Request FIX fields
    println!("\n--- Cancel Request FIX Fields ---");
    println!("  MsgType (35): F (Order Cancel Request)");
    println!("  OrigClOrdID (41): Original client order ID");
    println!("  ClOrdID (11): New client order ID for cancel");
    println!("  Symbol (55): Stock symbol");
    println!("  Side (54): Original order side");
    println!("  TransactTime (60): Transaction timestamp");

    // Cancel response scenarios
    println!("\n--- Cancel Response Scenarios ---");
    println!("  1. Cancel Accepted:");
    println!("     - ExecutionReport with ExecType=Canceled");
    println!("     - OrdStatus=Canceled");
    println!();
    println!("  2. Cancel Rejected:");
    println!("     - OrderCancelReject message");
    println!("     - CxlRejReason indicates why");
    println!();
    println!("  3. Too Late to Cancel:");
    println!("     - Order already filled");
    println!("     - OrderCancelReject with reason");

    // Handle cancel response
    println!("\n--- Handle Cancel Response ---");
    println!("  let msg = client.next_message().await?;");
    println!("  if let Some(msg_type) = msg.msg_type() {{");
    println!("      match msg_type {{");
    println!("          \"8\" => {{ // ExecutionReport");
    println!("              let report = client.parse_execution_report(&msg)?;");
    println!("              if report.ord_status == OrdStatus::Canceled {{");
    println!("                  println!(\"Order canceled successfully\");");
    println!("              }}");
    println!("          }}");
    println!("          \"9\" => {{ // OrderCancelReject");
    println!("              println!(\"Cancel rejected\");");
    println!("              // Parse rejection reason");
    println!("          }}");
    println!("          _ => {{}}");
    println!("      }}");
    println!("  }}");

    // Cancel rejection reasons
    println!("\n--- Cancel Rejection Reasons ---");
    println!("  0: Too late to cancel");
    println!("  1: Unknown order");
    println!("  2: Broker option");
    println!("  3: Order already pending cancel");
    println!("  4: Unable to process request");
    println!("  5: OrigOrdModTime did not match");
    println!("  6: Duplicate ClOrdID");

    // Cancel vs Cancel/Replace
    println!("\n--- Cancel vs Cancel/Replace ---");
    println!("  Cancel (MsgType F):");
    println!("    - Completely cancels the order");
    println!("    - No replacement order");
    println!();
    println!("  Cancel/Replace (MsgType G):");
    println!("    - Cancels and replaces with new order");
    println!("    - Can modify price, quantity, etc.");
    println!("    - Atomic operation");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Use unique ClOrdID for each cancel request");
    println!("2. Track original order ID accurately");
    println!("3. Handle race conditions (fill vs cancel)");
    println!("4. Implement timeout for cancel confirmation");
    println!("5. Log all cancel attempts and responses");

    println!("\n=== Example Complete ===");
}
