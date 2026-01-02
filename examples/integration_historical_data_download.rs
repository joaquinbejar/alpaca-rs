//! # Historical Data Download
//!
//! This example demonstrates how to download and save historical bars
//! for analysis or backtesting.
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
//! cargo run --example integration_historical_data_download
//! ```

use alpaca_base::{Credentials, Environment, Timeframe};

fn main() {
    println!("=== Historical Data Download ===\n");

    // Configuration
    println!("--- Configuration ---");
    println!("  Symbols: AAPL, MSFT, GOOGL");
    println!("  Timeframe: 1 Day");
    println!("  Date Range: 2024-01-01 to 2024-12-31");
    println!("  Output: CSV files");

    // Initialize client
    println!("\n--- Initialize Client ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;");

    // Download bars for single symbol
    println!("\n--- Download Single Symbol ---");
    println!("  let params = BarsParams::new()");
    println!("      .symbol(\"AAPL\")");
    println!("      .timeframe(Timeframe::Day)");
    println!("      .start(\"2024-01-01T00:00:00Z\")");
    println!("      .end(\"2024-12-31T23:59:59Z\")");
    println!("      .limit(10000);");
    println!("  ");
    println!("  let bars = client.get_bars(&params).await?;");
    println!("  println!(\"Downloaded {{}} bars for AAPL\", bars.len());");

    // Download bars for multiple symbols
    println!("\n--- Download Multiple Symbols ---");
    println!("  let symbols = vec![\"AAPL\", \"MSFT\", \"GOOGL\"];");
    println!("  let mut all_bars: HashMap<String, Vec<Bar>> = HashMap::new();");
    println!("  ");
    println!("  for symbol in symbols {{");
    println!("      let params = BarsParams::new()");
    println!("          .symbol(symbol)");
    println!("          .timeframe(Timeframe::Day)");
    println!("          .start(\"2024-01-01T00:00:00Z\")");
    println!("          .end(\"2024-12-31T23:59:59Z\");");
    println!("      ");
    println!("      let bars = client.get_bars(&params).await?;");
    println!("      println!(\"Downloaded {{}} bars for {{}}\", bars.len(), symbol);");
    println!("      all_bars.insert(symbol.to_string(), bars);");
    println!("  }}");

    // Handle pagination
    println!("\n--- Handle Pagination ---");
    println!("  let mut all_bars = Vec::new();");
    println!("  let mut page_token: Option<String> = None;");
    println!("  ");
    println!("  loop {{");
    println!("      let mut params = BarsParams::new()");
    println!("          .symbol(\"AAPL\")");
    println!("          .timeframe(Timeframe::Minute)");
    println!("          .limit(10000);");
    println!("      ");
    println!("      if let Some(token) = &page_token {{");
    println!("          params = params.page_token(token);");
    println!("      }}");
    println!("      ");
    println!("      let response = client.get_bars_with_pagination(&params).await?;");
    println!("      all_bars.extend(response.bars);");
    println!("      ");
    println!("      page_token = response.next_page_token;");
    println!("      if page_token.is_none() {{");
    println!("          break;");
    println!("      }}");
    println!("  }}");
    println!("  println!(\"Total bars downloaded: {{}}\", all_bars.len());");

    // Save to CSV
    println!("\n--- Save to CSV ---");
    println!("  fn save_bars_to_csv(bars: &[Bar], filename: &str) -> Result<()> {{");
    println!("      let mut writer = csv::Writer::from_path(filename)?;");
    println!("      ");
    println!("      // Write header");
    println!("      writer.write_record(&[");
    println!("          \"timestamp\", \"open\", \"high\", \"low\", \"close\", \"volume\"");
    println!("      ])?;");
    println!("      ");
    println!("      // Write data");
    println!("      for bar in bars {{");
    println!("          writer.write_record(&[");
    println!("              bar.timestamp.to_rfc3339(),");
    println!("              bar.open.to_string(),");
    println!("              bar.high.to_string(),");
    println!("              bar.low.to_string(),");
    println!("              bar.close.to_string(),");
    println!("              bar.volume.to_string(),");
    println!("          ])?;");
    println!("      }}");
    println!("      ");
    println!("      writer.flush()?;");
    println!("      Ok(())");
    println!("  }}");

    // Available timeframes
    println!("\n--- Available Timeframes ---");
    let timeframes = [
        (Timeframe::Minute, "1 minute bars"),
        (Timeframe::Hour, "1 hour bars"),
        (Timeframe::Day, "1 day bars"),
        (Timeframe::Week, "1 week bars"),
        (Timeframe::Month, "1 month bars"),
    ];

    for (tf, desc) in timeframes {
        println!("  {:?}: {}", tf, desc);
    }

    // Data quality checks
    println!("\n--- Data Quality Checks ---");
    println!("  fn validate_bars(bars: &[Bar]) -> bool {{");
    println!("      for bar in bars {{");
    println!("          // Check OHLC validity");
    println!("          if bar.high < bar.low {{");
    println!("              return false;");
    println!("          }}");
    println!("          if bar.open > bar.high || bar.open < bar.low {{");
    println!("              return false;");
    println!("          }}");
    println!("          if bar.close > bar.high || bar.close < bar.low {{");
    println!("              return false;");
    println!("          }}");
    println!("      }}");
    println!("      true");
    println!("  }}");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Use pagination for large date ranges");
    println!("2. Respect rate limits (add delays between requests)");
    println!("3. Validate downloaded data");
    println!("4. Store data with timestamps in UTC");
    println!("5. Handle market holidays (no data on closed days)");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;
    let _timeframe = Timeframe::Day;

    println!("\n=== Example Complete ===");
}
