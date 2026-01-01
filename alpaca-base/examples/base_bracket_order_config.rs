//! # Bracket Order Configuration
//!
//! This example demonstrates how to configure bracket orders with
//! take-profit and stop-loss legs.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_bracket_order_config
//! ```

use alpaca_base::{OrderClass, StopLoss, TakeProfit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Bracket Order Configuration ===\n");

    // 1. Take Profit Configuration
    println!("--- Take Profit ---");
    demonstrate_take_profit()?;

    // 2. Stop Loss Configuration
    println!("\n--- Stop Loss ---");
    demonstrate_stop_loss()?;

    // 3. Complete Bracket Order
    println!("\n--- Complete Bracket Order Example ---");
    demonstrate_bracket_order()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_take_profit() -> Result<(), Box<dyn std::error::Error>> {
    // Simple take profit at a limit price
    let tp = TakeProfit::new("155.00");
    println!("  Take Profit: sell at ${}", tp.limit_price);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&tp)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}

fn demonstrate_stop_loss() -> Result<(), Box<dyn std::error::Error>> {
    // Stop loss with market order (stop price only)
    let sl_market = StopLoss::new("145.00");
    println!("  Stop Loss (Market): trigger at ${}", sl_market.stop_price);
    println!("    When triggered: executes as market order");

    // Stop loss with limit order (stop price + limit price)
    let sl_limit = StopLoss::with_limit("145.00", "144.50");
    println!(
        "\n  Stop Loss (Limit): trigger at ${}, limit ${}",
        sl_limit.stop_price,
        sl_limit.limit_price.as_ref().unwrap()
    );
    println!("    When triggered: executes as limit order at $144.50");

    // Serialize to JSON
    println!("\n  JSON (with limit):");
    let json = serde_json::to_string_pretty(&sl_limit)?;
    println!("{}", json);

    Ok(())
}

fn demonstrate_bracket_order() -> Result<(), Box<dyn std::error::Error>> {
    // Simulating a bracket order structure
    println!("  Scenario: Buy 100 shares of AAPL at $150.00");
    println!();

    let entry_price = 150.00;
    let take_profit_price = entry_price * 1.05; // 5% profit target
    let stop_loss_price = entry_price * 0.97; // 3% stop loss

    let tp = TakeProfit::new(format!("{:.2}", take_profit_price));
    let sl = StopLoss::new(format!("{:.2}", stop_loss_price));

    println!("  Order Class: {:?}", OrderClass::Bracket);
    println!("  Entry: Buy 100 AAPL @ ${:.2}", entry_price);
    println!("  Take Profit: Sell @ ${} (+5%)", tp.limit_price);
    println!("  Stop Loss: Sell @ ${} (-3%)", sl.stop_price);

    // Risk/Reward calculation
    let risk = entry_price - stop_loss_price;
    let reward = take_profit_price - entry_price;
    let rr_ratio = reward / risk;

    println!("\n  Risk/Reward Analysis:");
    println!("    Risk per share: ${:.2}", risk);
    println!("    Reward per share: ${:.2}", reward);
    println!("    R:R Ratio: {:.2}:1", rr_ratio);

    // Complete bracket order JSON structure
    println!("\n  Complete Order JSON:");
    let order = serde_json::json!({
        "symbol": "AAPL",
        "qty": "100",
        "side": "buy",
        "type": "limit",
        "limit_price": format!("{:.2}", entry_price),
        "time_in_force": "day",
        "order_class": "bracket",
        "take_profit": {
            "limit_price": tp.limit_price
        },
        "stop_loss": {
            "stop_price": sl.stop_price
        }
    });
    println!("{}", serde_json::to_string_pretty(&order)?);

    Ok(())
}
