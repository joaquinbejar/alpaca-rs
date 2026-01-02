//! # WebSocket Configuration
//!
//! This example demonstrates how to configure the WebSocket client
//! with custom settings for reconnection, timeouts, and buffers.
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
//! cargo run -p alpaca-websocket --example ws_websocket_config
//! ```

use alpaca_websocket::{ConnectionState, StreamType, WebSocketConfig};

fn main() {
    println!("=== WebSocket Configuration ===\n");

    // Default configuration
    println!("--- Default Configuration ---");
    let default_config = WebSocketConfig::default();
    println!("  Reconnect enabled: {}", default_config.reconnect_enabled);
    println!(
        "  Max reconnect attempts: {}",
        default_config.reconnect_max_attempts
    );
    println!(
        "  Base reconnect delay: {}ms",
        default_config.reconnect_base_delay_ms
    );
    println!(
        "  Max reconnect delay: {}ms",
        default_config.reconnect_max_delay_ms
    );
    println!("  Ping interval: {}ms", default_config.ping_interval_ms);
    println!(
        "  Message buffer size: {}",
        default_config.message_buffer_size
    );
    println!(
        "  Connection timeout: {}ms",
        default_config.connection_timeout_ms
    );

    // Custom configuration with builder pattern
    println!("\n--- Custom Configuration (Builder Pattern) ---");
    let custom_config = WebSocketConfig::new()
        .max_reconnect_attempts(5)
        .reconnect_base_delay(500)
        .ping_interval(15000)
        .buffer_size(500)
        .connection_timeout(5000);

    println!("  Reconnect enabled: {}", custom_config.reconnect_enabled);
    println!(
        "  Max reconnect attempts: {}",
        custom_config.reconnect_max_attempts
    );
    println!(
        "  Base reconnect delay: {}ms",
        custom_config.reconnect_base_delay_ms
    );
    println!("  Ping interval: {}ms", custom_config.ping_interval_ms);
    println!(
        "  Message buffer size: {}",
        custom_config.message_buffer_size
    );
    println!(
        "  Connection timeout: {}ms",
        custom_config.connection_timeout_ms
    );

    // Disable reconnection
    println!("\n--- No Reconnection Configuration ---");
    let no_reconnect_config = WebSocketConfig::new().no_reconnect();
    println!(
        "  Reconnect enabled: {}",
        no_reconnect_config.reconnect_enabled
    );

    // Stream types and their URLs
    println!("\n--- Stream Types ---");
    let stream_types = [
        (StreamType::Stocks, "Stock market data"),
        (StreamType::Crypto, "Cryptocurrency data"),
        (StreamType::Options, "Options market data"),
        (StreamType::News, "Real-time news"),
        (StreamType::Trading, "Trading updates"),
    ];

    for (stream_type, description) in stream_types {
        println!("\n  {:?} - {}", stream_type, description);
        println!("    Paper URL: {}", stream_type.url(true));
        println!("    Live URL: {}", stream_type.url(false));
    }

    // Connection states
    println!("\n--- Connection States ---");
    let states = [
        (ConnectionState::Disconnected, "Not connected to server"),
        (
            ConnectionState::Connecting,
            "Attempting to establish connection",
        ),
        (ConnectionState::Connected, "Connected and authenticated"),
        (
            ConnectionState::Reconnecting,
            "Reconnecting after disconnect",
        ),
        (ConnectionState::Failed, "Connection failed permanently"),
    ];

    for (state, description) in states {
        println!("  {:?}: {}", state, description);
    }

    // Configuration recommendations
    println!("\n--- Configuration Recommendations ---");
    println!("  High-frequency trading:");
    println!("    - Lower ping interval (10-15 seconds)");
    println!("    - Larger buffer size (2000+)");
    println!("    - Aggressive reconnection (base delay 100-500ms)");
    println!();
    println!("  Standard trading:");
    println!("    - Default ping interval (30 seconds)");
    println!("    - Default buffer size (1000)");
    println!("    - Standard reconnection (base delay 1000ms)");
    println!();
    println!("  Low-latency requirements:");
    println!("    - Shorter connection timeout (3-5 seconds)");
    println!("    - Fewer max reconnect attempts (3-5)");
    println!("    - Consider disabling reconnection for fail-fast");

    println!("\n=== Example Complete ===");
}
