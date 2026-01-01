//! # Get Market Calendar
//!
//! This example demonstrates how to get the market calendar.
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
//! cargo run -p alpaca-http --example http_get_calendar
//! ```

use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;
use alpaca_http::endpoints::CalendarParams;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Get Market Calendar ===\n");

    // Create client for Paper trading
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // Get calendar for the next 10 trading days
    println!("\n--- Next 10 Trading Days ---");
    let today = Utc::now().date_naive();
    let end_date = today + Duration::days(20);

    let params = CalendarParams::new()
        .start(&today.to_string())
        .end(&end_date.to_string());

    match client.get_calendar(&params).await {
        Ok(calendar) => {
            println!("  Trading days retrieved: {}", calendar.len());

            for day in calendar.iter().take(10) {
                println!();
                println!("    Date: {}", day.date);
                println!("    Open: {}", day.open);
                println!("    Close: {}", day.close);
            }
        }
        Err(e) => {
            eprintln!("Error fetching calendar: {}", e);
        }
    }

    // Get calendar for a specific month
    println!("\n--- January 2024 Trading Days ---");
    let params = CalendarParams::new().start("2024-01-01").end("2024-01-31");

    match client.get_calendar(&params).await {
        Ok(calendar) => {
            println!("  Trading days in January 2024: {}", calendar.len());

            // Count by day of week (simplified)
            println!("\n  First 5 trading days:");
            for day in calendar.iter().take(5) {
                println!("    {} - {} to {}", day.date, day.open, day.close);
            }

            // Find any early closes
            let early_closes: Vec<_> = calendar.iter().filter(|d| d.close != "16:00").collect();

            if !early_closes.is_empty() {
                println!("\n  Early Close Days:");
                for day in &early_closes {
                    println!("    {} - closes at {}", day.date, day.close);
                }
            } else {
                println!("\n  No early close days in this period.");
            }
        }
        Err(e) => {
            eprintln!("Error fetching calendar: {}", e);
        }
    }

    // Market holidays info
    println!("\n--- Market Holiday Info ---");
    println!("  US Stock Market Holidays:");
    println!("    - New Year's Day");
    println!("    - Martin Luther King Jr. Day");
    println!("    - Presidents' Day");
    println!("    - Good Friday");
    println!("    - Memorial Day");
    println!("    - Juneteenth");
    println!("    - Independence Day");
    println!("    - Labor Day");
    println!("    - Thanksgiving Day");
    println!("    - Christmas Day");

    println!("\n=== Example Complete ===");
    Ok(())
}
