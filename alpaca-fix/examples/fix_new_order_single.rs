//! # FIX New Order Single
//!
//! This example demonstrates how to create FIX New Order Single messages.
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
//! cargo run -p alpaca-fix --example fix_new_order_single
//! ```
//!
//! **Note**: This example creates message structures but does not send them.
//! FIX protocol requires special access from Alpaca.

use alpaca_fix::{NewOrderSingle, OrdType, Side, TimeInForce};

fn main() {
    println!("=== FIX New Order Single ===\n");

    // Create a market order
    println!("--- Market Order ---");
    let market_order = NewOrderSingle::market("AAPL", Side::Buy, 100.0);

    println!("  Client Order ID: {}", market_order.cl_ord_id);
    println!("  Symbol: {}", market_order.symbol);
    println!("  Side: {:?}", market_order.side);
    println!("  Order Type: {:?}", market_order.ord_type);
    println!("  Quantity: {}", market_order.order_qty);
    println!("  Time in Force: {:?}", market_order.time_in_force);

    // Create a limit order
    println!("\n--- Limit Order ---");
    let limit_order = NewOrderSingle::limit("MSFT", Side::Sell, 50.0, 350.00)
        .with_time_in_force(TimeInForce::Gtc);

    println!("  Client Order ID: {}", limit_order.cl_ord_id);
    println!("  Symbol: {}", limit_order.symbol);
    println!("  Side: {:?}", limit_order.side);
    println!("  Order Type: {:?}", limit_order.ord_type);
    println!("  Quantity: {}", limit_order.order_qty);
    println!("  Price: ${:.2}", limit_order.price.unwrap_or(0.0));
    println!("  Time in Force: {:?}", limit_order.time_in_force);

    // Create a stop order
    println!("\n--- Stop Order ---");
    let stop_order = NewOrderSingle::stop("TSLA", Side::Sell, 25.0, 200.00);

    println!("  Client Order ID: {}", stop_order.cl_ord_id);
    println!("  Symbol: {}", stop_order.symbol);
    println!("  Side: {:?}", stop_order.side);
    println!("  Order Type: {:?}", stop_order.ord_type);
    println!("  Quantity: {}", stop_order.order_qty);
    println!("  Stop Price: ${:.2}", stop_order.stop_px.unwrap_or(0.0));

    // Create order with custom client order ID
    println!("\n--- Order with Custom ID ---");
    let custom_order = NewOrderSingle::market("GOOGL", Side::Buy, 10.0)
        .with_cl_ord_id("MY-ORDER-001")
        .with_account("my-account-id");

    println!("  Client Order ID: {}", custom_order.cl_ord_id);
    println!("  Symbol: {}", custom_order.symbol);
    println!("  Account: {:?}", custom_order.account);

    // FIX tag values
    println!("\n--- FIX Tag Values ---");
    println!("  Side Buy (Tag 54): {}", Side::Buy.as_char());
    println!("  Side Sell (Tag 54): {}", Side::Sell.as_char());
    println!("  OrdType Market (Tag 40): {}", OrdType::Market.as_char());
    println!("  OrdType Limit (Tag 40): {}", OrdType::Limit.as_char());
    println!("  TimeInForce Day (Tag 59): {}", TimeInForce::Day.as_char());
    println!("  TimeInForce GTC (Tag 59): {}", TimeInForce::Gtc.as_char());

    println!("\n=== Example Complete ===");
}
