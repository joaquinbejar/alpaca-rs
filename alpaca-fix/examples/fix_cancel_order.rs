//! # FIX Cancel Order
//!
//! This example demonstrates how to create cancel requests via FIX protocol.
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
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_fix::messages::{OrderCancelRequest, Side};

fn main() {
    println!("=== FIX Cancel Order ===\n");

    // Create cancel request for a buy order
    println!("--- Cancel Buy Order ---");
    let cancel_buy = OrderCancelRequest::new(
        "MKT-BUY-001", // Original client order ID to cancel
        "AAPL",        // Symbol
        Side::Buy,     // Original order side
    );

    println!("  Original ClOrdID: {}", cancel_buy.orig_cl_ord_id);
    println!("  Cancel ClOrdID: {}", cancel_buy.cl_ord_id);
    println!("  Symbol: {}", cancel_buy.symbol);
    println!("  Side: {:?}", cancel_buy.side);

    // Create cancel request for a sell order
    println!("\n--- Cancel Sell Order ---");
    let cancel_sell = OrderCancelRequest::new(
        "LMT-SELL-001", // Original client order ID to cancel
        "MSFT",         // Symbol
        Side::Sell,     // Original order side
    );

    println!("  Original ClOrdID: {}", cancel_sell.orig_cl_ord_id);
    println!("  Cancel ClOrdID: {}", cancel_sell.cl_ord_id);
    println!("  Symbol: {}", cancel_sell.symbol);
    println!("  Side: {:?}", cancel_sell.side);

    // FIX message fields
    println!("\n--- FIX Fields (Order Cancel Request) ---");
    println!("  MsgType (35): F");
    println!("  OrigClOrdID (41): {}", cancel_buy.orig_cl_ord_id);
    println!("  ClOrdID (11): {}", cancel_buy.cl_ord_id);
    println!("  Symbol (55): {}", cancel_buy.symbol);
    println!("  Side (54): {}", cancel_buy.side.as_char());

    // Cancel rejection reasons
    println!("\n--- Cancel Rejection Reasons (CxlRejReason) ---");
    println!("  0: Too late to cancel");
    println!("  1: Unknown order");
    println!("  2: Broker option");
    println!("  3: Order already pending cancel");
    println!("  4: Unable to process request");
    println!("  5: OrigOrdModTime did not match");
    println!("  6: Duplicate ClOrdID");

    // Usage note
    println!("\n--- Sending Cancel ---");
    println!("To send via FIX (requires active session):");
    println!("  let cl_ord_id = client.cancel_order(&cancel_buy).await?;");

    println!("\n=== Example Complete ===");
}
