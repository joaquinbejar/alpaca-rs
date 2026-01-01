# Examples Roadmap

This document outlines all examples to be created for each crate in the `alpaca-rs` workspace.

## Priority Legend

- **P1**: Essential - Core functionality that most users need
- **P2**: Important - Common use cases
- **P3**: Advanced - Specialized features

---

## alpaca-base Examples

### P1 - Core Types

| Example | Description | Types/Features Tested |
|---------|-------------|----------------------|
| `credentials_from_env.rs` | Load credentials from environment variables | `Credentials`, `Environment` |
| `error_handling.rs` | Demonstrate error types and handling patterns | `AlpacaError`, `ApiErrorCode`, `is_retryable()` |
| `order_types.rs` | Create different order type configurations | `OrderType`, `OrderSide`, `TimeInForce`, `OrderClass` |

### P2 - Market Data Types

| Example | Description | Types/Features Tested |
|---------|-------------|----------------------|
| `bar_params_builder.rs` | Build market data query parameters | `MultiBarsParams`, `CryptoBarsParams`, `OptionBarsParams` |
| `asset_filtering.rs` | Filter and query assets | `Asset`, `AssetClass`, `AssetStatus`, `ListAssetsParams` |

### P3 - Advanced Types

| Example | Description | Types/Features Tested |
|---------|-------------|----------------------|
| `bracket_order_config.rs` | Configure bracket orders with take-profit and stop-loss | `TakeProfit`, `StopLoss`, `OrderClass::Bracket` |
| `option_contract_params.rs` | Build options contract queries | `OptionContractParams`, `OptionType`, `OptionStyle` |
| `broker_account_types.rs` | Demonstrate broker account creation types | `Contact`, `Identity`, `Disclosures`, `Agreement` |
| `oauth_config.rs` | Configure OAuth 2.0 authentication | `OAuthConfig`, `OAuthScope`, `OAuthToken` |
| `rate_limit_config.rs` | Configure rate limiting behavior | `RateLimitConfig`, `RateLimitStatus` |
| `calendar_types.rs` | Work with market calendar types | `MarketClock`, `CalendarDay`, `MarketSession`, `TradingDay` |
| `currency_conversion.rs` | Currency conversion utilities | `Currency`, `ExchangeRate`, `CurrencyPair` |
| `ira_types.rs` | IRA account type configurations | `IraAccountType`, `IraContribution`, `IraDistribution` |
| `fix_protocol_config.rs` | FIX protocol session configuration | `FixVersion`, `FixSessionConfig`, `FixMsgType` |

### P3 - Test Utilities

| Example | Description | Types/Features Tested |
|---------|-------------|----------------------|
| `test_fixtures.rs` | Using test fixtures for unit tests | `test_utils::fixtures`, `sample_account()`, `sample_order()` |
| `test_assertions.rs` | Using assertion helpers | `test_utils::assertions`, `assert_account_active()` |

---

## alpaca-http Examples

### P1 - Account & Trading

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `get_account.rs` | Retrieve account information | `get_account()` |
| `create_market_order.rs` | Place a simple market order | `create_order()` with market order |
| `create_limit_order.rs` | Place a limit order | `create_order()` with limit order |
| `list_orders.rs` | List and filter orders | `list_orders()`, `OrderParams` |
| `cancel_order.rs` | Cancel an existing order | `cancel_order()`, `cancel_all_orders()` |
| `get_positions.rs` | List all positions | `list_positions()`, `get_position()` |
| `close_position.rs` | Close a position | `close_position()`, `close_all_positions()` |

### P2 - Advanced Orders

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `bracket_order.rs` | Create bracket order with take-profit and stop-loss | `create_order()` with `OrderClass::Bracket` |
| `oco_order.rs` | Create One-Cancels-Other order | `create_order()` with `OrderClass::Oco` |
| `oto_order.rs` | Create One-Triggers-Other order | `create_order()` with `OrderClass::Oto` |
| `trailing_stop_order.rs` | Create trailing stop orders | `create_order()` with trailing stop |
| `replace_order.rs` | Modify an existing order | `replace_order()`, `ReplaceOrderRequest` |
| `notional_order.rs` | Place order by dollar amount | `create_order()` with notional amount |

### P2 - Market Data

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `get_stock_bars.rs` | Fetch historical stock bars | `get_bars()`, `get_multi_bars()` |
| `get_latest_quote.rs` | Get real-time quote | `get_latest_quote()`, `get_latest_trade()` |
| `get_crypto_bars.rs` | Fetch cryptocurrency data | `get_crypto_bars()` |
| `get_trades.rs` | Fetch historical trades | `get_trades()` |
| `get_quotes.rs` | Fetch historical quotes | `get_quotes()` |

### P2 - Assets

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `list_assets.rs` | List and filter tradeable assets | `list_assets()`, `ListAssetsParams` |
| `get_asset.rs` | Get specific asset details | `get_asset()` |

### P2 - Account Activities

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `list_activities.rs` | List account activities | `list_activities()`, `ListActivitiesParams` |
| `portfolio_history.rs` | Get portfolio performance history | `get_portfolio_history()`, `PortfolioHistoryParams` |

### P3 - Options Trading

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `list_option_contracts.rs` | Query options contracts | `list_option_contracts()`, `OptionContractParams` |
| `get_option_contract.rs` | Get specific option contract | `get_option_contract()` |
| `option_market_data.rs` | Fetch options market data | `get_option_bars()` |

### P3 - Broker API

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `create_broker_account.rs` | Create a new broker account | `create_broker_account()` |
| `get_broker_account.rs` | Retrieve broker account details | `get_broker_account()`, `list_broker_accounts()` |
| `ach_relationships.rs` | Manage ACH funding relationships | `create_ach_relationship()`, `list_ach_relationships()` |
| `transfers.rs` | Create and manage transfers | `create_transfer()`, `list_transfers()` |
| `journals.rs` | Create journal entries | `create_journal()`, `list_journals()` |
| `documents.rs` | List and retrieve documents | `list_documents()`, `get_document()` |

### P3 - Calendar & Clock

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `market_clock.rs` | Check market open/close status | `get_clock()` |
| `trading_calendar.rs` | Get trading calendar | `get_calendar()`, `CalendarParams` |

### P3 - Corporate Actions

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `corporate_actions.rs` | List corporate actions | `list_corporate_actions()`, `CorporateActionsParams` |

### P3 - Local Currency Trading

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `exchange_rates.rs` | Get FX exchange rates | `get_exchange_rates()`, `get_exchange_rate()` |

### P3 - IRA Accounts

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `ira_contributions.rs` | Manage IRA contributions | `list_ira_contributions()`, `create_ira_contribution()` |
| `ira_distributions.rs` | List IRA distributions | `list_ira_distributions()` |
| `ira_beneficiaries.rs` | Manage IRA beneficiaries | `list_ira_beneficiaries()` |

### P3 - Watchlists

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `watchlists.rs` | Create and manage watchlists | `create_watchlist()`, `list_watchlists()`, `add_to_watchlist()` |

### P3 - News

| Example | Description | Endpoints Tested |
|---------|-------------|-----------------|
| `news.rs` | Fetch market news | `get_news()`, `NewsParams` |

---

## alpaca-fix Examples

### P1 - Connection & Session

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `fix_connect.rs` | Connect to FIX server and establish session | `FixClient::connect()`, `FixConfig`, logon handshake |
| `fix_disconnect.rs` | Graceful disconnect with logout | `FixClient::disconnect()`, logout confirmation |

### P1 - Order Routing

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `fix_market_order.rs` | Send market order via FIX | `NewOrderSingle::market()`, `send_order()` |
| `fix_limit_order.rs` | Send limit order via FIX | `NewOrderSingle::limit()`, `send_order()` |
| `fix_cancel_order.rs` | Cancel order via FIX | `OrderCancelRequest`, `cancel_order()` |

### P2 - Order Management

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `fix_replace_order.rs` | Modify order via FIX | `OrderCancelReplaceRequest`, `replace_order()` |
| `fix_stop_order.rs` | Send stop order via FIX | `NewOrderSingle::stop()`, stop price handling |
| `fix_execution_reports.rs` | Process execution reports | `ExecutionReport`, `parse_execution_report()` |

### P2 - Message Handling

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `fix_message_loop.rs` | Receive and process messages | `next_message()`, message channel |
| `fix_heartbeat.rs` | Heartbeat and TestRequest handling | Background heartbeat, automatic responses |

### P3 - Market Data

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `fix_market_data_request.rs` | Request market data via FIX | `MarketDataRequest::subscribe()`, `request_market_data()` |
| `fix_market_data_snapshot.rs` | Process market data snapshots | `MarketDataSnapshot`, `MarketDataEntry` |

### P3 - Advanced Session

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `fix_session_recovery.rs` | Handle sequence number gaps | `ResendRequest`, sequence validation |
| `fix_config_builder.rs` | Configure FIX session options | `FixConfig::builder()`, all config options |
| `fix_message_encoding.rs` | Encode/decode FIX messages | `FixEncoder`, `FixDecoder`, checksum validation |

---

## alpaca-websocket Examples

### P1 - Market Data Streaming

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `stream_trades.rs` | Subscribe to real-time trades | `StreamType::MarketData`, `subscribe_trades()` |
| `stream_quotes.rs` | Subscribe to real-time quotes | `StreamType::MarketData`, `subscribe_quotes()` |
| `stream_bars.rs` | Subscribe to real-time bars | `StreamType::MarketData`, `subscribe_bars()` |

### P1 - Trading Updates

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `trade_updates.rs` | Listen for order execution events | `StreamType::Trading`, `TradeUpdateEvent` |

### P2 - Crypto Streaming

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `stream_crypto.rs` | Subscribe to crypto market data | `StreamType::CryptoData` |

### P2 - Configuration

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `websocket_config.rs` | Configure WebSocket client | `WebSocketConfig`, builder pattern |
| `reconnection.rs` | Handle disconnections and reconnection | `reconnect_enabled`, `reconnect_max_attempts` |

### P3 - News Streaming

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `stream_news.rs` | Subscribe to real-time news | `StreamType::News` |

### P3 - Advanced Subscriptions

| Example | Description | Features Tested |
|---------|-------------|-----------------|
| `subscription_management.rs` | Dynamic subscription changes | `Subscription` builder, `subscribe()`, `unsubscribe()` |
| `multiple_symbols.rs` | Subscribe to multiple symbols | Batch subscriptions |

---

## Integration Examples (Cross-Crate)

### P1 - Complete Workflows

| Example | Description | Crates Used |
|---------|-------------|-------------|
| `paper_trading_bot.rs` | Simple trading bot with paper account | `alpaca-http`, `alpaca-base` |
| `realtime_order_tracker.rs` | Place order and track via WebSocket | `alpaca-http`, `alpaca-websocket`, `alpaca-base` |

### P2 - Data Collection

| Example | Description | Crates Used |
|---------|-------------|-------------|
| `historical_data_download.rs` | Download and save historical bars | `alpaca-http`, `alpaca-base` |
| `realtime_data_logger.rs` | Log streaming data to file | `alpaca-websocket`, `alpaca-base` |

### P3 - Advanced Strategies

| Example | Description | Crates Used |
|---------|-------------|-------------|
| `bracket_order_with_updates.rs` | Place bracket order and monitor legs | `alpaca-http`, `alpaca-websocket`, `alpaca-base` |
| `portfolio_rebalancer.rs` | Rebalance portfolio to target allocations | `alpaca-http`, `alpaca-base` |
| `options_scanner.rs` | Scan and filter options contracts | `alpaca-http`, `alpaca-base` |

---

## Example Structure Guidelines

Each example should follow this structure:

```rust
//! # Example Title
//!
//! Brief description of what this example demonstrates.
//!
//! ## Prerequisites
//! - Environment variables: ALPACA_API_KEY, ALPACA_API_SECRET
//! - Paper trading account (for trading examples)
//!
//! ## Usage
//! ```bash
//! cargo run --example example_name
//! ```

use alpaca_http::AlpacaHttpClient;
use alpaca_base::{Credentials, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials
    let credentials = Credentials::from_env()?;
    
    // Create client
    let client = AlpacaHttpClient::new(credentials, Environment::Paper)?;
    
    // Example logic here
    
    Ok(())
}
```

## Implementation Notes

1. **Error Handling**: All examples should demonstrate proper error handling
2. **Environment**: Trading examples should use Paper environment by default
3. **Documentation**: Each example should have comprehensive doc comments
4. **Logging**: Use `tracing` or `log` for output where appropriate
5. **Cleanup**: Examples that create resources should clean them up

## Total Examples Summary

| Crate | P1 | P2 | P3 | Total |
|-------|----|----|----|----|
| alpaca-base | 3 | 2 | 10 | 15 |
| alpaca-http | 7 | 12 | 15 | 34 |
| alpaca-fix | 5 | 5 | 5 | 15 |
| alpaca-websocket | 4 | 3 | 3 | 10 |
| Integration | 2 | 2 | 3 | 7 |
| **Total** | **21** | **24** | **36** | **81** |
