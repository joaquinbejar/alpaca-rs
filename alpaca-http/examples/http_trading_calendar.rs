//! # Trading Calendar
//!
//! This example demonstrates how to get the trading calendar
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
//! cargo run -p alpaca-http --example http_trading_calendar
//! ```

use alpaca_base::CalendarParams;

fn main() {
    println!("=== Trading Calendar ===\n");

    // Get calendar for date range
    println!("--- Get Calendar for Date Range ---");
    let params = CalendarParams::new().start("2024-01-01").end("2024-01-31");

    println!("  Parameters:");
    println!("    Start: {:?}", params.start);
    println!("    End: {:?}", params.end);

    println!("\n  API call:");
    println!("    let calendar = client.get_calendar(&params).await?;");
    println!("    for day in calendar {{");
    println!(
        "        println!(\"{{}} - Open: {{}}, Close: {{}}\", day.date, day.open, day.close);"
    );
    println!("    }}");

    // Calendar day fields
    println!("\n--- Calendar Day Fields ---");
    println!("  date: Trading date (YYYY-MM-DD)");
    println!("  open: Market open time (HH:MM)");
    println!("  close: Market close time (HH:MM)");
    println!("  session_open: Extended hours open time");
    println!("  session_close: Extended hours close time");

    // Get current month
    println!("\n--- Get Current Month ---");
    println!("  let now = Utc::now();");
    println!("  let start = now.format(\"%Y-%m-01\").to_string();");
    println!("  let end = now.format(\"%Y-%m-31\").to_string();");
    println!("  let params = CalendarParams::new().start(&start).end(&end);");
    println!("  let calendar = client.get_calendar(&params).await?;");

    // Count trading days
    println!("\n--- Count Trading Days ---");
    println!("  let params = CalendarParams::new()");
    println!("      .start(\"2024-01-01\")");
    println!("      .end(\"2024-12-31\");");
    println!("  let calendar = client.get_calendar(&params).await?;");
    println!("  println!(\"Trading days in 2024: {{}}\", calendar.len());");

    // Find next trading day
    println!("\n--- Find Next Trading Day ---");
    println!("  let today = Utc::now().format(\"%Y-%m-%d\").to_string();");
    println!("  let params = CalendarParams::new()");
    println!("      .start(&today)");
    println!("      .end(&next_week);");
    println!("  let calendar = client.get_calendar(&params).await?;");
    println!("  if let Some(next) = calendar.first() {{");
    println!("      println!(\"Next trading day: {{}}\", next.date);");
    println!("  }}");

    // Check if specific date is trading day
    println!("\n--- Check if Date is Trading Day ---");
    println!("  let date = \"2024-07-04\"; // Independence Day");
    println!("  let params = CalendarParams::new().start(date).end(date);");
    println!("  let calendar = client.get_calendar(&params).await?;");
    println!("  if calendar.is_empty() {{");
    println!("      println!(\"{{}} is NOT a trading day\", date);");
    println!("  }} else {{");
    println!("      println!(\"{{}} IS a trading day\", date);");
    println!("  }}");

    // Early close detection
    println!("\n--- Detect Early Close Days ---");
    println!("  for day in calendar {{");
    println!("      if day.close != \"16:00\" {{");
    println!("          println!(\"Early close on {{}}: {{}}\", day.date, day.close);");
    println!("      }}");
    println!("  }}");

    // Trading hours by day type
    println!("\n--- Trading Hours by Day Type ---");
    println!("  Regular Day:");
    println!("    Open: 09:30 ET");
    println!("    Close: 16:00 ET");
    println!();
    println!("  Early Close Day:");
    println!("    Open: 09:30 ET");
    println!("    Close: 13:00 ET");

    // Use cases
    println!("\n--- Use Cases ---");
    println!("1. Schedule automated trading strategies");
    println!("2. Calculate business days for settlement");
    println!("3. Plan vacation around market closures");
    println!("4. Build trading day countdown timers");
    println!("5. Validate order scheduling");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Cache calendar data (changes infrequently)");
    println!("2. Refresh cache at start of each year");
    println!("3. Handle edge cases (early close days)");
    println!("4. Use calendar for backtesting date ranges");
    println!("5. Combine with clock for real-time status");

    println!("\n=== Example Complete ===");
}
