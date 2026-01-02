//! # FIX Execution Reports
//!
//! This example demonstrates how to process execution reports via FIX protocol
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
//! cargo run -p alpaca-fix --example fix_execution_reports
//! ```
//!
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

use alpaca_fix::messages::{ExecType, OrdStatus};

fn main() {
    println!("=== FIX Execution Reports ===\n");

    // Execution report overview
    println!("--- Execution Report Overview ---");
    println!("  Execution reports (MsgType 8) provide order status updates.");
    println!("  They are sent by the server in response to:");
    println!("    - New orders");
    println!("    - Order modifications");
    println!("    - Order cancellations");
    println!("    - Fills and partial fills");

    // Parse execution report
    println!("\n--- Parse Execution Report ---");
    println!("  let msg = client.next_message().await?;");
    println!("  if msg.msg_type() == Some(\"8\") {{");
    println!("      let report = client.parse_execution_report(&msg)?;");
    println!("      println!(\"Order ID: {{}}\", report.order_id);");
    println!("      println!(\"Client Order ID: {{}}\", report.cl_ord_id);");
    println!("      println!(\"Exec Type: {{:?}}\", report.exec_type);");
    println!("      println!(\"Order Status: {{:?}}\", report.ord_status);");
    println!("  }}");

    // Execution report fields
    println!("\n--- Execution Report Fields ---");
    println!("  order_id: Server-assigned order ID");
    println!("  cl_ord_id: Client order ID");
    println!("  exec_id: Unique execution ID");
    println!("  exec_type: Type of execution event");
    println!("  ord_status: Current order status");
    println!("  symbol: Stock symbol");
    println!("  side: Buy or Sell");
    println!("  order_qty: Total order quantity");
    println!("  last_qty: Quantity of last fill");
    println!("  last_px: Price of last fill");
    println!("  cum_qty: Cumulative filled quantity");
    println!("  avg_px: Average fill price");
    println!("  leaves_qty: Remaining quantity");
    println!("  text: Optional message text");

    // Execution types
    println!("\n--- Execution Types (ExecType) ---");
    let exec_types = [
        (ExecType::New, "Order accepted"),
        (ExecType::PartialFill, "Partial fill occurred"),
        (ExecType::Fill, "Order completely filled"),
        (ExecType::Canceled, "Order canceled"),
        (ExecType::Replaced, "Order replaced"),
        (ExecType::PendingCancel, "Cancel pending"),
        (ExecType::Rejected, "Order rejected"),
        (ExecType::PendingNew, "New order pending"),
        (ExecType::Expired, "Order expired"),
    ];

    for (exec_type, description) in exec_types {
        println!("  {:?}: {}", exec_type, description);
    }

    // Order statuses
    println!("\n--- Order Statuses (OrdStatus) ---");
    let ord_statuses = [
        (OrdStatus::New, "Order accepted, not yet filled"),
        (OrdStatus::PartiallyFilled, "Partially filled"),
        (OrdStatus::Filled, "Completely filled"),
        (OrdStatus::Canceled, "Order canceled"),
        (OrdStatus::Replaced, "Order replaced"),
        (OrdStatus::PendingCancel, "Cancel request pending"),
        (OrdStatus::Rejected, "Order rejected"),
        (OrdStatus::PendingNew, "New order pending"),
        (OrdStatus::Expired, "Order expired"),
    ];

    for (status, description) in ord_statuses {
        println!("  {:?}: {}", status, description);
    }

    // Handle different execution types
    println!("\n--- Handle Execution Types ---");
    println!("  match report.exec_type {{");
    println!("      ExecType::New => {{");
    println!("          println!(\"Order accepted: {{}}\", report.order_id);");
    println!("      }}");
    println!("      ExecType::Fill => {{");
    println!("          println!(\"Filled: {{}} @ {{}}\", report.cum_qty, report.avg_px);");
    println!("      }}");
    println!("      ExecType::PartialFill => {{");
    println!("          let filled = report.last_qty.unwrap_or(0.0);");
    println!("          let price = report.last_px.unwrap_or(0.0);");
    println!("          println!(\"Partial: {{}} @ {{}}, remaining: {{}}\",");
    println!("              filled, price, report.leaves_qty);");
    println!("      }}");
    println!("      ExecType::Canceled => {{");
    println!("          println!(\"Canceled: {{}}\", report.cl_ord_id);");
    println!("      }}");
    println!("      ExecType::Rejected => {{");
    println!("          println!(\"Rejected: {{:?}}\", report.text);");
    println!("      }}");
    println!("      _ => {{}}");
    println!("  }}");

    // FIX message tags
    println!("\n--- Execution Report FIX Tags ---");
    println!("  MsgType (35): 8");
    println!("  OrderID (37): Server order ID");
    println!("  ClOrdID (11): Client order ID");
    println!("  ExecID (17): Execution ID");
    println!("  ExecType (150): Execution type");
    println!("  OrdStatus (39): Order status");
    println!("  Symbol (55): Stock symbol");
    println!("  Side (54): Order side");
    println!("  OrderQty (38): Order quantity");
    println!("  LastQty (32): Last fill quantity");
    println!("  LastPx (31): Last fill price");
    println!("  CumQty (14): Cumulative quantity");
    println!("  AvgPx (6): Average price");
    println!("  LeavesQty (151): Remaining quantity");
    println!("  Text (58): Message text");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Process all execution reports in order");
    println!("2. Track order state transitions");
    println!("3. Handle partial fills correctly");
    println!("4. Store execution IDs for reconciliation");
    println!("5. Log all execution reports");

    println!("\n=== Example Complete ===");
}
