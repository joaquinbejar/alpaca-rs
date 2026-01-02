//! # Options Scanner
//!
//! This example demonstrates how to scan and filter options contracts
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
//! cargo run --example integration_options_scanner
//! ```
//!
//! **Note**: Options trading requires approval from Alpaca.

use alpaca_base::{Credentials, Environment};

fn main() {
    println!("=== Options Scanner ===\n");

    // Options overview
    println!("--- Options Overview ---");
    println!("  Options give the right (not obligation) to buy/sell");
    println!("  an underlying asset at a specific price by a date.");
    println!();
    println!("  Call: Right to BUY at strike price");
    println!("  Put: Right to SELL at strike price");

    // Initialize client
    println!("\n--- Initialize Client ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;");

    // Get options chain
    println!("\n--- Get Options Chain ---");
    println!("  let params = OptionsChainParams::new(\"AAPL\")");
    println!("      .expiration_date_gte(\"2024-03-01\")");
    println!("      .expiration_date_lte(\"2024-06-30\")");
    println!("      .strike_price_gte(140.0)");
    println!("      .strike_price_lte(180.0);");
    println!("  ");
    println!("  let chain = client.get_options_chain(&params).await?;");
    println!("  println!(\"Found {{}} contracts\", chain.len());");

    // Filter by type
    println!("\n--- Filter by Option Type ---");
    println!("  // Get only calls");
    println!("  let calls: Vec<_> = chain.iter()");
    println!("      .filter(|c| c.option_type == OptionType::Call)");
    println!("      .collect();");
    println!("  ");
    println!("  // Get only puts");
    println!("  let puts: Vec<_> = chain.iter()");
    println!("      .filter(|c| c.option_type == OptionType::Put)");
    println!("      .collect();");

    // Options contract fields
    println!("\n--- Options Contract Fields ---");
    println!("  symbol: Option symbol (e.g., AAPL240315C00150000)");
    println!("  underlying_symbol: Underlying stock (e.g., AAPL)");
    println!("  option_type: Call or Put");
    println!("  strike_price: Strike price");
    println!("  expiration_date: Expiration date");
    println!("  open_interest: Number of open contracts");
    println!("  implied_volatility: IV percentage");

    // Scanner criteria
    println!("\n--- Scanner Criteria ---");
    println!("  struct ScanCriteria {{");
    println!("      min_open_interest: u32,");
    println!("      max_days_to_expiry: u32,");
    println!("      min_implied_volatility: f64,");
    println!("      max_implied_volatility: f64,");
    println!("      moneyness: Moneyness,  // ITM, ATM, OTM");
    println!("  }}");
    println!("  ");
    println!("  enum Moneyness {{");
    println!("      InTheMoney,   // Call: strike < price, Put: strike > price");
    println!("      AtTheMoney,   // strike ≈ price");
    println!("      OutOfMoney,   // Call: strike > price, Put: strike < price");
    println!("  }}");

    // Scan for high IV options
    println!("\n--- Scan: High IV Options ---");
    println!("  let high_iv = chain.iter()");
    println!("      .filter(|c| c.implied_volatility > 0.50)  // IV > 50%");
    println!("      .filter(|c| c.open_interest > 100)");
    println!("      .collect::<Vec<_>>();");
    println!("  ");
    println!("  println!(\"High IV options:\");");
    println!("  for opt in high_iv {{");
    println!("      println!(\"  {{}} IV: {{:.1}}%\", opt.symbol, opt.implied_volatility * 100.0);");
    println!("  }}");

    // Scan for liquid options
    println!("\n--- Scan: Liquid Options ---");
    println!("  let liquid = chain.iter()");
    println!("      .filter(|c| c.open_interest > 1000)");
    println!("      .filter(|c| c.volume > 100)");
    println!("      .collect::<Vec<_>>();");

    // Scan for near-expiry options
    println!("\n--- Scan: Near Expiry (< 30 days) ---");
    println!("  let today = Utc::now().date_naive();");
    println!("  let near_expiry = chain.iter()");
    println!("      .filter(|c| {{");
    println!("          let expiry = NaiveDate::parse_from_str(&c.expiration_date, \"%Y-%m-%d\").unwrap();");
    println!("          (expiry - today).num_days() < 30");
    println!("      }})");
    println!("      .collect::<Vec<_>>();");

    // Greeks (if available)
    println!("\n--- Options Greeks ---");
    println!("  Delta: Price change per $1 move in underlying");
    println!("  Gamma: Rate of change of delta");
    println!("  Theta: Time decay per day");
    println!("  Vega: Price change per 1% IV change");
    println!("  Rho: Price change per 1% interest rate change");

    // Strategy scanner
    println!("\n--- Strategy: Covered Call Scanner ---");
    println!("  // Find OTM calls for covered call strategy");
    println!("  let underlying_price = 150.0;");
    println!("  let covered_call_candidates = chain.iter()");
    println!("      .filter(|c| c.option_type == OptionType::Call)");
    println!("      .filter(|c| c.strike_price > underlying_price * 1.05)  // 5% OTM");
    println!("      .filter(|c| days_to_expiry(c) >= 30 && days_to_expiry(c) <= 45)");
    println!("      .filter(|c| c.open_interest > 100)");
    println!("      .collect::<Vec<_>>();");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Filter by open interest for liquidity");
    println!("2. Check bid-ask spread before trading");
    println!("3. Consider time decay (theta) for short positions");
    println!("4. Monitor implied volatility changes");
    println!("5. Understand assignment risk for ITM options");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;

    println!("\n=== Example Complete ===");
}
