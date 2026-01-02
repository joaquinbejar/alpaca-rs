//! # Realtime Data Logger
//!
//! This example demonstrates how to log streaming market data to a file
//! for later analysis.
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
//! cargo run --example integration_realtime_data_logger
//! ```

use alpaca_base::{Credentials, Environment};

fn main() {
    println!("=== Realtime Data Logger ===\n");

    // Configuration
    println!("--- Configuration ---");
    println!("  Symbols: AAPL, MSFT, SPY");
    println!("  Data Types: Trades, Quotes, Bars");
    println!("  Output: data_log.jsonl");
    println!("  Format: JSON Lines (one JSON object per line)");

    // Initialize WebSocket client
    println!("\n--- Initialize WebSocket Client ---");
    println!("  let credentials = Credentials::from_env()?;");
    println!("  let client = AlpacaWebSocketClient::from_env(Environment::Paper)?;");

    // Create log file
    println!("\n--- Create Log File ---");
    println!("  let file = OpenOptions::new()");
    println!("      .create(true)");
    println!("      .append(true)");
    println!("      .open(\"data_log.jsonl\")?;");
    println!("  let mut writer = BufWriter::new(file);");

    // Subscribe to market data
    println!("\n--- Subscribe to Market Data ---");
    println!("  let symbols = vec![\"AAPL\", \"MSFT\", \"SPY\"];");
    println!("  let mut stream = client.subscribe_market_data(");
    println!("      &symbols,");
    println!("      true,  // trades");
    println!("      true,  // quotes");
    println!("      true,  // bars");
    println!("  ).await?;");

    // Log entry structure
    println!("\n--- Log Entry Structure ---");
    println!("  #[derive(Serialize)]");
    println!("  struct LogEntry {{");
    println!("      timestamp: DateTime<Utc>,");
    println!("      data_type: String,");
    println!("      symbol: String,");
    println!("      data: serde_json::Value,");
    println!("  }}");

    // Main logging loop
    println!("\n--- Main Logging Loop ---");
    println!("  while let Some(msg) = stream.next().await {{");
    println!("      match msg {{");
    println!("          Ok(WebSocketMessage::Trade(trade)) => {{");
    println!("              let entry = LogEntry {{");
    println!("                  timestamp: Utc::now(),");
    println!("                  data_type: \"trade\".to_string(),");
    println!("                  symbol: trade.symbol.clone(),");
    println!("                  data: serde_json::to_value(&trade)?,");
    println!("              }};");
    println!("              writeln!(writer, \"{{}}\", serde_json::to_string(&entry)?)?;");
    println!("          }}");
    println!("          Ok(WebSocketMessage::Quote(quote)) => {{");
    println!("              let entry = LogEntry {{");
    println!("                  timestamp: Utc::now(),");
    println!("                  data_type: \"quote\".to_string(),");
    println!("                  symbol: quote.symbol.clone(),");
    println!("                  data: serde_json::to_value(&quote)?,");
    println!("              }};");
    println!("              writeln!(writer, \"{{}}\", serde_json::to_string(&entry)?)?;");
    println!("          }}");
    println!("          Ok(WebSocketMessage::Bar(bar)) => {{");
    println!("              let entry = LogEntry {{");
    println!("                  timestamp: Utc::now(),");
    println!("                  data_type: \"bar\".to_string(),");
    println!("                  symbol: bar.symbol.clone(),");
    println!("                  data: serde_json::to_value(&bar)?,");
    println!("              }};");
    println!("              writeln!(writer, \"{{}}\", serde_json::to_string(&entry)?)?;");
    println!("          }}");
    println!("          Err(e) => eprintln!(\"Error: {{}}\", e),");
    println!("          _ => {{}}");
    println!("      }}");
    println!("      ");
    println!("      // Flush periodically");
    println!("      writer.flush()?;");
    println!("  }}");

    // File rotation
    println!("\n--- File Rotation ---");
    println!("  fn rotate_log_file(base_name: &str) -> Result<File> {{");
    println!("      let timestamp = Utc::now().format(\"%Y%m%d_%H%M%S\");");
    println!("      let filename = format!(\"{{}}_{{}}.jsonl\", base_name, timestamp);");
    println!("      OpenOptions::new()");
    println!("          .create(true)");
    println!("          .write(true)");
    println!("          .open(filename)");
    println!("  }}");
    println!("  ");
    println!("  // Rotate every hour or when file exceeds size");
    println!("  if log_size > MAX_FILE_SIZE || should_rotate_hourly() {{");
    println!("      writer.flush()?;");
    println!("      file = rotate_log_file(\"data_log\")?;");
    println!("      writer = BufWriter::new(file);");
    println!("  }}");

    // Statistics tracking
    println!("\n--- Statistics Tracking ---");
    println!("  struct Stats {{");
    println!("      trades_logged: u64,");
    println!("      quotes_logged: u64,");
    println!("      bars_logged: u64,");
    println!("      bytes_written: u64,");
    println!("      start_time: Instant,");
    println!("  }}");
    println!("  ");
    println!("  // Print stats periodically");
    println!("  println!(\"Logged: {{}} trades, {{}} quotes, {{}} bars\",");
    println!("      stats.trades_logged, stats.quotes_logged, stats.bars_logged);");
    println!("  println!(\"Rate: {{:.2}} msgs/sec\", messages_per_second);");

    // Graceful shutdown
    println!("\n--- Graceful Shutdown ---");
    println!("  // Handle Ctrl+C");
    println!("  tokio::select! {{");
    println!("      _ = log_data(&mut stream, &mut writer) => {{}}");
    println!("      _ = tokio::signal::ctrl_c() => {{");
    println!("          println!(\"Shutting down...\");");
    println!("          writer.flush()?;");
    println!("      }}");
    println!("  }}");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Use buffered writes for performance");
    println!("2. Flush periodically to prevent data loss");
    println!("3. Implement file rotation for long-running loggers");
    println!("4. Use JSON Lines format for easy parsing");
    println!("5. Include timestamps in log entries");

    // Display example values
    let _credentials = Credentials::paper("demo_key", "demo_secret");
    let _environment = Environment::Paper;

    println!("\n=== Example Complete ===");
}
