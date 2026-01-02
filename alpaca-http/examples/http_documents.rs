//! # Documents
//!
//! This example demonstrates how to list and retrieve documents
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
//! cargo run -p alpaca-http --example http_documents
//! ```
//!
//! **Note**: This example demonstrates the API structure. Broker API
//! requires special credentials.

fn main() {
    println!("=== Documents ===\n");

    // Document types
    println!("--- Document Types ---");
    println!("  account_statement: Monthly account statements");
    println!("  trade_confirmation: Trade confirmations");
    println!("  tax_statement: Tax documents (1099, etc.)");
    println!("  account_application: Account application documents");
    println!("  margin_disclosure: Margin agreement disclosures");

    // List documents
    println!("\n--- List Documents ---");
    println!("  let params = ListDocumentsParams::new()");
    println!("      .start(\"2024-01-01\")");
    println!("      .end(\"2024-12-31\")");
    println!("      .doc_type(DocumentType::AccountStatement);");
    println!("  let documents = client.list_documents(&account_id, &params).await?;");
    println!("  for doc in documents {{");
    println!("      println!(\"ID: {{}}\", doc.id);");
    println!("      println!(\"Type: {{:?}}\", doc.document_type);");
    println!("      println!(\"Date: {{}}\", doc.date);");
    println!("  }}");

    // Get document
    println!("\n--- Get Document ---");
    println!("  let document = client.get_document(&account_id, &document_id).await?;");
    println!("  println!(\"ID: {{}}\", document.id);");
    println!("  println!(\"Type: {{:?}}\", document.document_type);");
    println!("  println!(\"Name: {{}}\", document.name);");

    // Download document
    println!("\n--- Download Document ---");
    println!("  let content = client.download_document(&account_id, &document_id).await?;");
    println!("  // content is the raw PDF/document bytes");
    println!("  std::fs::write(\"statement.pdf\", content)?;");

    // Document fields
    println!("\n--- Document Fields ---");
    println!("  id: Unique document identifier");
    println!("  document_type: Type of document");
    println!("  document_sub_type: Sub-type (if applicable)");
    println!("  name: Document name/filename");
    println!("  date: Document date");
    println!("  created_at: When document was created");

    // Tax documents
    println!("\n--- Tax Documents ---");
    println!("  1099-B: Proceeds from broker transactions");
    println!("  1099-DIV: Dividend income");
    println!("  1099-INT: Interest income");
    println!("  1099-MISC: Miscellaneous income");
    println!();
    println!("  Tax documents are typically available in February");
    println!("  for the previous tax year.");

    // Account statements
    println!("\n--- Account Statements ---");
    println!("  Generated monthly");
    println!("  Contains:");
    println!("    - Account summary");
    println!("    - Position details");
    println!("    - Transaction history");
    println!("    - Fees and charges");

    // Trade confirmations
    println!("\n--- Trade Confirmations ---");
    println!("  Generated for each trade");
    println!("  Contains:");
    println!("    - Trade details (symbol, qty, price)");
    println!("    - Execution time");
    println!("    - Commission and fees");
    println!("    - Settlement date");

    // Regulatory requirements
    println!("\n--- Regulatory Requirements ---");
    println!("  Brokers must provide:");
    println!("  - Monthly statements");
    println!("  - Trade confirmations");
    println!("  - Annual tax documents");
    println!("  - Regulatory disclosures");
    println!();
    println!("  Documents must be retained for regulatory periods.");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Provide easy document access to customers");
    println!("2. Send notifications when new documents are available");
    println!("3. Allow document download in multiple formats");
    println!("4. Implement document search and filtering");
    println!("5. Maintain document retention policies");

    println!("\n=== Example Complete ===");
}
