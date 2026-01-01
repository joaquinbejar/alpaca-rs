//! # Order Types Configuration
//!
//! This example demonstrates how to work with different order type
//! configurations using the alpaca-base types.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_order_types
//! ```
//!
//! ## Expected Output
//!
//! Demonstrates OrderType, OrderSide, TimeInForce, and OrderClass enums.

use alpaca_base::{OrderClass, OrderSide, OrderType, TimeInForce};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Alpaca Order Types ===\n");

    // 1. Order Types
    println!("--- Order Types ---");
    demonstrate_order_types();

    // 2. Order Sides
    println!("\n--- Order Sides ---");
    demonstrate_order_sides();

    // 3. Time in Force
    println!("\n--- Time in Force ---");
    demonstrate_time_in_force();

    // 4. Order Classes
    println!("\n--- Order Classes ---");
    demonstrate_order_classes();

    // 5. JSON Serialization
    println!("\n--- JSON Serialization ---");
    demonstrate_serialization()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_order_types() {
    let order_types = [
        (OrderType::Market, "Execute immediately at best available price"),
        (OrderType::Limit, "Execute at specified price or better"),
        (OrderType::Stop, "Trigger market order when stop price is reached"),
        (OrderType::StopLimit, "Trigger limit order when stop price is reached"),
        (OrderType::TrailingStop, "Stop price trails the market price"),
    ];

    for (order_type, description) in order_types {
        println!("  {:?}: {}", order_type, description);
    }

    // Default order type
    let default_type: OrderType = Default::default();
    println!("\n  Default order type: {:?}", default_type);
}

fn demonstrate_order_sides() {
    let sides = [
        (OrderSide::Buy, "Purchase shares/contracts"),
        (OrderSide::Sell, "Sell shares/contracts"),
    ];

    for (side, description) in sides {
        println!("  {:?}: {}", side, description);
    }

    // Default side
    let default_side: OrderSide = Default::default();
    println!("\n  Default order side: {:?}", default_side);
}

fn demonstrate_time_in_force() {
    let tif_options = [
        (TimeInForce::Day, "Valid for the trading day only"),
        (TimeInForce::Gtc, "Good 'til canceled - remains active until filled or canceled"),
        (TimeInForce::Opg, "Opening - execute at market open"),
        (TimeInForce::Cls, "Closing - execute at market close"),
        (TimeInForce::Ioc, "Immediate or cancel - fill immediately or cancel"),
        (TimeInForce::Fok, "Fill or kill - fill entirely or cancel"),
        (TimeInForce::Gtd, "Good 'til date - remains active until specified date"),
    ];

    for (tif, description) in tif_options {
        println!("  {:?}: {}", tif, description);
    }

    // Default TIF
    let default_tif: TimeInForce = Default::default();
    println!("\n  Default time in force: {:?}", default_tif);
}

fn demonstrate_order_classes() {
    let order_classes = [
        (OrderClass::Simple, "Standard single order"),
        (OrderClass::Bracket, "Entry order with take-profit and stop-loss"),
        (OrderClass::Oco, "One-Cancels-Other - two orders, one cancels the other"),
        (OrderClass::Oto, "One-Triggers-Other - primary order triggers secondary"),
    ];

    for (order_class, description) in order_classes {
        println!("  {:?}: {}", order_class, description);
    }

    // Example: Bracket order structure
    println!("\n  Bracket Order Structure:");
    println!("    Entry: Buy 100 AAPL at market");
    println!("    Take Profit: Sell at $160 (limit)");
    println!("    Stop Loss: Sell at $140 (stop)");
}

fn demonstrate_serialization() -> Result<(), Box<dyn std::error::Error>> {
    // Serialize to JSON (as used in API requests)
    let order_type = OrderType::StopLimit;
    let json = serde_json::to_string(&order_type)?;
    println!("  OrderType::StopLimit -> {}", json);

    let side = OrderSide::Buy;
    let json = serde_json::to_string(&side)?;
    println!("  OrderSide::Buy -> {}", json);

    let tif = TimeInForce::Gtc;
    let json = serde_json::to_string(&tif)?;
    println!("  TimeInForce::Gtc -> {}", json);

    let order_class = OrderClass::Bracket;
    let json = serde_json::to_string(&order_class)?;
    println!("  OrderClass::Bracket -> {}", json);

    // Deserialize from JSON (as received from API)
    println!("\n  Deserialization:");
    let parsed: OrderType = serde_json::from_str("\"trailing_stop\"")?;
    println!("    \"trailing_stop\" -> {:?}", parsed);

    let parsed: TimeInForce = serde_json::from_str("\"ioc\"")?;
    println!("    \"ioc\" -> {:?}", parsed);

    Ok(())
}
