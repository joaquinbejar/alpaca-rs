//! # Test Assertions
//!
//! This example demonstrates how to use assertion helpers for testing.
//! Requires the `test-utils` feature.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_test_assertions --features test-utils
//! ```

#[cfg(feature = "test-utils")]
use alpaca_base::test_utils::{assertions, fixtures};
#[cfg(feature = "test-utils")]
use alpaca_base::{OrderSide, OrderType, PositionSide};

fn main() {
    println!("=== Test Assertions ===\n");

    #[cfg(feature = "test-utils")]
    {
        // 1. Order Assertions
        println!("--- Order Assertions ---");
        let order = fixtures::sample_order("AAPL", OrderSide::Buy, "100");

        // This would panic if assertions fail
        assertions::assert_order_basics(&order, "AAPL", OrderSide::Buy, OrderType::Market);
        println!("  ✓ assert_order_basics passed");
        println!("    Symbol: {} == AAPL", order.symbol);
        println!("    Side: {:?} == Buy", order.side);
        println!("    Type: {:?} == Market", order.order_type);

        // 2. Position Assertions
        println!("\n--- Position Assertions ---");
        let position = fixtures::sample_position("MSFT", "50", "300.00");

        assertions::assert_position_basics(&position, "MSFT", PositionSide::Long);
        println!("  ✓ assert_position_basics passed");
        println!("    Symbol: {} == MSFT", position.symbol);
        println!("    Side: {:?} == Long", position.side);

        // 3. Account Assertions
        println!("\n--- Account Assertions ---");
        let account = fixtures::sample_account();

        assertions::assert_account_active(&account);
        println!("  ✓ assert_account_active passed");
        println!("    Status: {:?} == Active", account.status);
        println!("    Trading Blocked: {} == false", account.trading_blocked);
        println!("    Account Blocked: {} == false", account.account_blocked);

        // 4. Custom Assertions Example
        println!("\n--- Custom Assertion Patterns ---");
        demonstrate_custom_assertions();
    }

    #[cfg(not(feature = "test-utils"))]
    {
        println!("  This example requires the 'test-utils' feature.");
        println!(
            "  Run with: cargo run -p alpaca-base --example base_test_assertions --features test-utils"
        );
    }

    println!("\n=== Example Complete ===");
}

#[cfg(feature = "test-utils")]
fn demonstrate_custom_assertions() {
    println!("  Example custom assertion patterns:");
    println!();
    println!("    // Assert order is filled");
    println!("    fn assert_order_filled(order: &Order) {{");
    println!("        assert_eq!(order.status, OrderStatus::Filled);");
    println!("        assert!(order.filled_at.is_some());");
    println!("        assert!(order.filled_avg_price.is_some());");
    println!("    }}");
    println!();
    println!("    // Assert position is profitable");
    println!("    fn assert_position_profitable(position: &Position) {{");
    println!("        let pl: f64 = position.unrealized_pl.parse().unwrap();");
    println!("        assert!(pl > 0.0, \"Position should be profitable\");");
    println!("    }}");
    println!();
    println!("    // Assert account has buying power");
    println!("    fn assert_has_buying_power(account: &Account, min: f64) {{");
    println!("        let bp: f64 = account.buying_power.parse().unwrap();");
    println!("        assert!(bp >= min, \"Insufficient buying power\");");
    println!("    }}");
}
