//! # Market Clock
//!
//! This example demonstrates how to check market open/close status
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
//! cargo run -p alpaca-http --example http_market_clock
//! ```

fn main() {
    println!("=== Market Clock ===\n");

    // Get market clock
    println!("--- Get Market Clock ---");
    println!("  let clock = client.get_clock().await?;");
    println!("  println!(\"Market is open: {{}}\", clock.is_open);");
    println!("  println!(\"Next open: {{}}\", clock.next_open);");
    println!("  println!(\"Next close: {{}}\", clock.next_close);");

    // Clock fields
    println!("\n--- Clock Fields ---");
    println!("  timestamp: Current server time (UTC)");
    println!("  is_open: Whether the market is currently open");
    println!("  next_open: Next market open time (UTC)");
    println!("  next_close: Next market close time (UTC)");

    // Market hours
    println!("\n--- US Market Hours (Eastern Time) ---");
    println!("  Pre-market:    4:00 AM - 9:30 AM ET");
    println!("  Regular:       9:30 AM - 4:00 PM ET");
    println!("  After-hours:   4:00 PM - 8:00 PM ET");

    // Market sessions
    println!("\n--- Market Sessions ---");
    println!("  PRE_MARKET: Extended hours before regular session");
    println!("  REGULAR: Standard trading hours");
    println!("  AFTER_HOURS: Extended hours after regular session");
    println!("  CLOSED: Market is closed");

    // Check if market is open
    println!("\n--- Check Market Status ---");
    println!("  if clock.is_open {{");
    println!("      println!(\"Market is OPEN - trading allowed\");");
    println!("      println!(\"Closes at: {{}}\", clock.next_close);");
    println!("  }} else {{");
    println!("      println!(\"Market is CLOSED\");");
    println!("      println!(\"Opens at: {{}}\", clock.next_open);");
    println!("  }}");

    // Time until open/close
    println!("\n--- Calculate Time Until Open/Close ---");
    println!("  let now = Utc::now();");
    println!("  if clock.is_open {{");
    println!("      let until_close = clock.next_close - now;");
    println!("      println!(\"Time until close: {{}} minutes\", until_close.num_minutes());");
    println!("  }} else {{");
    println!("      let until_open = clock.next_open - now;");
    println!("      println!(\"Time until open: {{}} minutes\", until_open.num_minutes());");
    println!("  }}");

    // Market holidays
    println!("\n--- Market Holidays (US) ---");
    println!("  New Year's Day");
    println!("  Martin Luther King Jr. Day");
    println!("  Presidents' Day");
    println!("  Good Friday");
    println!("  Memorial Day");
    println!("  Juneteenth");
    println!("  Independence Day");
    println!("  Labor Day");
    println!("  Thanksgiving Day");
    println!("  Christmas Day");

    // Early close days
    println!("\n--- Early Close Days ---");
    println!("  Day before Independence Day (if weekday)");
    println!("  Day after Thanksgiving");
    println!("  Christmas Eve (if weekday)");
    println!("  Market closes at 1:00 PM ET on these days");

    // Use cases
    println!("\n--- Use Cases ---");
    println!("1. Schedule order submissions");
    println!("2. Display market status in UI");
    println!("3. Trigger alerts before market open/close");
    println!("4. Prevent orders during closed hours");
    println!("5. Plan trading sessions");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Cache clock data with short TTL (1-5 minutes)");
    println!("2. Use server timestamp, not local time");
    println!("3. Handle timezone conversions carefully");
    println!("4. Check clock before submitting orders");
    println!("5. Account for extended hours trading");

    println!("\n=== Example Complete ===");
}
