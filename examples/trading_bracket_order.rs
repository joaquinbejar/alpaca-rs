//! Example: Creating Bracket Orders with Alpaca API
//!
//! This example demonstrates how to create bracket orders using the alpaca-http crate.
//! A bracket order consists of a primary order with attached take-profit and stop-loss legs.
//!
//! ## Required Environment Variables
//!
//! Create a `.env` file in the project root with:
//! ```env
//! ALPACA_API_KEY=your_api_key
//! ALPACA_API_SECRET=your_api_secret
//! ALPACA_BASE_URL=https://paper-api.alpaca.markets
//! ```
//!
//! ## Running the Example
//!
//! ```bash
//! cargo run --example trading_bracket_order
//! ```

use alpaca_http::{AlpacaHttpClient, CreateOrderRequest, OrderParams};
use alpaca_base::{OrderSide, OrderType, OrderQueryStatus, TakeProfit, StopLoss, Environment, Credentials};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let api_key = env::var("ALPACA_API_KEY")
        .expect("ALPACA_API_KEY must be set in .env file");
    let api_secret = env::var("ALPACA_API_SECRET")
        .expect("ALPACA_API_SECRET must be set in .env file");
    let base_url = env::var("ALPACA_BASE_URL")
        .unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string());

    println!("Connecting to Alpaca API at: {}", base_url);

    // Determine environment from URL
    let environment = if base_url.contains("paper") {
        Environment::Paper
    } else {
        Environment::Live
    };

    // Create credentials and HTTP client
    let credentials = Credentials::new(api_key, api_secret);
    let client = AlpacaHttpClient::new(credentials, environment)?;

    // Get account information
    let account = client.get_account().await?;
    println!("Account ID: {}", account.id);
    println!("Buying Power: ${}", account.buying_power);
    println!("Portfolio Value: ${}", account.portfolio_value);

    // Example 1: Create a bracket order
    // This creates a buy order with automatic take-profit and stop-loss legs
    println!("\n--- Creating Bracket Order ---");
    
    let take_profit = TakeProfit::new("160.00");
    let stop_loss = StopLoss::with_limit("140.00", "139.50");
    
    let bracket_order = CreateOrderRequest::bracket(
        "AAPL",
        OrderSide::Buy,
        "1",
        OrderType::Limit,
        take_profit,
        stop_loss,
    )
    .with_limit_price("150.00")
    .client_order_id("bracket-example-001");

    println!("Bracket order request: {:?}", bracket_order);
    
    // Uncomment to actually submit the order:
    // let order = client.create_order(&bracket_order).await?;
    // println!("Created bracket order: {:?}", order);

    // Example 2: Create different order types
    println!("\n--- Order Type Examples ---");

    // Market order
    let market_order = CreateOrderRequest::market("AAPL", OrderSide::Buy, "1")
        .client_order_id("market-example-001");
    println!("Market order: {:?}", market_order);

    // Limit order
    let limit_order = CreateOrderRequest::limit("AAPL", OrderSide::Buy, "1", "150.00")
        .extended_hours(true)
        .client_order_id("limit-example-001");
    println!("Limit order: {:?}", limit_order);

    // Stop order
    let stop_order = CreateOrderRequest::stop("AAPL", OrderSide::Sell, "1", "145.00")
        .client_order_id("stop-example-001");
    println!("Stop order: {:?}", stop_order);

    // Stop-limit order
    let stop_limit_order = CreateOrderRequest::stop_limit(
        "AAPL",
        OrderSide::Sell,
        "1",
        "145.00",
        "144.50",
    )
    .client_order_id("stop-limit-example-001");
    println!("Stop-limit order: {:?}", stop_limit_order);

    // Trailing stop order (by price)
    let trailing_stop_price = CreateOrderRequest::trailing_stop_price(
        "AAPL",
        OrderSide::Sell,
        "1",
        "5.00",
    )
    .client_order_id("trailing-price-example-001");
    println!("Trailing stop (price): {:?}", trailing_stop_price);

    // Trailing stop order (by percent)
    let trailing_stop_percent = CreateOrderRequest::trailing_stop_percent(
        "AAPL",
        OrderSide::Sell,
        "1",
        "2.5",
    )
    .client_order_id("trailing-percent-example-001");
    println!("Trailing stop (percent): {:?}", trailing_stop_percent);

    // Example 3: Query existing orders
    println!("\n--- Querying Orders ---");
    
    let order_params = OrderParams::new()
        .status(OrderQueryStatus::All)
        .limit(10)
        .nested(true);
    
    let orders = client.get_orders(&order_params).await?;
    println!("Found {} orders", orders.len());
    
    for order in orders.iter().take(5) {
        println!(
            "  - {} {} {} @ {:?} ({})",
            order.side,
            order.qty.as_deref().unwrap_or("N/A"),
            order.symbol,
            order.limit_price,
            order.status
        );
    }

    println!("\nExample completed successfully!");
    Ok(())
}
