//! # IRA Contributions
//!
//! This example demonstrates how to manage IRA contributions
//! for broker accounts using the Alpaca Broker API.
//!
//! ## Prerequisites
//!
//! Set environment variables:
//! - `ALPACA_API_KEY`: Your Alpaca Broker API key
//! - `ALPACA_API_SECRET`: Your Alpaca Broker secret key
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-http --example http_ira_contributions
//! ```
//!
//! **Note**: This example demonstrates the API structure. Broker API
//! requires special credentials and IRA-enabled accounts.

fn main() {
    println!("=== IRA Contributions ===\n");

    // IRA account types
    println!("--- IRA Account Types ---");
    println!("  Traditional IRA: Tax-deductible contributions, taxed on withdrawal");
    println!("  Roth IRA: After-tax contributions, tax-free growth and withdrawal");
    println!("  SEP IRA: For self-employed and small business owners");
    println!("  SIMPLE IRA: For small businesses with employees");

    // Contribution limits (2024)
    println!("\n--- Contribution Limits (2024) ---");
    println!("  Under 50: $7,000 per year");
    println!("  50 and over: $8,000 per year (includes $1,000 catch-up)");
    println!();
    println!("  Note: Limits apply across all IRA accounts combined");

    // List contributions
    println!("\n--- List IRA Contributions ---");
    println!("  let params = ListIraContributionsParams::new()");
    println!("      .tax_year(2024)");
    println!("      .contribution_type(IraContributionType::Regular);");
    println!("  let contributions = client.list_ira_contributions(&account_id, &params).await?;");
    println!("  for contrib in contributions {{");
    println!("      println!(\"Amount: ${{}}\", contrib.amount);");
    println!("      println!(\"Type: {{:?}}\", contrib.contribution_type);");
    println!("      println!(\"Tax Year: {{}}\", contrib.tax_year);");
    println!("  }}");

    // Contribution types
    println!("\n--- Contribution Types ---");
    println!("  REGULAR: Standard annual contribution");
    println!("  ROLLOVER: Transfer from another retirement account");
    println!("  CONVERSION: Roth conversion from Traditional IRA");
    println!("  RECHARACTERIZATION: Change contribution type");
    println!("  EMPLOYER: Employer contribution (SEP/SIMPLE)");

    // Create contribution
    println!("\n--- Create IRA Contribution ---");
    println!("  let request = CreateIraContributionRequest {{");
    println!("      amount: \"5000.00\".to_string(),");
    println!("      contribution_type: IraContributionType::Regular,");
    println!("      tax_year: 2024,");
    println!("  }};");
    println!("  let contribution = client.create_ira_contribution(&account_id, &request).await?;");

    // Contribution deadlines
    println!("\n--- Contribution Deadlines ---");
    println!("  Regular contributions: April 15 of following year");
    println!("  Example: 2024 contributions due by April 15, 2025");
    println!();
    println!("  Employer contributions (SEP): Tax filing deadline + extensions");

    // Distributions
    println!("\n--- IRA Distributions ---");
    println!("  let request = CreateIraDistributionRequest {{");
    println!("      amount: \"1000.00\".to_string(),");
    println!("      distribution_type: IraDistributionType::Normal,");
    println!("      federal_tax_withholding: Some(\"10\".to_string()),");
    println!("      state_tax_withholding: Some(\"5\".to_string()),");
    println!("  }};");
    println!("  let distribution = client.create_ira_distribution(&account_id, &request).await?;");

    // Distribution types
    println!("\n--- Distribution Types ---");
    println!("  NORMAL: Standard distribution (59½ or older)");
    println!("  EARLY: Early distribution (before 59½, may have penalty)");
    println!("  QUALIFIED: Qualified Roth distribution");
    println!("  REQUIRED: Required Minimum Distribution (RMD)");
    println!("  ROLLOVER: Transfer to another retirement account");

    // Tax withholding
    println!("\n--- Tax Withholding ---");
    println!("  Federal: Default 10% for Traditional IRA distributions");
    println!("  State: Varies by state");
    println!();
    println!("  Roth IRA qualified distributions: No withholding required");

    // Beneficiaries
    println!("\n--- IRA Beneficiaries ---");
    println!("  let beneficiaries = client.list_ira_beneficiaries(&account_id).await?;");
    println!("  for ben in beneficiaries {{");
    println!("      println!(\"Name: {{}} {{}}\", ben.first_name, ben.last_name);");
    println!("      println!(\"Percentage: {{}}%\", ben.percentage);");
    println!("      println!(\"Type: {{:?}}\", ben.beneficiary_type);");
    println!("  }}");

    // Compliance
    println!("\n--- Compliance Considerations ---");
    println!("1. Verify contribution limits before accepting");
    println!("2. Track contributions across tax years");
    println!("3. Calculate RMDs for applicable accounts");
    println!("4. Report distributions to IRS (Form 1099-R)");
    println!("5. Maintain beneficiary designations");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Provide contribution limit tracking to users");
    println!("2. Send reminders before contribution deadlines");
    println!("3. Calculate and display RMD amounts");
    println!("4. Allow easy beneficiary management");
    println!("5. Provide tax withholding calculators");

    println!("\n=== Example Complete ===");
}
