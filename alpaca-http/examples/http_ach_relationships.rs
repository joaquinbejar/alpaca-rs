//! # ACH Relationships
//!
//! This example demonstrates how to manage ACH funding relationships
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
//! cargo run -p alpaca-http --example http_ach_relationships
//! ```
//!
//! **Note**: This example demonstrates the API structure. Broker API
//! requires special credentials.

fn main() {
    println!("=== ACH Relationships ===\n");

    // What is ACH?
    println!("--- What is ACH? ---");
    println!("  ACH (Automated Clearing House) is a network for");
    println!("  electronic fund transfers between bank accounts.");
    println!("  Used for deposits and withdrawals from trading accounts.");

    // Create ACH relationship
    println!("\n--- Create ACH Relationship ---");
    println!("  Required fields:");
    println!("    - account_owner_name: Name on the bank account");
    println!("    - bank_account_type: CHECKING or SAVINGS");
    println!("    - bank_account_number: Bank account number");
    println!("    - bank_routing_number: 9-digit ABA routing number");
    println!();
    println!("  API call:");
    println!("    let request = CreateAchRelationshipRequest {{");
    println!("        account_owner_name: \"John Doe\".to_string(),");
    println!("        bank_account_type: BankAccountType::Checking,");
    println!("        bank_account_number: \"123456789\".to_string(),");
    println!("        bank_routing_number: \"021000021\".to_string(),");
    println!("        nickname: Some(\"My Checking\".to_string()),");
    println!("    }};");
    println!("    let ach = client.create_ach_relationship(&account_id, &request).await?;");

    // ACH relationship states
    println!("\n--- ACH Relationship States ---");
    println!("  QUEUED: Relationship creation queued");
    println!("  PENDING: Awaiting micro-deposit verification");
    println!("  APPROVED: Ready for transfers");
    println!("  CANCELED: Relationship canceled");

    // List ACH relationships
    println!("\n--- List ACH Relationships ---");
    println!("  let relationships = client.list_ach_relationships(&account_id).await?;");
    println!("  for ach in relationships {{");
    println!("      println!(\"ID: {{}}\", ach.id);");
    println!("      println!(\"Bank: {{}}\", ach.bank_name);");
    println!("      println!(\"Status: {{:?}}\", ach.status);");
    println!("      println!(\"Last 4: {{}}\", ach.account_last_four);");
    println!("  }}");

    // Micro-deposit verification
    println!("\n--- Micro-Deposit Verification ---");
    println!("  When ACH is PENDING, verify with micro-deposits:");
    println!("  1. Alpaca sends two small deposits (e.g., $0.12, $0.34)");
    println!("  2. User checks bank statement for amounts");
    println!("  3. Submit amounts for verification:");
    println!();
    println!("    let amounts = [\"0.12\", \"0.34\"];");
    println!("    client.verify_ach_relationship(&account_id, &ach_id, &amounts).await?;");

    // Delete ACH relationship
    println!("\n--- Delete ACH Relationship ---");
    println!("  client.delete_ach_relationship(&account_id, &ach_id).await?;");
    println!("  Note: Cannot delete if pending transfers exist");

    // Bank account types
    println!("\n--- Bank Account Types ---");
    println!("  CHECKING: Standard checking account");
    println!("  SAVINGS: Savings account");

    // Security considerations
    println!("\n--- Security Considerations ---");
    println!("1. Never log full bank account numbers");
    println!("2. Use HTTPS for all API calls");
    println!("3. Validate routing numbers before submission");
    println!("4. Implement rate limiting on verification attempts");
    println!("5. Monitor for suspicious activity patterns");

    // Common errors
    println!("\n--- Common Errors ---");
    println!("  invalid_routing_number: ABA routing number not valid");
    println!("  duplicate_relationship: Bank account already linked");
    println!("  verification_failed: Micro-deposit amounts incorrect");
    println!("  account_not_found: Broker account doesn't exist");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Allow users to nickname their bank accounts");
    println!("2. Show only last 4 digits of account numbers");
    println!("3. Provide clear verification instructions");
    println!("4. Set reasonable timeout for verification (3-5 days)");
    println!("5. Send reminders for pending verifications");

    println!("\n=== Example Complete ===");
}
