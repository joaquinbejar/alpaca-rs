//! # Calendar Types
//!
//! This example demonstrates market calendar and clock types.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_calendar_types
//! ```

use alpaca_base::{CalendarDay, CalendarParams, MarketClock, MarketSession};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Calendar Types ===\n");

    // 1. Market Sessions
    println!("--- Market Sessions ---");
    demonstrate_sessions();

    // 2. Calendar Day
    println!("\n--- Calendar Day ---");
    demonstrate_calendar_day()?;

    // 3. Market Clock
    println!("\n--- Market Clock ---");
    demonstrate_market_clock()?;

    // 4. Calendar Parameters
    println!("\n--- Calendar Parameters ---");
    demonstrate_calendar_params()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_sessions() {
    let sessions = [
        (MarketSession::PreMarket, "4:00 AM - 9:30 AM ET"),
        (MarketSession::Regular, "9:30 AM - 4:00 PM ET"),
        (MarketSession::AfterHours, "4:00 PM - 8:00 PM ET"),
        (MarketSession::Closed, "Market closed"),
    ];

    println!("  Market Sessions:");
    for (session, hours) in &sessions {
        let trading = if session.is_trading_allowed() {
            "✓"
        } else {
            "✗"
        };
        let regular = if session.is_regular() {
            "(regular)"
        } else {
            ""
        };
        println!("    {:?}: {} {} {}", session, hours, trading, regular);
    }
}

fn demonstrate_calendar_day() -> Result<(), Box<dyn std::error::Error>> {
    // Sample calendar day (as returned from API)
    let day = CalendarDay {
        date: "2024-01-15".to_string(),
        open: "09:30".to_string(),
        close: "16:00".to_string(),
        settlement_date: Some("2024-01-17".to_string()),
        session_open: Some("04:00".to_string()),
        session_close: Some("20:00".to_string()),
    };

    println!("  Calendar Day: {}", day.date);
    println!("    Regular Hours: {} - {}", day.open, day.close);
    println!(
        "    Extended Hours: {:?} - {:?}",
        day.session_open, day.session_close
    );

    let json = serde_json::to_string_pretty(&day)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}

fn demonstrate_market_clock() -> Result<(), Box<dyn std::error::Error>> {
    // Sample market clock (as returned from API)
    let clock_open = MarketClock {
        timestamp: "2024-01-15T14:30:00Z".to_string(),
        is_open: true,
        next_open: "2024-01-16T14:30:00Z".to_string(),
        next_close: "2024-01-15T21:00:00Z".to_string(),
    };

    println!("  Market Clock (Open):");
    println!("    Timestamp: {}", clock_open.timestamp);
    println!("    Is Open: {}", clock_open.is_open);
    println!("    Current Session: {:?}", clock_open.current_session());
    println!("    Next Close: {}", clock_open.next_close);

    let clock_closed = MarketClock {
        timestamp: "2024-01-15T22:00:00Z".to_string(),
        is_open: false,
        next_open: "2024-01-16T14:30:00Z".to_string(),
        next_close: "2024-01-16T21:00:00Z".to_string(),
    };

    println!("\n  Market Clock (Closed):");
    println!("    Timestamp: {}", clock_closed.timestamp);
    println!("    Is Open: {}", clock_closed.is_open);
    println!("    Current Session: {:?}", clock_closed.current_session());
    println!("    Next Open: {}", clock_closed.next_open);

    Ok(())
}

fn demonstrate_calendar_params() -> Result<(), Box<dyn std::error::Error>> {
    // Query calendar for a date range
    let params = CalendarParams::new().start("2024-01-01").end("2024-01-31");

    println!("  Calendar Query: January 2024");
    println!("    Start: {:?}", params.start);
    println!("    End: {:?}", params.end);

    let json = serde_json::to_string_pretty(&params)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}
