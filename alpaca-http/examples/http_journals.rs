//! # Journals
//!
//! This example demonstrates how to create and manage journal entries
//! for transferring assets between broker accounts.
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
//! cargo run -p alpaca-http --example http_journals
//! ```
//!
//! **Note**: This example demonstrates the API structure. Broker API
//! requires special credentials.

fn main() {
    println!("=== Journals ===\n");

    // What are journals?
    println!("--- What are Journals? ---");
    println!("  Journals are internal transfers between broker accounts.");
    println!("  Used for:");
    println!("    - Moving cash between accounts");
    println!("    - Transferring securities between accounts");
    println!("    - Correcting errors");
    println!("    - Fee collection");

    // Journal types
    println!("\n--- Journal Entry Types ---");
    println!("  JNLC: Cash journal (transfer cash)");
    println!("  JNLS: Security journal (transfer securities)");

    // Create cash journal
    println!("\n--- Create Cash Journal (JNLC) ---");
    println!("  let request = CreateJournalRequest {{");
    println!("      entry_type: JournalEntryType::Jnlc,");
    println!("      from_account: \"source-account-uuid\".to_string(),");
    println!("      to_account: \"dest-account-uuid\".to_string(),");
    println!("      amount: \"100.00\".to_string(),");
    println!("      symbol: None,");
    println!("      qty: None,");
    println!("      description: Some(\"Monthly fee\".to_string()),");
    println!("  }};");
    println!("  let journal = client.create_journal(&request).await?;");

    // Create security journal
    println!("\n--- Create Security Journal (JNLS) ---");
    println!("  let request = CreateJournalRequest {{");
    println!("      entry_type: JournalEntryType::Jnls,");
    println!("      from_account: \"source-account-uuid\".to_string(),");
    println!("      to_account: \"dest-account-uuid\".to_string(),");
    println!("      amount: None,");
    println!("      symbol: Some(\"AAPL\".to_string()),");
    println!("      qty: Some(\"10\".to_string()),");
    println!("      description: Some(\"Security transfer\".to_string()),");
    println!("  }};");
    println!("  let journal = client.create_journal(&request).await?;");

    // Journal states
    println!("\n--- Journal States ---");
    println!("  QUEUED: Journal queued for processing");
    println!("  PENDING: Journal being processed");
    println!("  EXECUTED: Journal completed successfully");
    println!("  CANCELED: Journal canceled");
    println!("  REJECTED: Journal rejected");

    // List journals
    println!("\n--- List Journals ---");
    println!("  let params = ListJournalsParams::new()");
    println!("      .entry_type(JournalEntryType::Jnlc)");
    println!("      .after(\"2024-01-01\")");
    println!("      .before(\"2024-12-31\");");
    println!("  let journals = client.list_journals(&params).await?;");

    // Get journal
    println!("\n--- Get Journal ---");
    println!("  let journal = client.get_journal(&journal_id).await?;");
    println!("  println!(\"ID: {{}}\", journal.id);");
    println!("  println!(\"Type: {{:?}}\", journal.entry_type);");
    println!("  println!(\"Status: {{:?}}\", journal.status);");

    // Cancel journal
    println!("\n--- Cancel Journal ---");
    println!("  client.cancel_journal(&journal_id).await?;");
    println!("  Note: Only QUEUED or PENDING journals can be canceled");

    // Batch journals
    println!("\n--- Batch Journals ---");
    println!("  Create multiple journals in one request:");
    println!("  let entries = vec![");
    println!("      BatchJournalEntry {{ to_account: \"acc1\", amount: \"10.00\" }},");
    println!("      BatchJournalEntry {{ to_account: \"acc2\", amount: \"20.00\" }},");
    println!("  ];");
    println!("  let request = BatchJournalRequest {{");
    println!("      entry_type: JournalEntryType::Jnlc,");
    println!("      from_account: \"source-account\".to_string(),");
    println!("      entries,");
    println!("  }};");
    println!("  let results = client.create_batch_journal(&request).await?;");

    // Use cases
    println!("\n--- Common Use Cases ---");
    println!("  1. Fee Collection");
    println!("     - Transfer fees from customer to firm account");
    println!();
    println!("  2. Interest Payments");
    println!("     - Distribute interest to customer accounts");
    println!();
    println!("  3. Error Correction");
    println!("     - Move funds/securities to correct accounts");
    println!();
    println!("  4. Account Consolidation");
    println!("     - Merge multiple accounts into one");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Always include descriptive notes");
    println!("2. Validate account balances before journaling");
    println!("3. Use batch journals for efficiency");
    println!("4. Keep audit trail of all journals");
    println!("5. Implement approval workflow for large amounts");

    println!("\n=== Example Complete ===");
}
