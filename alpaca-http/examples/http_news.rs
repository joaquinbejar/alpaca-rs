//! # Market News
//!
//! This example demonstrates how to fetch market news
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
//! cargo run -p alpaca-http --example http_news
//! ```

use alpaca_base::NewsParams;

fn main() {
    println!("=== Market News ===\n");

    // Basic news request
    println!("--- Basic News Request ---");
    println!("  let params = NewsParams::new().limit(10);");
    println!("  let response = client.get_enhanced_news(&params).await?;");
    println!("  for article in response.news {{");
    println!("      println!(\"{{}} - {{}}\", article.headline, article.source);");
    println!("  }}");

    // News for specific symbols
    println!("\n--- News for Specific Symbols ---");
    let params = NewsParams::new()
        .symbols("AAPL,MSFT,GOOGL")
        .limit(5)
        .sort_desc();

    println!("  Filter parameters:");
    println!("    Symbols: {:?}", params.symbols);
    println!("    Limit: {:?}", params.limit);
    println!("    Sort: {:?}", params.sort);

    println!("\n  API call:");
    println!("    let response = client.get_enhanced_news_for_symbols(\"AAPL,MSFT\", 10).await?;");

    // News with content
    println!("\n--- News with Full Content ---");
    let params_with_content = NewsParams::new()
        .symbols("TSLA")
        .with_content()
        .exclude_empty()
        .limit(3);

    println!("  Parameters:");
    println!(
        "    Include content: {:?}",
        params_with_content.include_content
    );
    println!(
        "    Exclude contentless: {:?}",
        params_with_content.exclude_contentless
    );

    // News by time range
    println!("\n--- News by Time Range ---");
    let params_time = NewsParams::new()
        .time_range("2024-01-01T00:00:00Z", "2024-01-31T23:59:59Z")
        .sort_asc();

    println!("  Time range:");
    println!("    Start: {:?}", params_time.start);
    println!("    End: {:?}", params_time.end);
    println!("    Sort: {:?}", params_time.sort);

    // Latest news shortcut
    println!("\n--- Latest News Shortcut ---");
    println!("  let response = client.get_latest_enhanced_news(20).await?;");

    // News article fields
    println!("\n--- News Article Fields ---");
    println!("  id: Unique article identifier");
    println!("  headline: Article headline");
    println!("  author: Article author");
    println!("  source: News source (e.g., Benzinga, Reuters)");
    println!("  summary: Brief summary");
    println!("  content: Full article content (if requested)");
    println!("  url: Link to original article");
    println!("  symbols: Related stock symbols");
    println!("  created_at: Publication timestamp");
    println!("  updated_at: Last update timestamp");
    println!("  images: Associated images");

    // Pagination
    println!("\n--- Pagination ---");
    println!("  let mut page_token: Option<String> = None;");
    println!("  loop {{");
    println!("      let params = NewsParams::new()");
    println!("          .limit(50)");
    println!("          .page_token(page_token.as_deref().unwrap_or(\"\"));");
    println!("      let response = client.get_enhanced_news(&params).await?;");
    println!("      // Process articles...");
    println!("      page_token = response.next_page_token;");
    println!("      if page_token.is_none() {{ break; }}");
    println!("  }}");

    // News sources
    println!("\n--- Common News Sources ---");
    println!("  Benzinga: Real-time market news");
    println!("  Reuters: Global news agency");
    println!("  Bloomberg: Financial news");
    println!("  Dow Jones: Business news");
    println!("  PR Newswire: Press releases");

    // Use cases
    println!("\n--- Use Cases ---");
    println!("1. Sentiment analysis on news headlines");
    println!("2. Event-driven trading strategies");
    println!("3. Portfolio monitoring alerts");
    println!("4. Research and due diligence");
    println!("5. News aggregation dashboards");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Cache news to reduce API calls");
    println!("2. Use symbol filters for relevant news");
    println!("3. Implement rate limiting");
    println!("4. Parse timestamps consistently");
    println!("5. Handle missing content gracefully");

    println!("\n=== Example Complete ===");
}
