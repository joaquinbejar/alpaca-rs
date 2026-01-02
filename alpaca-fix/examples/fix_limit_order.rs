//! # FIX Limit Order
//!
//! This example demonstrates how to send a limit order via FIX protocol
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
//! cargo run -p alpaca-fix --example fix_limit_order
//! ```
//!
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

use alpaca_fix::messages::{NewOrderSingle, Side, TimeInForce};

fn main() {
    println!("=== FIX Limit Order ===\n");

    // Create limit buy order
    println!("--- Create Limit Buy Order ---");
    let buy_order = NewOrderSingle::limit("AAPL", Side::Buy, 100.0, 150.00)
        .with_cl_ord_id("limit-001")
        .with_time_in_force(TimeInForce::Gtc);

    println!("  Symbol: {}", buy_order.symbol);
    println!("  Side: {:?}", buy_order.side);
    println!("  Order Type: {:?}", buy_order.ord_type);
    println!("  Quantity: {}", buy_order.order_qty);
    println!("  Limit Price: {:?}", buy_order.price);
    println!("  Time in Force: {:?}", buy_order.time_in_force);
    println!("  Client Order ID: {}", buy_order.cl_ord_id);

    // Create limit sell order
    println!("\n--- Create Limit Sell Order ---");
    let sell_order = NewOrderSingle::limit("MSFT", Side::Sell, 50.0, 400.00)
        .with_cl_ord_id("limit-002")
        .with_time_in_force(TimeInForce::Day);

    println!("  Symbol: {}", sell_order.symbol);
    println!("  Side: {:?}", sell_order.side);
    println!("  Limit Price: {:?}", sell_order.price);

    // Send order
    println!("\n--- Send Order ---");
    println!("  let cl_ord_id = client.send_order(&buy_order).await?;");
    println!("  println!(\"Limit order sent: {{}}\", cl_ord_id);");

    // New Order Single FIX fields for limit order
    println!("\n--- Limit Order FIX Fields ---");
    println!("  MsgType (35): D (New Order Single)");
    println!("  ClOrdID (11): Client order ID");
    println!("  Symbol (55): Stock symbol");
    println!("  Side (54): 1=Buy, 2=Sell");
    println!("  OrdType (40): 2=Limit");
    println!("  Price (44): Limit price");
    println!("  OrderQty (38): Order quantity");
    println!("  TimeInForce (59): Order duration");
    println!("  TransactTime (60): Transaction timestamp");

    // Limit order characteristics
    println!("\n--- Limit Order Characteristics ---");
    println!("  - Executes only at specified price or better");
    println!("  - Buy limit: executes at limit price or lower");
    println!("  - Sell limit: executes at limit price or higher");
    println!("  - May not fill if price not reached");
    println!("  - Provides price certainty");

    // Price improvement
    println!("\n--- Price Improvement ---");
    println!("  Limit orders may receive price improvement:");
    println!("  - Buy at $150.00, filled at $149.95 = $0.05 improvement");
    println!("  - Sell at $400.00, filled at $400.10 = $0.10 improvement");

    // Partial fills
    println!("\n--- Handling Partial Fills ---");
    println!("  loop {{");
    println!("      let msg = client.next_message().await?;");
    println!("      let report = client.parse_execution_report(&msg)?;");
    println!("      match report.exec_type {{");
    println!("          ExecType::PartialFill => {{");
    println!("              println!(\"Partial fill: {{}} @ {{}}\",");
    println!("                  report.last_qty.unwrap_or(0.0),");
    println!("                  report.last_px.unwrap_or(0.0));");
    println!("              println!(\"Remaining: {{}}\", report.leaves_qty);");
    println!("          }}");
    println!("          ExecType::Fill => {{");
    println!("              println!(\"Order filled completely\");");
    println!("              break;");
    println!("          }}");
    println!("          _ => {{}}");
    println!("      }}");
    println!("  }}");

    // GTC vs Day orders
    println!("\n--- GTC vs Day Orders ---");
    println!("  Day Order:");
    println!("    - Expires at end of trading day");
    println!("    - Automatically canceled if not filled");
    println!();
    println!("  GTC (Good Till Canceled):");
    println!("    - Remains active until filled or canceled");
    println!("    - May remain open for days/weeks");
    println!("    - Check broker's GTC expiration policy");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Set realistic limit prices");
    println!("2. Consider bid-ask spread");
    println!("3. Use GTC for longer-term strategies");
    println!("4. Monitor unfilled orders");
    println!("5. Cancel stale orders to free buying power");

    println!("\n=== Example Complete ===");
}
