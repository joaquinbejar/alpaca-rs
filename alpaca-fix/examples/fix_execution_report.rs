//! # FIX Execution Report
//!
//! This example demonstrates FIX Execution Report message structure and parsing.
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
//! cargo run -p alpaca-fix --example fix_execution_report
//! ```
//!
//! **Note**: This example demonstrates message structures.
//! FIX protocol requires special access from Alpaca.

use alpaca_fix::{ExecType, ExecutionReport, OrdStatus, Side};

fn main() {
    println!("=== FIX Execution Report ===\n");

    // Simulate an execution report for a new order
    println!("--- New Order Execution Report ---");
    let new_order_report = ExecutionReport {
        order_id: "alpaca-order-123".to_string(),
        cl_ord_id: "client-order-456".to_string(),
        exec_id: "exec-789".to_string(),
        exec_type: ExecType::New,
        ord_status: OrdStatus::New,
        symbol: "AAPL".to_string(),
        side: Side::Buy,
        order_qty: 100.0,
        last_qty: None,
        last_px: None,
        cum_qty: 0.0,
        avg_px: 0.0,
        leaves_qty: 100.0,
        text: None,
    };

    print_execution_report(&new_order_report);

    // Simulate a partial fill
    println!("\n--- Partial Fill Execution Report ---");
    let partial_fill = ExecutionReport {
        order_id: "alpaca-order-123".to_string(),
        cl_ord_id: "client-order-456".to_string(),
        exec_id: "exec-790".to_string(),
        exec_type: ExecType::PartialFill,
        ord_status: OrdStatus::PartiallyFilled,
        symbol: "AAPL".to_string(),
        side: Side::Buy,
        order_qty: 100.0,
        last_qty: Some(50.0),
        last_px: Some(175.50),
        cum_qty: 50.0,
        avg_px: 175.50,
        leaves_qty: 50.0,
        text: None,
    };

    print_execution_report(&partial_fill);

    // Simulate a full fill
    println!("\n--- Full Fill Execution Report ---");
    let full_fill = ExecutionReport {
        order_id: "alpaca-order-123".to_string(),
        cl_ord_id: "client-order-456".to_string(),
        exec_id: "exec-791".to_string(),
        exec_type: ExecType::Fill,
        ord_status: OrdStatus::Filled,
        symbol: "AAPL".to_string(),
        side: Side::Buy,
        order_qty: 100.0,
        last_qty: Some(50.0),
        last_px: Some(175.55),
        cum_qty: 100.0,
        avg_px: 175.525,
        leaves_qty: 0.0,
        text: None,
    };

    print_execution_report(&full_fill);

    // Simulate a rejection
    println!("\n--- Rejected Order Execution Report ---");
    let rejected = ExecutionReport {
        order_id: "alpaca-order-999".to_string(),
        cl_ord_id: "client-order-999".to_string(),
        exec_id: "exec-999".to_string(),
        exec_type: ExecType::Rejected,
        ord_status: OrdStatus::Rejected,
        symbol: "INVALID".to_string(),
        side: Side::Buy,
        order_qty: 100.0,
        last_qty: None,
        last_px: None,
        cum_qty: 0.0,
        avg_px: 0.0,
        leaves_qty: 0.0,
        text: Some("Unknown symbol".to_string()),
    };

    print_execution_report(&rejected);

    // Execution type reference
    println!("\n--- Execution Types (Tag 150) ---");
    println!("  0 = New");
    println!("  1 = Partial Fill");
    println!("  2 = Fill");
    println!("  4 = Canceled");
    println!("  5 = Replaced");
    println!("  6 = Pending Cancel");
    println!("  8 = Rejected");
    println!("  A = Pending New");
    println!("  C = Expired");

    // Order status reference
    println!("\n--- Order Status (Tag 39) ---");
    println!("  0 = New");
    println!("  1 = Partially Filled");
    println!("  2 = Filled");
    println!("  4 = Canceled");
    println!("  8 = Rejected");

    println!("\n=== Example Complete ===");
}

fn print_execution_report(report: &ExecutionReport) {
    println!("  Order ID: {}", report.order_id);
    println!("  Client Order ID: {}", report.cl_ord_id);
    println!("  Exec ID: {}", report.exec_id);
    println!(
        "  Exec Type: {:?} ({})",
        report.exec_type,
        report.exec_type.as_char()
    );
    println!(
        "  Order Status: {:?} ({})",
        report.ord_status,
        report.ord_status.as_char()
    );
    println!("  Symbol: {}", report.symbol);
    println!("  Side: {:?}", report.side);
    println!("  Order Qty: {}", report.order_qty);
    if let Some(last_qty) = report.last_qty {
        println!("  Last Qty: {}", last_qty);
    }
    if let Some(last_px) = report.last_px {
        println!("  Last Price: ${:.2}", last_px);
    }
    println!("  Cum Qty: {}", report.cum_qty);
    println!("  Avg Price: ${:.3}", report.avg_px);
    println!("  Leaves Qty: {}", report.leaves_qty);
    if let Some(text) = &report.text {
        println!("  Text: {}", text);
    }
}
