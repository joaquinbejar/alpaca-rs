//! # OAuth Configuration
//!
//! This example demonstrates OAuth 2.0 configuration for Alpaca API.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_oauth_config
//! ```

use alpaca_base::{OAuthConfig, OAuthScope, OAuthToken};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OAuth 2.0 Configuration ===\n");

    // 1. OAuth Scopes
    println!("--- OAuth Scopes ---");
    demonstrate_scopes();

    // 2. OAuth Config
    println!("\n--- OAuth Config ---");
    demonstrate_config();

    // 3. OAuth Token
    println!("\n--- OAuth Token ---");
    demonstrate_token();

    // 4. Authorization URL
    println!("\n--- Authorization Flow ---");
    demonstrate_auth_flow();

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_scopes() {
    let scopes = [
        (OAuthScope::AccountWrite, "Read/write account information"),
        (OAuthScope::Trading, "Place and manage orders"),
        (OAuthScope::Data, "Access market data"),
    ];

    println!("  Available Scopes:");
    for (scope, description) in &scopes {
        println!("    {} - {}", scope, description);
    }
}

fn demonstrate_config() {
    let config = OAuthConfig::new(
        "your-client-id",
        "your-client-secret",
        "https://yourapp.com/callback",
    )
    .scope(OAuthScope::Trading)
    .scope(OAuthScope::Data);

    println!("  OAuth Config:");
    println!("    Client ID: {}", config.client_id);
    println!("    Redirect URI: {}", config.redirect_uri);
    println!("    Scopes: {:?}", config.scopes);

    // Build authorization URL
    let auth_url = config.authorization_url("random_state_string");
    println!("\n  Authorization URL:");
    println!("    {}", auth_url);
}

fn demonstrate_token() {
    // Simulated token response
    let token = OAuthToken {
        access_token: "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...".to_string(),
        refresh_token: Some("refresh_token_here".to_string()),
        token_type: "Bearer".to_string(),
        expires_in: Some(3600),
        scope: Some("trading data".to_string()),
    };

    println!("  Token Response:");
    println!("    Token Type: {}", token.token_type);
    println!("    Expires In: {:?} seconds", token.expires_in);
    println!("    Has Refresh Token: {}", token.has_refresh_token());
    println!("    Scope: {:?}", token.scope);

    // Generate auth header
    let header = token.auth_header();
    println!("\n  Authorization Header:");
    println!("    {}", header);
}

fn demonstrate_auth_flow() {
    println!("  OAuth 2.0 Authorization Code Flow:");
    println!();
    println!("    1. Redirect user to authorization URL");
    println!("       GET https://app.alpaca.markets/oauth/authorize");
    println!("       ?response_type=code");
    println!("       &client_id=YOUR_CLIENT_ID");
    println!("       &redirect_uri=YOUR_REDIRECT_URI");
    println!("       &scope=trading+data");
    println!();
    println!("    2. User authorizes your app");
    println!();
    println!("    3. Alpaca redirects to your callback with code");
    println!("       GET YOUR_REDIRECT_URI?code=AUTHORIZATION_CODE");
    println!();
    println!("    4. Exchange code for access token");
    println!("       POST https://api.alpaca.markets/oauth/token");
    println!("       grant_type=authorization_code");
    println!("       code=AUTHORIZATION_CODE");
    println!("       client_id=YOUR_CLIENT_ID");
    println!("       client_secret=YOUR_CLIENT_SECRET");
    println!("       redirect_uri=YOUR_REDIRECT_URI");
    println!();
    println!("    5. Use access token for API requests");
    println!("       Authorization: Bearer ACCESS_TOKEN");
}
