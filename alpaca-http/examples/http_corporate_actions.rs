//! # Corporate Actions
//!
//! This example demonstrates how to list corporate actions
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
//! cargo run -p alpaca-http --example http_corporate_actions
//! ```

use alpaca_base::CorporateActionsParams;

fn main() {
    println!("=== Corporate Actions ===\n");

    // Get corporate actions
    println!("--- Get Corporate Actions ---");
    let params = CorporateActionsParams::new()
        .symbols("AAPL,MSFT,GOOGL")
        .date_range("2024-01-01", "2024-12-31")
        .limit(50);

    println!("  Parameters:");
    println!("    Symbols: {:?}", params.symbols);
    println!("    Start: {:?}", params.start);
    println!("    End: {:?}", params.end);
    println!("    Limit: {:?}", params.limit);

    println!("\n  API call:");
    println!("    let response = client.get_corporate_actions(&params).await?;");
    println!("    for action in response.corporate_actions {{");
    println!("        println!(\"{{:?}} - {{}}\", action.action_type, action.symbol);");
    println!("    }}");

    // Filter by action type
    println!("\n--- Filter by Action Type ---");
    let dividend_params = CorporateActionsParams::new()
        .types("dividend")
        .symbols("AAPL");

    println!("  Dividend filter:");
    println!("    Types: {:?}", dividend_params.types);

    // Corporate action types
    println!("\n--- Corporate Action Types ---");
    println!("  dividend: Cash dividend payment");
    println!("  split: Stock split (forward or reverse)");
    println!("  merger: Company merger");
    println!("  spinoff: Company spinoff");
    println!("  acquisition: Company acquisition");
    println!("  rights_issue: Rights offering");
    println!("  reorganization: Corporate reorganization");

    // Corporate action fields
    println!("\n--- Corporate Action Fields ---");
    println!("  id: Unique identifier");
    println!("  action_type: Type of corporate action");
    println!("  sub_type: Sub-type (e.g., cash dividend, stock dividend)");
    println!("  symbol: Stock symbol");
    println!("  initiating_symbol: Original symbol (for mergers)");
    println!("  target_symbol: Target symbol (for mergers)");
    println!("  declaration_date: When action was announced");
    println!("  ex_date: Ex-dividend/ex-distribution date");
    println!("  record_date: Record date for eligibility");
    println!("  payable_date: Payment date");
    println!("  cash: Cash amount per share");
    println!("  old_rate: Old share ratio (for splits)");
    println!("  new_rate: New share ratio (for splits)");

    // Dividend example
    println!("\n--- Dividend Example ---");
    println!("  action_type: dividend");
    println!("  sub_type: cash");
    println!("  symbol: AAPL");
    println!("  declaration_date: 2024-02-01");
    println!("  ex_date: 2024-02-09");
    println!("  record_date: 2024-02-12");
    println!("  payable_date: 2024-02-15");
    println!("  cash: 0.24");

    // Stock split example
    println!("\n--- Stock Split Example ---");
    println!("  action_type: split");
    println!("  sub_type: forward");
    println!("  symbol: NVDA");
    println!("  ex_date: 2024-06-10");
    println!("  old_rate: 1");
    println!("  new_rate: 10");
    println!("  (10-for-1 split: each share becomes 10 shares)");

    // Pagination
    println!("\n--- Pagination ---");
    println!("  let mut page_token: Option<String> = None;");
    println!("  loop {{");
    println!("      let params = CorporateActionsParams::new()");
    println!("          .limit(100)");
    println!("          .page_token(page_token.as_deref().unwrap_or(\"\"));");
    println!("      let response = client.get_corporate_actions(&params).await?;");
    println!("      // Process actions...");
    println!("      page_token = response.next_page_token;");
    println!("      if page_token.is_none() {{ break; }}");
    println!("  }}");

    // Use cases
    println!("\n--- Use Cases ---");
    println!("1. Track dividend income");
    println!("2. Adjust historical prices for splits");
    println!("3. Monitor merger/acquisition activity");
    println!("4. Calculate total shareholder return");
    println!("5. Update portfolio for corporate events");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Monitor ex-dates for dividend capture");
    println!("2. Adjust position sizes after splits");
    println!("3. Track record dates for eligibility");
    println!("4. Handle symbol changes in mergers");
    println!("5. Cache historical actions for backtesting");

    println!("\n=== Example Complete ===");
}
