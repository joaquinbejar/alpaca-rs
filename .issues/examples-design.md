# Alpaca-RS Examples Implementation Design

This document defines the standards, naming conventions, and workflow for implementing the comprehensive example suite (Phase 7) for the `alpaca-rs` workspace.

## Context

- **Roadmap Phase**: Phase 7: Comprehensive Example Suite
- **Source Reference**: `.issues/examples.md` (List of all 81 planned examples)
- **Goal**: Provide high-quality, documented, and runnable examples for every feature of the library.

## Directory Structure

All examples MUST be placed in the root `examples/` directory using the following naming convention to avoid conflicts across crates:

```bash
examples/
├── <crate-short-name>_<example_name>.rs
└── integration_<example_name>.rs
```

**Short Names:**
- `base`: alpaca-base
- `http`: alpaca-http
- `ws`: alpaca-websocket
- `fix`: alpaca-fix

**Example:** `http_create_market_order.rs`

## GitHub Issue Strategy

To manage 81 examples efficiently, we group them into **12 logic batches**. Each batch corresponds to one GitHub issue.

### Issue Labels
- `type:example` (New label)
- `priority:high/medium/low` (Aligned with P1/P2/P3 in roadmap)
- `phase:7`
- `crate:base`, `crate:http`, `crate:ws`, `crate:fix`, `crate:integration`

### Batch Definitions

| Issue # | Title | Priority | Examples Included | Labels |
| :--- | :--- | :--- | :--- | :--- |
| E001 | Base: Core & Market Data | High | `base_credentials_from_env`, `base_error_handling`, `base_order_types`, `base_bar_params_builder`, `base_asset_filtering` | `crate:base`, `priority:high` |
| E002 | Base: Specialized & Utilities | Low | `base_bracket_order_config`, `base_broker_account_types`, `base_currency_conversion`, `base_ira_types`, etc. | `crate:base`, `priority:low` |
| E003 | HTTP: Core Trading | High | `http_get_account`, `http_create_market_order`, `http_create_limit_order`, `http_get_positions`, `http_close_position`, etc. | `crate:http`, `priority:high` |
| E004 | HTTP: Advanced Orders | Medium | `http_bracket_order`, `http_oco_order`, `http_oto_order`, `http_trailing_stop_order`, `http_notional_order` | `crate:http`, `priority:medium` |
| E005 | HTTP: Market Data & Assets | Medium | `http_get_stock_bars`, `http_get_latest_quote`, `http_get_crypto_bars`, `http_list_assets`, `http_get_asset` | `crate:http`, `priority:medium` |
| E006 | HTTP: Account & Portfolio | Medium | `http_list_activities`, `http_portfolio_history`, `http_watchlists` | `crate:http`, `priority:medium` |
| E007 | HTTP: Options Trading | Low | `http_list_option_contracts`, `http_get_option_contract`, `http_option_market_data` | `crate:http`, `priority:low` |
| E008 | HTTP: Broker API | Low | `http_create_broker_account`, `http_ach_relationships`, `http_transfers`, `http_journals`, `http_documents`, `http_ira_contributions` | `crate:http`, `priority:low` |
| E009 | HTTP: Specialty (LCT, News, Calendar) | Low | `http_news`, `http_exchange_rates`, `http_market_clock`, `http_trading_calendar` | `crate:http`, `priority:low` |
| E010 | FIX: Connection & Trading | High | `fix_connect`, `fix_market_order`, `fix_limit_order`, `fix_execution_reports`, `fix_heartbeat` | `crate:fix`, `priority:high` |
| E011 | WebSocket: Streaming & Updates | High | `ws_stream_trades`, `ws_stream_quotes`, `ws_trade_updates`, `ws_reconnection`, `ws_stream_crypto` | `crate:ws`, `priority:high` |
| E012 | Integration: Bott & Strategies | Medium | `paper_trading_bot`, `realtime_order_tracker`, `historical_data_download`, `portfolio_rebalancer` | `crate:integration`, `priority:medium` |

## Implementation Workflow

> [!IMPORTANT]
> **A dedicated feature branch is MANDATORY for every batch/change.**
> Never implement examples directly on the `main` branch.

Follow these steps for each batch:

### 1. Preparation
- Create the GitHub issue using `gh issue create`.
- Base body on the corresponding table in `.issues/examples.md`.
- Create a feature branch: `feat/examples-E0XX`.

### 2. Implementation Standards
Each example MUST:
- **Compile and Run**: Use `cargo run --example <name>`.
- **Environment Aware**: Use `Credentials::from_env()` where applicable.
- **Documented**: Include a module-level doc comment explaining:
    - What the example does.
    - Prerequisites (API keys, environment).
    - Expected output.
- **Robust**: Handle errors using `Result<(), Box<dyn Error>>` in `main`.

### 3. README Updates
- For every batch implemented, update the `README.md` of the affected crate(s) to include the new examples in the `Examples` section.

### 4. Validation
```bash
# Verify the example compiles and runs (requires ALPACA_API_KEY/SECRET)
cargo run --example <name>
# Run linter and tests locally
make lint-fix pre-push
```

**MANDATORY**: GitHub Actions MUST pass for the PR before it can be merged.

### 5. PR & Merge
- Create a single PR for the entire batch.
- PR title: `feat(examples): implement <Batch Title> (E0XX)`
- Body: `Closes #<GH-ISSUE-ID>`

## Definition of "Done" for Phase 7
- 81 examples implemented and placed in `examples/`.
- All examples are documented in their respective crate's `README.md`.
- All examples pass GitHub Actions (CI) checks, including `makefile-validation` and `semver`.
- All 12 examples-related GitHub issues are closed.
