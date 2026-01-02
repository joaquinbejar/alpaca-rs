//! # Portfolio Rebalancer
//!
//! This example demonstrates how to rebalance a portfolio to target
//! allocations using the Alpaca HTTP API.
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
//! cargo run --example integration_portfolio_rebalancer
//! ```

use alpaca_base::{Credentials, Environment, OrderSide, TimeInForce};

fn main() {
    println!("=== Portfolio Rebalancer ===\n");

    // Target allocation
    println!("--- Target Allocation ---");
    println!("  AAPL: 30%");
    println!("  MSFT: 25%");
    println!("  GOOGL: 20%");
    println!("  SPY: 15%");
    println!("  Cash: 10%");

    // Initialize client
    println!("\n--- Initialize Client ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;");

    // Get current portfolio
    println!("\n--- Get Current Portfolio ---");
    println!("  let account = client.get_account().await?;");
    println!("  let portfolio_value = account.portfolio_value.parse::<f64>()?;");
    println!("  println!(\"Portfolio Value: ${:.2}\", portfolio_value);");
    println!("  ");
    println!("  let positions = client.get_positions().await?;");
    println!("  for pos in &positions {{");
    println!("      let value = pos.market_value.parse::<f64>()?;");
    println!("      let pct = value / portfolio_value * 100.0;");
    println!("      println!(\"  {{}}: ${:.2} ({:.1}%)\", pos.symbol, value, pct);");
    println!("  }}");

    // Calculate current vs target
    println!("\n--- Calculate Rebalance ---");
    println!("  struct Allocation {{");
    println!("      symbol: String,");
    println!("      target_pct: f64,");
    println!("      current_pct: f64,");
    println!("      current_value: f64,");
    println!("      target_value: f64,");
    println!("      diff_value: f64,");
    println!("  }}");
    println!("  ");
    println!("  fn calculate_rebalance(");
    println!("      positions: &[Position],");
    println!("      targets: &HashMap<String, f64>,");
    println!("      portfolio_value: f64,");
    println!("  ) -> Vec<Allocation> {{");
    println!("      targets.iter().map(|(symbol, target_pct)| {{");
    println!("          let current = positions.iter()");
    println!("              .find(|p| &p.symbol == symbol)");
    println!("              .map(|p| p.market_value.parse::<f64>().unwrap_or(0.0))");
    println!("              .unwrap_or(0.0);");
    println!("          ");
    println!("          let target_value = portfolio_value * target_pct / 100.0;");
    println!("          Allocation {{");
    println!("              symbol: symbol.clone(),");
    println!("              target_pct: *target_pct,");
    println!("              current_pct: current / portfolio_value * 100.0,");
    println!("              current_value: current,");
    println!("              target_value,");
    println!("              diff_value: target_value - current,");
    println!("          }}");
    println!("      }}).collect()");
    println!("  }}");

    // Generate orders
    println!("\n--- Generate Rebalance Orders ---");
    println!("  fn generate_orders(");
    println!("      allocations: &[Allocation],");
    println!("      prices: &HashMap<String, f64>,");
    println!("      min_trade_value: f64,");
    println!("  ) -> Vec<CreateOrderRequest> {{");
    println!("      allocations.iter()");
    println!("          .filter(|a| a.diff_value.abs() > min_trade_value)");
    println!("          .map(|a| {{");
    println!("              let price = prices.get(&a.symbol).unwrap_or(&0.0);");
    println!("              let qty = (a.diff_value.abs() / price).floor();");
    println!("              let side = if a.diff_value > 0.0 {{");
    println!("                  OrderSide::Buy");
    println!("              }} else {{");
    println!("                  OrderSide::Sell");
    println!("              }};");
    println!("              CreateOrderRequest::market(&a.symbol, qty, side)");
    println!("                  .time_in_force(TimeInForce::Day)");
    println!("          }})");
    println!("          .collect()");
    println!("  }}");

    // Execute rebalance
    println!("\n--- Execute Rebalance ---");
    println!("  // Sell orders first to free up cash");
    println!("  let sell_orders: Vec<_> = orders.iter()");
    println!("      .filter(|o| o.side == OrderSide::Sell)");
    println!("      .collect();");
    println!("  ");
    println!("  for order in sell_orders {{");
    println!("      let result = client.create_order(order).await?;");
    println!("      println!(\"Sell order: {{}} {{}}\", result.symbol, result.qty);");
    println!("  }}");
    println!("  ");
    println!("  // Wait for sells to settle");
    println!("  tokio::time::sleep(Duration::from_secs(2)).await;");
    println!("  ");
    println!("  // Then buy orders");
    println!("  let buy_orders: Vec<_> = orders.iter()");
    println!("      .filter(|o| o.side == OrderSide::Buy)");
    println!("      .collect();");
    println!("  ");
    println!("  for order in buy_orders {{");
    println!("      let result = client.create_order(order).await?;");
    println!("      println!(\"Buy order: {{}} {{}}\", result.symbol, result.qty);");
    println!("  }}");

    // Rebalance thresholds
    println!("\n--- Rebalance Thresholds ---");
    println!("  const MIN_TRADE_VALUE: f64 = 100.0;  // Minimum $100 trade");
    println!("  const DRIFT_THRESHOLD: f64 = 5.0;    // Rebalance if >5% drift");
    println!("  ");
    println!("  fn needs_rebalance(allocations: &[Allocation]) -> bool {{");
    println!("      allocations.iter().any(|a| {{");
    println!("          (a.current_pct - a.target_pct).abs() > DRIFT_THRESHOLD");
    println!("      }})");
    println!("  }}");

    // Tax-efficient rebalancing
    println!("\n--- Tax-Efficient Rebalancing ---");
    println!("  1. Use new contributions to buy underweight assets");
    println!("  2. Reinvest dividends into underweight assets");
    println!("  3. Sell overweight assets with losses first (tax-loss harvesting)");
    println!("  4. Consider holding period for long-term gains");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Set minimum trade size to avoid small orders");
    println!("2. Execute sells before buys");
    println!("3. Use limit orders for better execution");
    println!("4. Consider transaction costs");
    println!("5. Rebalance periodically (quarterly/annually)");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;
    let _side = OrderSide::Buy;
    let _tif = TimeInForce::Day;

    println!("\n=== Example Complete ===");
}
