//! # FIX Order Cancel
//!
//! This example demonstrates how to create FIX Order Cancel and Cancel/Replace messages.
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
//! cargo run -p alpaca-fix --example fix_order_cancel
//! ```
//!
//! **Note**: This example creates message structures but does not send them.
//! FIX protocol requires special access from Alpaca.

use alpaca_fix::{OrdType, OrderCancelReplaceRequest, OrderCancelRequest, Side};

fn main() {
    println!("=== FIX Order Cancel ===\n");

    // Create an order cancel request
    println!("--- Order Cancel Request (MsgType F) ---");
    let cancel_request = OrderCancelRequest::new("original-order-id-123", "AAPL", Side::Buy);

    println!(
        "  Original Client Order ID: {}",
        cancel_request.orig_cl_ord_id
    );
    println!("  New Client Order ID: {}", cancel_request.cl_ord_id);
    println!("  Symbol: {}", cancel_request.symbol);
    println!("  Side: {:?}", cancel_request.side);

    // Create an order cancel/replace request (modify order)
    println!("\n--- Order Cancel/Replace Request (MsgType G) ---");
    let replace_request = OrderCancelReplaceRequest::new(
        "original-order-id-456",
        "MSFT",
        Side::Sell,
        OrdType::Limit,
        75.0,
    )
    .with_price(355.00);

    println!(
        "  Original Client Order ID: {}",
        replace_request.orig_cl_ord_id
    );
    println!("  New Client Order ID: {}", replace_request.cl_ord_id);
    println!("  Symbol: {}", replace_request.symbol);
    println!("  Side: {:?}", replace_request.side);
    println!("  Order Type: {:?}", replace_request.ord_type);
    println!("  New Quantity: {}", replace_request.order_qty);
    println!("  New Price: ${:.2}", replace_request.price.unwrap_or(0.0));

    // Demonstrate cancel workflow
    println!("\n--- Cancel Workflow ---");
    println!("  1. Send NewOrderSingle (MsgType D)");
    println!("     -> Receive ExecutionReport with OrdStatus=New");
    println!("  2. Send OrderCancelRequest (MsgType F)");
    println!("     -> Receive ExecutionReport with OrdStatus=Canceled");
    println!("     OR");
    println!("     -> Receive OrderCancelReject if cancel fails");

    // Demonstrate replace workflow
    println!("\n--- Replace Workflow ---");
    println!("  1. Send NewOrderSingle (MsgType D)");
    println!("     -> Receive ExecutionReport with OrdStatus=New");
    println!("  2. Send OrderCancelReplaceRequest (MsgType G)");
    println!("     -> Receive ExecutionReport with OrdStatus=Replaced");
    println!("     OR");
    println!("     -> Receive OrderCancelReject if replace fails");

    // FIX message type codes
    println!("\n--- FIX Message Types ---");
    println!("  D = New Order Single");
    println!("  F = Order Cancel Request");
    println!("  G = Order Cancel/Replace Request");
    println!("  8 = Execution Report");
    println!("  9 = Order Cancel Reject");

    println!("\n=== Example Complete ===");
}
