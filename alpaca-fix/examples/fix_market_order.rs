//! # FIX Market Order
//!
//! This example demonstrates how to create market orders via FIX protocol.
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
//! cargo run -p alpaca-fix --example fix_market_order
//! ```
//!
//! **Note**: FIX protocol requires special access from Alpaca.

use alpaca_fix::messages::{NewOrderSingle, OrdType, Side, TimeInForce};

fn main() {
    println!("=== FIX Market Order ===\n");

    // Create market buy order
    println!("--- Market Buy Order ---");
    let buy_order = NewOrderSingle::market("AAPL", Side::Buy, 100.0)
        .with_cl_ord_id("MKT-BUY-001")
        .with_time_in_force(TimeInForce::Day);

    println!("  Symbol: {}", buy_order.symbol);
    println!("  Side: {:?}", buy_order.side);
    println!("  Order Type: {:?}", buy_order.ord_type);
    println!("  Quantity: {}", buy_order.order_qty);
    println!("  Time in Force: {:?}", buy_order.time_in_force);
    println!("  Client Order ID: {}", buy_order.cl_ord_id);

    // Create market sell order
    println!("\n--- Market Sell Order ---");
    let sell_order = NewOrderSingle::market("MSFT", Side::Sell, 50.0)
        .with_cl_ord_id("MKT-SELL-001")
        .with_time_in_force(TimeInForce::Day);

    println!("  Symbol: {}", sell_order.symbol);
    println!("  Side: {:?}", sell_order.side);
    println!("  Order Type: {:?}", sell_order.ord_type);
    println!("  Quantity: {}", sell_order.order_qty);

    // Show all order types
    println!("\n--- Order Types ---");
    let order_types = [
        OrdType::Market,
        OrdType::Limit,
        OrdType::Stop,
        OrdType::StopLimit,
    ];
    for ot in order_types {
        println!("  {:?} = '{}'", ot, ot.as_char());
    }

    // Show all sides
    println!("\n--- Order Sides ---");
    let sides = [Side::Buy, Side::Sell];
    for s in sides {
        println!("  {:?} = '{}'", s, s.as_char());
    }

    // Show time in force options
    println!("\n--- Time in Force ---");
    let tifs = [
        TimeInForce::Day,
        TimeInForce::Gtc,
        TimeInForce::Ioc,
        TimeInForce::Fok,
    ];
    for tif in tifs {
        println!("  {:?} = '{}'", tif, tif.as_char());
    }

    // FIX message fields
    println!("\n--- FIX Fields (New Order Single) ---");
    println!("  MsgType (35): D");
    println!("  ClOrdID (11): {}", buy_order.cl_ord_id);
    println!("  Symbol (55): {}", buy_order.symbol);
    println!("  Side (54): {}", buy_order.side.as_char());
    println!("  OrdType (40): {}", buy_order.ord_type.as_char());
    println!("  OrderQty (38): {}", buy_order.order_qty);
    println!("  TimeInForce (59): {}", buy_order.time_in_force.as_char());

    // Usage note
    println!("\n--- Sending Orders ---");
    println!("To send via FIX (requires active session):");
    println!("  let cl_ord_id = client.send_order(&buy_order).await?;");

    println!("\n=== Example Complete ===");
}
