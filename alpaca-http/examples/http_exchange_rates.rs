//! # Exchange Rates
//!
//! This example demonstrates how to get FX exchange rates
//! using the Alpaca HTTP API.
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
//! cargo run -p alpaca-http --example http_exchange_rates
//! ```

use alpaca_base::{Currency, ExchangeRate};

fn main() {
    println!("=== Exchange Rates ===\n");

    // Get all exchange rates
    println!("--- Get All Exchange Rates ---");
    println!("  let rates = client.get_exchange_rates().await?;");
    println!("  for rate in rates {{");
    println!("      println!(\"{{:?}}/{{:?}}: {{}}\", rate.base, rate.quote, rate.rate);");
    println!("  }}");

    // Get specific currency pair
    println!("\n--- Get Specific Currency Pair ---");
    println!("  let rate = client.get_exchange_rate(\"EURUSD\").await?;");
    println!("  println!(\"EUR/USD: {{}}\", rate.rate);");

    // Common currency pairs
    println!("\n--- Common Currency Pairs ---");
    let pairs = [
        ("EURUSD", "Euro to US Dollar"),
        ("GBPUSD", "British Pound to US Dollar"),
        ("USDJPY", "US Dollar to Japanese Yen"),
        ("USDCAD", "US Dollar to Canadian Dollar"),
        ("AUDUSD", "Australian Dollar to US Dollar"),
        ("USDCHF", "US Dollar to Swiss Franc"),
    ];

    for (pair, description) in pairs {
        println!("  {}: {}", pair, description);
    }

    // Currency conversion example
    println!("\n--- Currency Conversion ---");
    let rate = ExchangeRate::new(Currency::Eur, Currency::Usd, 1.10);
    let amount_eur = 100.0;
    let amount_usd = rate.convert(amount_eur);

    println!("  Example rate: EUR/USD = {}", rate.rate);
    println!("  Convert {} EUR to USD: {:.2} USD", amount_eur, amount_usd);
    println!("  Inverse rate (USD/EUR): {:.6}", rate.inverse());

    // Available currencies
    println!("\n--- Available Currencies ---");
    println!("  USD: US Dollar");
    println!("  EUR: Euro");
    println!("  GBP: British Pound");
    println!("  JPY: Japanese Yen");
    println!("  CAD: Canadian Dollar");
    println!("  AUD: Australian Dollar");
    println!("  CHF: Swiss Franc");
    println!("  CNY: Chinese Yuan");
    println!("  HKD: Hong Kong Dollar");
    println!("  SGD: Singapore Dollar");

    // Exchange rate fields
    println!("\n--- Exchange Rate Fields ---");
    println!("  base: Base currency");
    println!("  quote: Quote currency");
    println!("  rate: Exchange rate value");
    println!("  timestamp: Rate timestamp");

    // Use cases
    println!("\n--- Use Cases ---");
    println!("1. Convert international stock prices");
    println!("2. Calculate portfolio value in home currency");
    println!("3. Analyze currency exposure");
    println!("4. FX trading strategies");
    println!("5. International transaction calculations");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Cache rates with reasonable TTL");
    println!("2. Handle rate staleness gracefully");
    println!("3. Use mid-market rates for estimates");
    println!("4. Consider bid/ask spread for actual trades");
    println!("5. Log rate timestamps for audit trails");

    println!("\n=== Example Complete ===");
}
