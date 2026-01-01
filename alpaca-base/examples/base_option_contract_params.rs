//! # Option Contract Parameters
//!
//! This example demonstrates how to build options contract query parameters.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_option_contract_params
//! ```

use alpaca_base::{OptionContractParams, OptionStyle, OptionType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Option Contract Parameters ===\n");

    // 1. Basic Query
    println!("--- Basic Query ---");
    demonstrate_basic_query()?;

    // 2. Filtered Query
    println!("\n--- Filtered Query ---");
    demonstrate_filtered_query()?;

    // 3. Option Types and Styles
    println!("\n--- Option Types and Styles ---");
    demonstrate_option_enums()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_basic_query() -> Result<(), Box<dyn std::error::Error>> {
    // Query all options for a symbol
    let params = OptionContractParams::new().underlying_symbol("AAPL");

    println!("  Query: All AAPL options");
    println!("  Params: {:?}", params);

    let json = serde_json::to_string_pretty(&params)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}

fn demonstrate_filtered_query() -> Result<(), Box<dyn std::error::Error>> {
    // Query with multiple filters
    let params = OptionContractParams::new()
        .underlying_symbol("AAPL")
        .expiration_date("2024-03-15")
        .option_type(OptionType::Call)
        .strike_price_range("150.00", "200.00");

    println!("  Query: AAPL calls expiring 2024-03-15, strikes $150-$200");
    println!("\n  Parameters:");
    println!("    Underlying: {:?}", params.underlying_symbol);
    println!("    Expiration: {:?}", params.expiration_date);
    println!("    Type: {:?}", params.option_type);
    println!("    Strike >= {:?}", params.strike_price_gte);
    println!("    Strike <= {:?}", params.strike_price_lte);

    let json = serde_json::to_string_pretty(&params)?;
    println!("\n  JSON:\n{}", json);

    // Query puts
    let puts_params = OptionContractParams::new()
        .underlying_symbol("SPY")
        .option_type(OptionType::Put)
        .expiration_date("2024-03-15");

    println!("\n  Query: SPY puts expiring in H1 2024");
    let json = serde_json::to_string_pretty(&puts_params)?;
    println!("{}", json);

    Ok(())
}

fn demonstrate_option_enums() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Option Types:");
    let types = [
        (OptionType::Call, "Right to BUY at strike price"),
        (OptionType::Put, "Right to SELL at strike price"),
    ];
    for (opt_type, desc) in &types {
        let json = serde_json::to_string(opt_type)?;
        println!("    {:?} ({}) - {}", opt_type, json, desc);
    }

    println!("\n  Option Styles:");
    let styles = [
        (
            OptionStyle::American,
            "Can exercise any time before expiration",
        ),
        (OptionStyle::European, "Can only exercise at expiration"),
    ];
    for (style, desc) in &styles {
        let json = serde_json::to_string(style)?;
        println!("    {:?} ({}) - {}", style, json, desc);
    }

    // OCC Symbol format explanation
    println!("\n  OCC Symbol Format:");
    println!("    Example: AAPL240315C00150000");
    println!("    ├── AAPL      = Underlying symbol");
    println!("    ├── 240315    = Expiration (YYMMDD)");
    println!("    ├── C         = Call (P for Put)");
    println!("    └── 00150000  = Strike ($150.00 * 1000)");

    Ok(())
}
