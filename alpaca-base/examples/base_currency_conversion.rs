//! # Currency Conversion
//!
//! This example demonstrates currency types and exchange rate utilities.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_currency_conversion
//! ```

use alpaca_base::{Currency, CurrencyPair, ExchangeRate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Currency Conversion ===\n");

    // 1. Currency Types
    println!("--- Currency Types ---");
    demonstrate_currencies()?;

    // 2. Currency Pairs
    println!("\n--- Currency Pairs ---");
    demonstrate_currency_pairs()?;

    // 3. Exchange Rates
    println!("\n--- Exchange Rates ---");
    demonstrate_exchange_rates()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_currencies() -> Result<(), Box<dyn std::error::Error>> {
    let currencies = [
        Currency::Usd,
        Currency::Eur,
        Currency::Gbp,
        Currency::Jpy,
        Currency::Cad,
        Currency::Aud,
        Currency::Chf,
    ];

    println!("  Supported Currencies:");
    for currency in &currencies {
        let json = serde_json::to_string(currency)?;
        println!("    {:?} -> {}", currency, json);
    }

    Ok(())
}

fn demonstrate_currency_pairs() -> Result<(), Box<dyn std::error::Error>> {
    // Create currency pair
    let pair = CurrencyPair::new(Currency::Eur, Currency::Usd);

    println!("  Currency Pair: {:?}/{:?}", pair.base, pair.quote);
    println!("    Base: {:?} (what you're buying/selling)", pair.base);
    println!("    Quote: {:?} (price currency)", pair.quote);

    // Common pairs
    println!("\n  Common FX Pairs:");
    let pairs = [
        CurrencyPair::new(Currency::Eur, Currency::Usd),
        CurrencyPair::new(Currency::Gbp, Currency::Usd),
        CurrencyPair::new(Currency::Usd, Currency::Jpy),
        CurrencyPair::new(Currency::Usd, Currency::Cad),
    ];
    for p in &pairs {
        println!("    {:?}/{:?}", p.base, p.quote);
    }

    Ok(())
}

fn demonstrate_exchange_rates() -> Result<(), Box<dyn std::error::Error>> {
    // Sample exchange rate
    let rate = ExchangeRate {
        base: Currency::Eur,
        quote: Currency::Usd,
        rate: 1.0850,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    println!("  Exchange Rate:");
    println!("    Pair: {:?}/{:?}", rate.base, rate.quote);
    println!("    Rate: {:.4}", rate.rate);
    println!("    Timestamp: {}", rate.timestamp);

    // Conversion examples
    let eur_amount = 1000.0;
    let usd_amount = rate.convert(eur_amount);
    println!("\n  Conversion:");
    println!("    {} EUR = {:.2} USD", eur_amount, usd_amount);

    // Inverse rate
    let inverse_rate = rate.inverse();
    println!("\n  Inverse Rate:");
    println!("    Pair: {:?}/{:?}", rate.quote, rate.base);
    println!("    Rate: {:.4}", inverse_rate);

    let usd_to_convert = 1000.0;
    let eur_result = usd_to_convert * inverse_rate;
    println!("    {} USD = {:.2} EUR", usd_to_convert, eur_result);

    Ok(())
}
