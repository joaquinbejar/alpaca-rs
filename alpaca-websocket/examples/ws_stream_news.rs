//! # News Stream
//!
//! This example demonstrates how to subscribe to real-time news
//! using the Alpaca WebSocket API.
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
//! cargo run -p alpaca-websocket --example ws_stream_news
//! ```
//!
//! **Note**: News streaming requires appropriate API subscription.

use alpaca_websocket::StreamType;

fn main() {
    println!("=== News Stream ===\n");

    // News stream configuration
    println!("--- News Stream Configuration ---");
    let stream_type = StreamType::News;
    println!("  Stream type: {:?}", stream_type);
    println!("  Paper URL: {}", stream_type.url(true));
    println!("  Live URL: {}", stream_type.url(false));

    // News subscription format
    println!("\n--- News Subscription ---");
    println!("  To subscribe to news for specific symbols:");
    println!("  {{");
    println!("    \"action\": \"subscribe\",");
    println!("    \"news\": [\"AAPL\", \"TSLA\", \"*\"]");
    println!("  }}");
    println!();
    println!("  Use \"*\" to subscribe to all news");

    // News message format
    println!("\n--- News Message Format ---");
    println!("  {{");
    println!("    \"T\": \"n\",           // Message type: news");
    println!("    \"id\": 12345,        // News article ID");
    println!("    \"headline\": \"...\", // Article headline");
    println!("    \"summary\": \"...\",  // Article summary");
    println!("    \"author\": \"...\",   // Author name");
    println!("    \"created_at\": \"...\", // ISO timestamp");
    println!("    \"updated_at\": \"...\", // ISO timestamp");
    println!("    \"url\": \"...\",      // Article URL");
    println!("    \"content\": \"...\",  // Full content");
    println!("    \"symbols\": [\"AAPL\"], // Related symbols");
    println!("    \"source\": \"...\"    // News source");
    println!("  }}");

    // News sources
    println!("\n--- News Sources ---");
    println!("  Alpaca aggregates news from multiple sources:");
    println!("  - Benzinga");
    println!("  - Bloomberg");
    println!("  - Dow Jones");
    println!("  - And more...");

    // Use cases
    println!("\n--- News Trading Use Cases ---");
    println!("  1. Sentiment Analysis");
    println!("     - Parse headlines for positive/negative sentiment");
    println!("     - Trigger trades based on sentiment scores");
    println!();
    println!("  2. Event-Driven Trading");
    println!("     - React to earnings announcements");
    println!("     - Trade on merger/acquisition news");
    println!("     - Respond to regulatory filings");
    println!();
    println!("  3. Risk Management");
    println!("     - Monitor news for held positions");
    println!("     - Set alerts for breaking news");
    println!("     - Pause trading during high-impact events");

    // Example workflow
    println!("\n--- Example Workflow ---");
    println!("  1. Connect to news stream");
    println!("     let client = AlpacaWebSocketClient::new_news(credentials, env);");
    println!();
    println!("  2. Subscribe to symbols");
    println!("     client.subscribe_news(&[\"AAPL\", \"TSLA\"]).await?;");
    println!();
    println!("  3. Process news events");
    println!("     while let Some(news) = stream.next().await {{");
    println!("         analyze_sentiment(&news.headline);");
    println!("         if should_trade(&news) {{");
    println!("             execute_trade(&news.symbols).await;");
    println!("         }}");
    println!("     }}");

    // Filtering strategies
    println!("\n--- Filtering Strategies ---");
    println!("  Filter by symbol:");
    println!("    news.symbols.contains(&\"AAPL\")");
    println!();
    println!("  Filter by source:");
    println!("    news.source == \"Benzinga\"");
    println!();
    println!("  Filter by keywords:");
    println!("    news.headline.contains(\"earnings\")");
    println!();
    println!("  Filter by time:");
    println!("    news.created_at > market_open");

    // Rate considerations
    println!("\n--- Rate Considerations ---");
    println!("  - News volume varies by market conditions");
    println!("  - Major events can generate high volume");
    println!("  - Consider buffering for processing");
    println!("  - Implement deduplication if needed");

    println!("\n=== Example Complete ===");
}
