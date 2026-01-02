//! # Transfers
//!
//! This example demonstrates how to create and manage fund transfers
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
//! cargo run -p alpaca-http --example http_transfers
//! ```
//!
//! **Note**: This example demonstrates the API structure. Broker API
//! requires special credentials.

fn main() {
    println!("=== Transfers ===\n");

    // Transfer types
    println!("--- Transfer Types ---");
    println!("  ACH: Bank transfer via ACH network (1-3 business days)");
    println!("  WIRE: Wire transfer (same day, higher fees)");
    println!("  INTERNAL: Transfer between Alpaca accounts");

    // Transfer directions
    println!("\n--- Transfer Directions ---");
    println!("  INCOMING: Deposit funds into trading account");
    println!("  OUTGOING: Withdraw funds from trading account");

    // Create ACH transfer (deposit)
    println!("\n--- Create ACH Deposit ---");
    println!("  let request = CreateTransferRequest {{");
    println!("      transfer_type: TransferType::Ach,");
    println!("      relationship_id: \"ach-relationship-uuid\".to_string(),");
    println!("      amount: \"1000.00\".to_string(),");
    println!("      direction: TransferDirection::Incoming,");
    println!("  }};");
    println!("  let transfer = client.create_transfer(&account_id, &request).await?;");

    // Create ACH transfer (withdrawal)
    println!("\n--- Create ACH Withdrawal ---");
    println!("  let request = CreateTransferRequest {{");
    println!("      transfer_type: TransferType::Ach,");
    println!("      relationship_id: \"ach-relationship-uuid\".to_string(),");
    println!("      amount: \"500.00\".to_string(),");
    println!("      direction: TransferDirection::Outgoing,");
    println!("  }};");
    println!("  let transfer = client.create_transfer(&account_id, &request).await?;");

    // Transfer states
    println!("\n--- Transfer States ---");
    println!("  QUEUED: Transfer queued for processing");
    println!("  PENDING: Transfer in progress");
    println!("  SENT_TO_CLEARING: Sent to clearing house");
    println!("  COMPLETE: Transfer completed successfully");
    println!("  CANCELED: Transfer canceled");
    println!("  RETURNED: Transfer returned (e.g., NSF)");
    println!("  REJECTED: Transfer rejected");

    // List transfers
    println!("\n--- List Transfers ---");
    println!("  let params = ListTransfersParams::new()");
    println!("      .direction(TransferDirection::Incoming)");
    println!("      .limit(50);");
    println!("  let transfers = client.list_transfers(&account_id, &params).await?;");
    println!("  for transfer in transfers {{");
    println!(
        "      println!(\"{{}} - ${{}} {{:?}}\", transfer.id, transfer.amount, transfer.status);"
    );
    println!("  }}");

    // Get single transfer
    println!("\n--- Get Transfer ---");
    println!("  let transfer = client.get_transfer(&account_id, &transfer_id).await?;");
    println!("  println!(\"Amount: ${{}}\", transfer.amount);");
    println!("  println!(\"Status: {{:?}}\", transfer.status);");
    println!("  println!(\"Created: {{}}\", transfer.created_at);");

    // Cancel transfer
    println!("\n--- Cancel Transfer ---");
    println!("  client.cancel_transfer(&account_id, &transfer_id).await?;");
    println!("  Note: Only QUEUED or PENDING transfers can be canceled");

    // Transfer timing
    println!("\n--- Transfer Timing ---");
    println!("  ACH Deposits:");
    println!("    - Standard: 3-5 business days");
    println!("    - Same-day ACH: 1 business day (if supported)");
    println!();
    println!("  ACH Withdrawals:");
    println!("    - Standard: 3-5 business days");
    println!();
    println!("  Wire Transfers:");
    println!("    - Incoming: Same day (before cutoff)");
    println!("    - Outgoing: Same day (before cutoff)");

    // Instant deposits
    println!("\n--- Instant Deposits ---");
    println!("  Some accounts may have instant deposit enabled:");
    println!("  - Funds available immediately for trading");
    println!("  - Subject to limits based on account history");
    println!("  - ACH still settles in background");

    // Error handling
    println!("\n--- Common Errors ---");
    println!("  insufficient_funds: Not enough balance for withdrawal");
    println!("  invalid_amount: Amount must be positive");
    println!("  relationship_not_approved: ACH not yet verified");
    println!("  transfer_limit_exceeded: Daily/monthly limit reached");
    println!("  account_restricted: Account has trading restrictions");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Validate amounts before submission");
    println!("2. Show estimated completion dates to users");
    println!("3. Implement webhooks for transfer status updates");
    println!("4. Keep audit trail of all transfers");
    println!("5. Set appropriate daily/monthly limits");

    println!("\n=== Example Complete ===");
}
