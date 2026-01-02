//! # Realtime Order Tracker
//!
//! This example demonstrates how to place an order and track its status
//! in real-time using WebSocket updates.
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
//! cargo run --example integration_realtime_order_tracker
//! ```

use alpaca_base::{Credentials, Environment, OrderSide, TimeInForce};

fn main() {
    println!("=== Realtime Order Tracker ===\n");

    // Initialize clients
    println!("--- Initialize Clients ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let http_client = AlpacaHttpClient::new(credentials.clone(), Environment::Paper)?;");
    println!("  let ws_client = AlpacaWebSocketClient::from_env(Environment::Paper)?;");

    // Connect to WebSocket for trade updates
    println!("\n--- Connect to Trade Updates ---");
    println!("  let mut stream = ws_client.subscribe_trade_updates().await?;");
    println!("  println!(\"Connected to trade updates stream\");");

    // Place order via HTTP
    println!("\n--- Place Order via HTTP ---");
    println!("  let order = CreateOrderRequest::limit(\"AAPL\", 10.0, OrderSide::Buy, 150.00)");
    println!("      .time_in_force(TimeInForce::Day)");
    println!("      .client_order_id(\"tracker-001\");");
    println!("  ");
    println!("  let placed_order = http_client.create_order(&order).await?;");
    println!("  println!(\"Order placed: {{}}\", placed_order.id);");
    println!("  println!(\"Status: {{:?}}\", placed_order.status);");

    // Track order via WebSocket
    println!("\n--- Track Order via WebSocket ---");
    println!("  let order_id = placed_order.id.clone();");
    println!("  ");
    println!("  while let Some(update) = stream.next().await {{");
    println!("      match update {{");
    println!("          Ok(WebSocketMessage::TradeUpdate(trade_update)) => {{");
    println!("              if trade_update.order.id == order_id {{");
    println!("                  println!(\"Order Update:\");");
    println!("                  println!(\"  Event: {{:?}}\", trade_update.event);");
    println!("                  println!(\"  Status: {{:?}}\", trade_update.order.status);");
    println!("                  println!(\"  Filled Qty: {{}}\", trade_update.order.filled_qty);");
    println!("                  ");
    println!("                  // Check if order is terminal");
    println!("                  match trade_update.event {{");
    println!("                      TradeUpdateEvent::Fill => {{");
    println!("                          println!(\"Order filled completely!\");");
    println!("                          break;");
    println!("                      }}");
    println!("                      TradeUpdateEvent::Canceled |");
    println!("                      TradeUpdateEvent::Rejected |");
    println!("                      TradeUpdateEvent::Expired => {{");
    println!("                          println!(\"Order terminated: {{:?}}\", trade_update.event);");
    println!("                          break;");
    println!("                      }}");
    println!("                      _ => {{}}");
    println!("                  }}");
    println!("              }}");
    println!("          }}");
    println!("          Err(e) => eprintln!(\"Error: {{}}\", e),");
    println!("          _ => {{}}");
    println!("      }}");
    println!("  }}");

    // Trade update events
    println!("\n--- Trade Update Events ---");
    println!("  New: Order accepted");
    println!("  Fill: Order filled (partial or complete)");
    println!("  PartialFill: Partial fill occurred");
    println!("  Canceled: Order canceled");
    println!("  Expired: Order expired");
    println!("  Rejected: Order rejected");
    println!("  PendingNew: Order pending acceptance");
    println!("  PendingCancel: Cancel pending");
    println!("  PendingReplace: Replace pending");
    println!("  Replaced: Order replaced");

    // Order state machine
    println!("\n--- Order State Machine ---");
    println!("  New -> PartialFill -> Fill (success)");
    println!("  New -> Fill (immediate fill)");
    println!("  New -> Canceled (user canceled)");
    println!("  New -> Expired (time expired)");
    println!("  New -> Rejected (validation failed)");

    // Timeout handling
    println!("\n--- Timeout Handling ---");
    println!("  let timeout = Duration::from_secs(60);");
    println!("  ");
    println!("  tokio::select! {{");
    println!("      result = track_order(&mut stream, &order_id) => {{");
    println!("          println!(\"Order tracking complete: {{:?}}\", result);");
    println!("      }}");
    println!("      _ = tokio::time::sleep(timeout) => {{");
    println!("          println!(\"Timeout waiting for order update\");");
    println!("          // Optionally cancel the order");
    println!("          http_client.cancel_order(&order_id).await?;");
    println!("      }}");
    println!("  }}");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Always connect WebSocket before placing order");
    println!("2. Handle all terminal states (fill, cancel, reject, expire)");
    println!("3. Implement timeout for order tracking");
    println!("4. Store order state for recovery");
    println!("5. Log all order events for audit");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;
    let _side = OrderSide::Buy;
    let _tif = TimeInForce::Day;

    println!("\n=== Example Complete ===");
}
