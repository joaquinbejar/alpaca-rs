//! # Order Types Example
//!
//! This example demonstrates how to configure different order types using
//! the core enums in `alpaca-base`.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example base_order_types
//! ```

use alpaca_base::{OrderType, OrderSide, TimeInForce, OrderClass};

fn main() {
    println!("=== Alpaca Order Configuration ===\n");

    // 1. Basic Market Buy Order
    let side = OrderSide::Buy;
    let order_type = OrderType::Market;
    let tif = TimeInForce::Gtc; // Good 'Til Cancelled
    
    println!("--- Market Buy Order (GTC) ---");
    println!("  Side: {:?}", side);
    println!("  Type: {:?}", order_type);
    println!("  Time In Force: {:?}", tif);

    // 2. Limit Sell Order
    let side = OrderSide::Sell;
    let order_type = OrderType::Limit;
    let tif = TimeInForce::Day;
    
    println!("\n--- Limit Sell Order (Day) ---");
    println!("  Side: {:?}", side);
    println!("  Type: {:?}", order_type);
    println!("  Time In Force: {:?}", tif);

    // 3. Advanced Order Classes (Bracket, OCO, OTO)
    println!("\n--- Order Classes ---");
    
    let simple = OrderClass::Simple;
    println!("  Simple: {:?}", simple);
    
    let bracket = OrderClass::Bracket;
    println!("  Bracket: {:?} (Entry + Take Profit + Stop Loss)", bracket);
    
    let oco = OrderClass::Oco;
    println!("  OCO: {:?} (One-Cancels-Other)", oco);

    println!("\n=== Example Complete ===");
}
