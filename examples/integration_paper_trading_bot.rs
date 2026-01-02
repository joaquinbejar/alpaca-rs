//! # Paper Trading Bot
//!
//! This example demonstrates a simple paper trading bot that monitors
//! market conditions and places orders based on simple rules.
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
//! cargo run --example integration_paper_trading_bot
//! ```
//!
//! **Note**: This example demonstrates the bot structure. Always test
//! thoroughly with paper trading before any live trading.

use alpaca_base::{Credentials, Environment, OrderSide, TimeInForce};

fn main() {
    println!("=== Paper Trading Bot ===\n");

    // Bot configuration
    println!("--- Bot Configuration ---");
    println!("  Symbol: AAPL");
    println!("  Environment: Paper");
    println!("  Strategy: Simple Moving Average Crossover");
    println!("  Position Size: 10 shares");
    println!("  Max Positions: 1");

    // Initialize client
    println!("\n--- Initialize Client ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;");

    // Check account status
    println!("\n--- Check Account Status ---");
    println!("  let account = client.get_account().await?;");
    println!("  println!(\"Buying Power: ${{}}\", account.buying_power);");
    println!("  println!(\"Portfolio Value: ${{}}\", account.portfolio_value);");
    println!("  if account.trading_blocked {{");
    println!("      panic!(\"Trading is blocked!\");");
    println!("  }}");

    // Bot state
    println!("\n--- Bot State ---");
    println!("  struct BotState {{");
    println!("      position: Option<Position>,");
    println!("      last_signal: Option<Signal>,");
    println!("      trades_today: u32,");
    println!("  }}");

    // Trading signals
    println!("\n--- Trading Signals ---");
    println!("  enum Signal {{");
    println!("      Buy,");
    println!("      Sell,");
    println!("      Hold,");
    println!("  }}");

    // Strategy implementation
    println!("\n--- Strategy: SMA Crossover ---");
    println!("  fn calculate_signal(bars: &[Bar]) -> Signal {{");
    println!("      let short_sma = calculate_sma(&bars, 10);");
    println!("      let long_sma = calculate_sma(&bars, 20);");
    println!("      ");
    println!("      if short_sma > long_sma {{");
    println!("          Signal::Buy");
    println!("      }} else if short_sma < long_sma {{");
    println!("          Signal::Sell");
    println!("      }} else {{");
    println!("          Signal::Hold");
    println!("      }}");
    println!("  }}");

    // Main trading loop
    println!("\n--- Main Trading Loop ---");
    println!("  loop {{");
    println!("      // Check if market is open");
    println!("      let clock = client.get_clock().await?;");
    println!("      if !clock.is_open {{");
    println!("          println!(\"Market closed, waiting...\");");
    println!("          tokio::time::sleep(Duration::from_secs(60)).await;");
    println!("          continue;");
    println!("      }}");
    println!("      ");
    println!("      // Get historical data");
    println!("      let bars = client.get_bars(&params).await?;");
    println!("      ");
    println!("      // Calculate signal");
    println!("      let signal = calculate_signal(&bars);");
    println!("      ");
    println!("      // Execute based on signal");
    println!("      match signal {{");
    println!("          Signal::Buy => execute_buy(&client).await?,");
    println!("          Signal::Sell => execute_sell(&client).await?,");
    println!("          Signal::Hold => {{}},");
    println!("      }}");
    println!("      ");
    println!("      // Wait before next iteration");
    println!("      tokio::time::sleep(Duration::from_secs(60)).await;");
    println!("  }}");

    // Execute buy
    println!("\n--- Execute Buy ---");
    println!("  async fn execute_buy(client: &AlpacaHttpClient) -> Result<()> {{");
    println!("      // Check if we already have a position");
    println!("      let positions = client.get_positions().await?;");
    println!("      if positions.iter().any(|p| p.symbol == \"AAPL\") {{");
    println!("          return Ok(()); // Already have position");
    println!("      }}");
    println!("      ");
    println!("      let order = CreateOrderRequest::market(\"AAPL\", 10.0, OrderSide::Buy)");
    println!("          .time_in_force(TimeInForce::Day);");
    println!("      let result = client.create_order(&order).await?;");
    println!("      println!(\"Buy order placed: {{}}\", result.id);");
    println!("      Ok(())");
    println!("  }}");

    // Execute sell
    println!("\n--- Execute Sell ---");
    println!("  async fn execute_sell(client: &AlpacaHttpClient) -> Result<()> {{");
    println!("      // Check if we have a position to sell");
    println!("      let positions = client.get_positions().await?;");
    println!("      if let Some(pos) = positions.iter().find(|p| p.symbol == \"AAPL\") {{");
    println!("          let qty = pos.qty.parse::<f64>().unwrap_or(0.0);");
    println!("          let order = CreateOrderRequest::market(\"AAPL\", qty, OrderSide::Sell)");
    println!("              .time_in_force(TimeInForce::Day);");
    println!("          let result = client.create_order(&order).await?;");
    println!("          println!(\"Sell order placed: {{}}\", result.id);");
    println!("      }}");
    println!("      Ok(())");
    println!("  }}");

    // Risk management
    println!("\n--- Risk Management ---");
    println!("1. Position sizing: Fixed 10 shares");
    println!("2. Max positions: 1 at a time");
    println!("3. Stop loss: Not implemented (add for production)");
    println!("4. Daily loss limit: Not implemented (add for production)");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Always test with paper trading first");
    println!("2. Implement proper error handling");
    println!("3. Add logging for all trades");
    println!("4. Monitor bot performance regularly");
    println!("5. Implement circuit breakers for losses");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;
    let _side = OrderSide::Buy;
    let _tif = TimeInForce::Day;

    println!("\n=== Example Complete ===");
}
