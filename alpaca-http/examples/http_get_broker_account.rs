//! # Get Broker Account
//!
//! This example demonstrates how to retrieve and list broker accounts
//! using the Alpaca Broker API.
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
//! cargo run -p alpaca-http --example http_get_broker_account
//! ```
//!
//! **Note**: This example demonstrates the API structure. Broker API
//! requires special credentials.

use alpaca_base::{BrokerAccountStatus, ListBrokerAccountsParams};

fn main() {
    println!("=== Get Broker Account ===\n");

    // Get single account by ID
    println!("--- Get Account by ID ---");
    println!("  let account = client.get_broker_account(\"account-uuid\").await?;");
    println!("  println!(\"Account ID: {{}}\", account.id);");
    println!("  println!(\"Account Number: {{}}\", account.account_number);");
    println!("  println!(\"Status: {{:?}}\", account.status);");

    // Account fields
    println!("\n--- Account Fields ---");
    println!("  id: Unique account identifier (UUID)");
    println!("  account_number: Human-readable account number");
    println!("  status: Current account status");
    println!("  crypto_status: Crypto trading status (if enabled)");
    println!("  currency: Account currency (e.g., USD)");
    println!("  created_at: Account creation timestamp");

    // List accounts with parameters
    println!("\n--- List Accounts ---");
    let params = ListBrokerAccountsParams::new()
        .status(BrokerAccountStatus::Active)
        .query("john");

    println!("  Filter parameters:");
    println!("    Status: {:?}", params.status);
    println!("    Query: {:?}", params.query);
    println!("    Sort: {:?}", params.sort);

    println!("\n  API call:");
    println!("    let accounts = client.list_broker_accounts(&params).await?;");
    println!("    for account in accounts {{");
    println!("        println!(\"{{}} - {{:?}}\", account.id, account.status);");
    println!("    }}");

    // Filter by status
    println!("\n--- Filter by Status ---");
    let statuses = [
        (BrokerAccountStatus::Onboarding, "New accounts being set up"),
        (BrokerAccountStatus::Active, "Fully active accounts"),
        (BrokerAccountStatus::Approved, "Awaiting full activation"),
        (BrokerAccountStatus::ActionRequired, "Need user action"),
        (BrokerAccountStatus::Disabled, "Disabled accounts"),
    ];

    for (status, description) in statuses {
        println!("  {:?}: {}", status, description);
    }

    // Filter by date range
    println!("\n--- Filter by Date ---");
    println!("  let params = ListBrokerAccountsParams::new()");
    println!("      .created_after(\"2024-01-01\")");
    println!("      .created_before(\"2024-12-31\");");

    // Pagination
    println!("\n--- Pagination ---");
    println!("  Results are paginated by default");
    println!("  Use query parameters to control:");
    println!("    - page_token: Token for next page");
    println!("    - page_size: Number of results per page");

    // Update account
    println!("\n--- Update Account ---");
    println!("  let update = UpdateBrokerAccountRequest {{");
    println!("      contact: Some(new_contact),");
    println!("      ..Default::default()");
    println!("  }};");
    println!("  let updated = client.update_broker_account(&id, &update).await?;");

    // Delete account
    println!("\n--- Close Account ---");
    println!("  client.close_broker_account(&account_id).await?;");
    println!("  Note: This initiates account closure process");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Cache account IDs to avoid repeated lookups");
    println!("2. Use status filters to find accounts needing attention");
    println!("3. Implement pagination for large account lists");
    println!("4. Monitor account status changes via webhooks");
    println!("5. Store account_number for customer reference");

    println!("\n=== Example Complete ===");
}
