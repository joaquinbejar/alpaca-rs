//! # Order Lifecycle Integration
//!
//! This example demonstrates the complete order lifecycle from creation
//! to execution, combining HTTP and WebSocket clients.
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
//! cargo run --example integration_order_lifecycle
//! ```
//!
//! **Note**: This example demonstrates the order lifecycle pattern but does not
//! execute real trades.

use alpaca_base::{Credentials, Environment, OrderSide, OrderStatus, TimeInForce};
use alpaca_http::{AlpacaHttpClient, CreateOrderRequest, OrderParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Order Lifecycle Integration ===\n");

    // Setup
    let credentials = Credentials::from_env()?;
    let environment = Environment::Paper;
    let http_client = AlpacaHttpClient::new(credentials, environment);
    println!("HTTP client created for {:?} environment", environment);

    // Phase 1: Pre-trade checks
    println!("\n--- Phase 1: Pre-Trade Checks ---");
    
    // Check account status
    let account = http_client.get_account().await?;
    println!("Account Status: {:?}", account.status);
    println!("Buying Power: ${}", account.buying_power);
    println!("Day Trade Count: {}", account.daytrade_count);
    
    // Check if market is open
    let clock = http_client.get_clock().await?;
    println!("Market Open: {}", clock.is_open);

    // Phase 2: Order creation (demonstration)
    println!("\n--- Phase 2: Order Creation ---");
    
    // Create a market order request
    let market_order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "1");
    println!("Market Order Request:");
    println!("  Symbol: AAPL");
    println!("  Side: Buy");
    println!("  Qty: 1");
    println!("  Type: Market");

    // Create a limit order request
    let limit_order = CreateOrderRequest::limit("MSFT", OrderSide::Buy, "1", "350.00")
        .time_in_force(TimeInForce::Gtc);
    println!("\nLimit Order Request:");
    println!("  Symbol: MSFT");
    println!("  Side: Buy");
    println!("  Qty: 1");
    println!("  Type: Limit @ $350.00");
    println!("  Time in Force: GTC");

    // Phase 3: Order submission (commented out for safety)
    println!("\n--- Phase 3: Order Submission ---");
    println!("To submit an order:");
    println!("  let order = http_client.create_order(&market_order).await?;");
    println!("  println!(\"Order ID: {{}}\", order.id);");
    println!("  println!(\"Status: {{:?}}\", order.status);");

    // Phase 4: Order monitoring
    println!("\n--- Phase 4: Order Monitoring ---");
    
    // List open orders
    let params = OrderParams::new().status(alpaca_base::OrderQueryStatus::Open);
    match http_client.get_orders(&params).await {
        Ok(orders) => {
            println!("Open Orders: {}", orders.len());
            for order in orders.iter().take(3) {
                println!(
                    "  {} {} {} @ {:?} - Status: {:?}",
                    order.symbol,
                    format!("{:?}", order.side),
                    order.qty.as_deref().unwrap_or("0"),
                    order.order_type,
                    order.status
                );
            }
        }
        Err(e) => println!("Error listing orders: {}", e),
    }

    // Phase 5: Order modification
    println!("\n--- Phase 5: Order Modification ---");
    println!("To modify an order:");
    println!("  let replace_req = ReplaceOrderRequest::new()");
    println!("      .qty(\"2\")");
    println!("      .limit_price(\"355.00\");");
    println!("  let updated = http_client.replace_order(&order_id, &replace_req).await?;");

    // Phase 6: Order cancellation
    println!("\n--- Phase 6: Order Cancellation ---");
    println!("To cancel an order:");
    println!("  http_client.cancel_order(&order_id).await?;");
    println!("To cancel all orders:");
    println!("  http_client.cancel_all_orders().await?;");

    // Phase 7: Position management
    println!("\n--- Phase 7: Position Management ---");
    match http_client.get_positions().await {
        Ok(positions) => {
            println!("Current Positions: {}", positions.len());
            for pos in positions.iter().take(3) {
                let pnl_pct = if pos.cost_basis.parse::<f64>().unwrap_or(1.0) != 0.0 {
                    let cost = pos.cost_basis.parse::<f64>().unwrap_or(1.0);
                    let pnl = pos.unrealized_pl.parse::<f64>().unwrap_or(0.0);
                    (pnl / cost) * 100.0
                } else {
                    0.0
                };
                println!(
                    "  {} - {} shares | P&L: ${} ({:.2}%)",
                    pos.symbol, pos.qty, pos.unrealized_pl, pnl_pct
                );
            }
        }
        Err(e) => println!("Error getting positions: {}", e),
    }

    // Order lifecycle summary
    println!("\n--- Order Lifecycle Summary ---");
    println!("1. PRE-TRADE: Check account, buying power, market status");
    println!("2. CREATE: Build order request with symbol, side, qty, type");
    println!("3. SUBMIT: Send order via HTTP API");
    println!("4. MONITOR: Track order status via HTTP or WebSocket");
    println!("5. MODIFY: Replace order to change qty/price if needed");
    println!("6. CANCEL: Cancel unfilled orders if strategy changes");
    println!("7. POSITION: Manage resulting positions");

    // Order status transitions
    println!("\n--- Order Status Transitions ---");
    println!("  New -> Accepted -> Filled (success)");
    println!("  New -> Accepted -> Partially Filled -> Filled");
    println!("  New -> Accepted -> Canceled (user canceled)");
    println!("  New -> Rejected (validation failed)");
    println!("  New -> Accepted -> Expired (TIF expired)");

    println!("\n=== Example Complete ===");
    Ok(())
}
