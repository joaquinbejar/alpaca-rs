//! # FIX Market Order
//!
//! This example demonstrates how to send a market order via FIX protocol
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
//! cargo run -p alpaca-fix --example fix_market_order
//! ```
//!
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

use alpaca_fix::messages::{NewOrderSingle, Side, TimeInForce};

fn main() {
    println!("=== FIX Market Order ===\n");

    // Create market buy order
    println!("--- Create Market Buy Order ---");
    let buy_order = NewOrderSingle::market("AAPL", Side::Buy, 100.0)
        .with_cl_ord_id("order-001")
        .with_time_in_force(TimeInForce::Day);

    println!("  Symbol: {}", buy_order.symbol);
    println!("  Side: {:?}", buy_order.side);
    println!("  Order Type: {:?}", buy_order.ord_type);
    println!("  Quantity: {}", buy_order.order_qty);
    println!("  Time in Force: {:?}", buy_order.time_in_force);
    println!("  Client Order ID: {}", buy_order.cl_ord_id);

    // Create market sell order
    println!("\n--- Create Market Sell Order ---");
    let sell_order = NewOrderSingle::market("MSFT", Side::Sell, 50.0)
        .with_cl_ord_id("order-002")
        .with_time_in_force(TimeInForce::Day);

    println!("  Symbol: {}", sell_order.symbol);
    println!("  Side: {:?}", sell_order.side);
    println!("  Quantity: {}", sell_order.order_qty);

    // Send order
    println!("\n--- Send Order ---");
    println!("  // Ensure session is active");
    println!("  assert_eq!(client.state().await, SessionState::Active);");
    println!();
    println!("  // Send the order");
    println!("  let cl_ord_id = client.send_order(&buy_order).await?;");
    println!("  println!(\"Order sent: {{}}\", cl_ord_id);");

    // New Order Single FIX fields
    println!("\n--- New Order Single FIX Fields ---");
    println!("  MsgType (35): D (New Order Single)");
    println!("  ClOrdID (11): Client order ID");
    println!("  Symbol (55): Stock symbol");
    println!("  Side (54): 1=Buy, 2=Sell");
    println!("  OrdType (40): 1=Market");
    println!("  OrderQty (38): Order quantity");
    println!("  TimeInForce (59): 0=Day, 1=GTC, 3=IOC, 4=FOK");
    println!("  TransactTime (60): Transaction timestamp");

    // Time in Force options
    println!("\n--- Time in Force Options ---");
    println!("  Day: Valid for trading day only");
    println!("  GTC: Good Till Canceled");
    println!("  IOC: Immediate Or Cancel");
    println!("  FOK: Fill Or Kill");
    println!("  OPG: At the Opening");
    println!("  CLS: At the Close");

    // Market order characteristics
    println!("\n--- Market Order Characteristics ---");
    println!("  - Executes immediately at best available price");
    println!("  - No price guarantee");
    println!("  - High fill probability");
    println!("  - May experience slippage in volatile markets");
    println!("  - Best for liquid securities");

    // Wait for execution report
    println!("\n--- Wait for Execution Report ---");
    println!("  loop {{");
    println!("      let msg = client.next_message().await?;");
    println!("      if let Some(msg_type) = msg.msg_type() {{");
    println!("          if msg_type == \"8\" {{ // ExecutionReport");
    println!("              let report = client.parse_execution_report(&msg)?;");
    println!("              println!(\"Status: {{:?}}\", report.ord_status);");
    println!("              if report.ord_status == OrdStatus::Filled {{");
    println!("                  println!(\"Filled at: {{}}\", report.avg_px);");
    println!("                  break;");
    println!("              }}");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Use unique client order IDs");
    println!("2. Track order state via execution reports");
    println!("3. Handle partial fills");
    println!("4. Implement timeout for order confirmation");
    println!("5. Log all order activity");

    println!("\n=== Example Complete ===");
}
