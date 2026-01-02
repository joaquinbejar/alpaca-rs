//! # Bracket Order with Updates
//!
//! This example demonstrates how to place a bracket order and monitor
//! all legs via WebSocket updates.
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
//! cargo run --example integration_bracket_order_with_updates
//! ```

use alpaca_base::{Credentials, Environment, OrderSide, TimeInForce};

fn main() {
    println!("=== Bracket Order with Updates ===\n");

    // What is a bracket order?
    println!("--- What is a Bracket Order? ---");
    println!("  A bracket order consists of three linked orders:");
    println!("  1. Entry order (market or limit)");
    println!("  2. Take profit order (limit sell above entry)");
    println!("  3. Stop loss order (stop sell below entry)");
    println!();
    println!("  When entry fills, both exit orders become active.");
    println!("  When one exit fills, the other is canceled (OCO).");

    // Initialize clients
    println!("\n--- Initialize Clients ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let http_client = AlpacaHttpClient::new(credentials.clone(), Environment::Paper)?;");
    println!("  let ws_client = AlpacaWebSocketClient::from_env(Environment::Paper)?;");
    println!("  let mut stream = ws_client.subscribe_trade_updates().await?;");

    // Create bracket order
    println!("\n--- Create Bracket Order ---");
    println!("  let bracket = CreateOrderRequest::market(\"AAPL\", 10.0, OrderSide::Buy)");
    println!("      .time_in_force(TimeInForce::Day)");
    println!("      .take_profit(TakeProfitParams {{");
    println!("          limit_price: 160.00,  // Sell when price reaches $160");
    println!("      }})");
    println!("      .stop_loss(StopLossParams {{");
    println!("          stop_price: 145.00,   // Sell when price drops to $145");
    println!("          limit_price: None,    // Market order at stop");
    println!("      }});");
    println!("  ");
    println!("  let order = http_client.create_order(&bracket).await?;");
    println!("  println!(\"Bracket order placed: {{}}\", order.id);");

    // Track order legs
    println!("\n--- Track Order Legs ---");
    println!("  struct BracketState {{");
    println!("      entry_id: String,");
    println!("      take_profit_id: Option<String>,");
    println!("      stop_loss_id: Option<String>,");
    println!("      entry_filled: bool,");
    println!("      exit_filled: bool,");
    println!("  }}");
    println!("  ");
    println!("  let mut state = BracketState {{");
    println!("      entry_id: order.id.clone(),");
    println!("      take_profit_id: order.legs.get(0).map(|l| l.id.clone()),");
    println!("      stop_loss_id: order.legs.get(1).map(|l| l.id.clone()),");
    println!("      entry_filled: false,");
    println!("      exit_filled: false,");
    println!("  }};");

    // Monitor updates
    println!("\n--- Monitor Updates ---");
    println!("  while let Some(update) = stream.next().await {{");
    println!("      if let Ok(WebSocketMessage::TradeUpdate(tu)) = update {{");
    println!("          let order_id = &tu.order.id;");
    println!("          ");
    println!("          if order_id == &state.entry_id {{");
    println!("              match tu.event {{");
    println!("                  TradeUpdateEvent::Fill => {{");
    println!("                      println!(\"Entry filled at ${{}}\", tu.order.filled_avg_price);");
    println!("                      state.entry_filled = true;");
    println!("                  }}");
    println!("                  TradeUpdateEvent::Rejected => {{");
    println!("                      println!(\"Entry rejected!\");");
    println!("                      break;");
    println!("                  }}");
    println!("                  _ => {{}}");
    println!("              }}");
    println!("          }} else if Some(order_id) == state.take_profit_id.as_ref() {{");
    println!("              if tu.event == TradeUpdateEvent::Fill {{");
    println!("                  println!(\"Take profit hit! Sold at ${{}}\", tu.order.filled_avg_price);");
    println!("                  state.exit_filled = true;");
    println!("                  break;");
    println!("              }}");
    println!("          }} else if Some(order_id) == state.stop_loss_id.as_ref() {{");
    println!("              if tu.event == TradeUpdateEvent::Fill {{");
    println!("                  println!(\"Stop loss triggered! Sold at ${{}}\", tu.order.filled_avg_price);");
    println!("                  state.exit_filled = true;");
    println!("                  break;");
    println!("              }}");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Calculate P&L
    println!("\n--- Calculate P&L ---");
    println!("  fn calculate_pnl(entry_price: f64, exit_price: f64, qty: f64, side: OrderSide) -> f64 {{");
    println!("      match side {{");
    println!("          OrderSide::Buy => (exit_price - entry_price) * qty,");
    println!("          OrderSide::Sell => (entry_price - exit_price) * qty,");
    println!("      }}");
    println!("  }}");
    println!("  ");
    println!("  let pnl = calculate_pnl(entry_price, exit_price, 10.0, OrderSide::Buy);");
    println!("  println!(\"P&L: ${:.2}\", pnl);");

    // Bracket order scenarios
    println!("\n--- Bracket Order Scenarios ---");
    println!("  Scenario 1: Price goes up");
    println!("    Entry fills at $150");
    println!("    Price rises to $160");
    println!("    Take profit fills -> Stop loss canceled");
    println!("    P&L: +$100 (10 shares × $10)");
    println!();
    println!("  Scenario 2: Price goes down");
    println!("    Entry fills at $150");
    println!("    Price drops to $145");
    println!("    Stop loss fills -> Take profit canceled");
    println!("    P&L: -$50 (10 shares × $5)");

    // Modify bracket legs
    println!("\n--- Modify Bracket Legs ---");
    println!("  // Adjust take profit");
    println!("  let replace = ReplaceOrderRequest {{");
    println!("      limit_price: Some(165.00),  // New take profit");
    println!("      ..Default::default()");
    println!("  }};");
    println!("  http_client.replace_order(&take_profit_id, &replace).await?;");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Set realistic take profit and stop loss levels");
    println!("2. Consider volatility when setting stop distance");
    println!("3. Monitor all three legs for updates");
    println!("4. Handle partial fills appropriately");
    println!("5. Log all bracket order events");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;
    let _side = OrderSide::Buy;
    let _tif = TimeInForce::Day;

    println!("\n=== Example Complete ===");
}
