//! # IRA Types
//!
//! This example demonstrates IRA account types and related structures.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_ira_types
//! ```

use alpaca_base::{IraAccountType, IraBeneficiary, IraContribution, IraDistribution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== IRA Types ===\n");

    // 1. IRA Account Types
    println!("--- IRA Account Types ---");
    demonstrate_account_types()?;

    // 2. IRA Contributions
    println!("\n--- IRA Contributions ---");
    demonstrate_contributions()?;

    // 3. IRA Distributions
    println!("\n--- IRA Distributions ---");
    demonstrate_distributions()?;

    // 4. IRA Beneficiaries
    println!("\n--- IRA Beneficiaries ---");
    demonstrate_beneficiaries()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_account_types() -> Result<(), Box<dyn std::error::Error>> {
    let types = [
        (
            IraAccountType::Traditional,
            "Tax-deferred contributions, taxed on withdrawal",
        ),
        (
            IraAccountType::Roth,
            "After-tax contributions, tax-free growth",
        ),
        (
            IraAccountType::Sep,
            "Simplified Employee Pension for self-employed",
        ),
        (
            IraAccountType::Simple,
            "Savings Incentive Match Plan for Employees",
        ),
    ];

    println!("  IRA Account Types:");
    for (ira_type, description) in &types {
        let json = serde_json::to_string(ira_type)?;
        println!("    {} - {}", json, description);
    }

    // Display trait
    println!("\n  Display format:");
    for (ira_type, _) in &types {
        println!("    {:?} displays as: {}", ira_type, ira_type);
    }

    Ok(())
}

fn demonstrate_contributions() -> Result<(), Box<dyn std::error::Error>> {
    let contribution = IraContribution {
        id: "contrib_123".to_string(),
        account_id: "account_456".to_string(),
        amount: "6500.00".to_string(),
        tax_year: 2024,
        date: "2024-03-15".to_string(),
        contribution_type: Some("regular".to_string()),
    };

    println!("  Sample Contribution:");
    println!("    ID: {}", contribution.id);
    println!("    Amount: ${}", contribution.amount);
    println!("    Year: {}", contribution.tax_year);
    println!("    Type: {:?}", contribution.contribution_type);
    println!("    Date: {}", contribution.date);

    // 2024 contribution limits
    println!("\n  2024 Contribution Limits:");
    println!("    Under 50: $7,000");
    println!("    50 and over: $8,000 (includes $1,000 catch-up)");

    Ok(())
}

fn demonstrate_distributions() -> Result<(), Box<dyn std::error::Error>> {
    let distribution = IraDistribution {
        id: "dist_789".to_string(),
        account_id: "account_456".to_string(),
        amount: "10000.00".to_string(),
        date: "2024-06-15".to_string(),
        reason: Some("normal".to_string()),
        federal_withholding: Some("1000.00".to_string()),
        state_withholding: Some("500.00".to_string()),
    };

    println!("  Sample Distribution:");
    println!("    ID: {}", distribution.id);
    println!("    Amount: ${}", distribution.amount);
    println!("    Reason: {:?}", distribution.reason);
    println!(
        "    Federal Withholding: {:?}",
        distribution.federal_withholding
    );
    println!(
        "    State Withholding: {:?}",
        distribution.state_withholding
    );
    println!("    Date: {}", distribution.date);

    println!("\n  Distribution Types:");
    println!("    - normal: Regular distribution (59½+)");
    println!("    - early: Early distribution (before 59½, may incur penalty)");
    println!("    - rmd: Required Minimum Distribution (72+)");
    println!("    - rollover: Transfer to another retirement account");

    Ok(())
}

fn demonstrate_beneficiaries() -> Result<(), Box<dyn std::error::Error>> {
    let beneficiary = IraBeneficiary {
        id: "ben_001".to_string(),
        account_id: "account_456".to_string(),
        name: "Jane Doe".to_string(),
        beneficiary_type: "primary".to_string(),
        percentage: 100.0,
        relationship: Some("spouse".to_string()),
    };

    println!("  Sample Beneficiary:");
    println!("    Name: {}", beneficiary.name);
    println!("    Relationship: {:?}", beneficiary.relationship);
    println!("    Type: {}", beneficiary.beneficiary_type);
    println!("    Percentage: {}%", beneficiary.percentage);

    println!("\n  Beneficiary Types:");
    println!("    - primary: First in line to receive assets");
    println!("    - contingent: Receives if primary is unavailable");

    Ok(())
}
