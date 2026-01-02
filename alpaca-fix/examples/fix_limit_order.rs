//! # FIX Limit Order
//!
//! This example demonstrates how to create limit orders via FIX protocol.
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
//! cargo run -p alpaca-fix --example fix_limit_order
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_fix::messages::{NewOrderSingle, Side, TimeInForce};

fn main() {
    println!("=== FIX Limit Order ===\n");

    // Create limit buy order
    println!("--- Limit Buy Order ---");
    let buy_order = NewOrderSingle::limit("AAPL", Side::Buy, 100.0, 150.00)
        .with_cl_ord_id("LMT-BUY-001")
        .with_time_in_force(TimeInForce::Gtc);

    println!("  Symbol: {}", buy_order.symbol);
    println!("  Side: {:?}", buy_order.side);
    println!("  Order Type: {:?}", buy_order.ord_type);
    println!("  Quantity: {}", buy_order.order_qty);
    println!("  Limit Price: {:?}", buy_order.price);
    println!("  Time in Force: {:?}", buy_order.time_in_force);
    println!("  Client Order ID: {}", buy_order.cl_ord_id);

    // Create limit sell order
    println!("\n--- Limit Sell Order ---");
    let sell_order = NewOrderSingle::limit("MSFT", Side::Sell, 50.0, 400.00)
        .with_cl_ord_id("LMT-SELL-001")
        .with_time_in_force(TimeInForce::Day);

    println!("  Symbol: {}", sell_order.symbol);
    println!("  Side: {:?}", sell_order.side);
    println!("  Order Type: {:?}", sell_order.ord_type);
    println!("  Quantity: {}", sell_order.order_qty);
    println!("  Limit Price: {:?}", sell_order.price);
    println!("  Time in Force: {:?}", sell_order.time_in_force);

    // Create stop order
    println!("\n--- Stop Order ---");
    let stop_order =
        NewOrderSingle::stop("GOOGL", Side::Sell, 25.0, 140.00).with_cl_ord_id("STP-SELL-001");

    println!("  Symbol: {}", stop_order.symbol);
    println!("  Side: {:?}", stop_order.side);
    println!("  Order Type: {:?}", stop_order.ord_type);
    println!("  Quantity: {}", stop_order.order_qty);
    println!("  Stop Price: {:?}", stop_order.stop_px);

    // FIX message fields
    println!("\n--- FIX Fields (Limit Order) ---");
    println!("  MsgType (35): D");
    println!("  ClOrdID (11): {}", buy_order.cl_ord_id);
    println!("  Symbol (55): {}", buy_order.symbol);
    println!("  Side (54): {}", buy_order.side.as_char());
    println!("  OrdType (40): {}", buy_order.ord_type.as_char());
    println!("  Price (44): {}", buy_order.price.unwrap_or(0.0));
    println!("  OrderQty (38): {}", buy_order.order_qty);
    println!("  TimeInForce (59): {}", buy_order.time_in_force.as_char());

    // Usage note
    println!("\n--- Sending Orders ---");
    println!("To send via FIX (requires active session):");
    println!("  let cl_ord_id = client.send_order(&buy_order).await?;");

    println!("\n=== Example Complete ===");
}
