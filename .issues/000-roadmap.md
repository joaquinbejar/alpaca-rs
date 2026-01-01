# Alpaca-RS Implementation Roadmap

## Overview

This document outlines the implementation roadmap for full Alpaca Markets API support in the alpaca-rs library.

## Current State

The library now provides full coverage of the Alpaca Markets API:
- **alpaca-base**: Core types, models, authentication, and error handling.
- **alpaca-http**: Comprehensive REST API client for Trading, Market Data, Broker, News, etc.
- **alpaca-websocket**: Real-time streaming for Market Data (Stocks, Crypto, News) and Trading updates.
- **alpaca-fix**: FIX Protocol support for institutional trading.

All 25 core implementation phases are complete.

## Priority Levels

- **High**: Core functionality needed for most use cases
- **Medium**: Important features for advanced use cases
- **Low**: Specialized features for specific use cases

## Implementation Phases

### Phase 1: Core Trading (High Priority)

1. [Issue 012](012-orders-advanced.md) - Advanced Order Types
2. [Issue 015](015-error-handling.md) - Comprehensive Error Handling
3. [Issue 017](017-testing-infrastructure.md) - Testing Infrastructure
4. [Issue 018](018-documentation.md) - Documentation

### Phase 2: Options & Enhanced Market Data (High Priority)

5. [Issue 001](001-options-trading-api.md) - Options Trading API
6. [Issue 005](005-market-data-stocks-enhanced.md) - Enhanced Stock Market Data
7. [Issue 007](007-websocket-streams-enhanced.md) - Enhanced WebSocket Streaming

### Phase 3: Broker API (High Priority)

8. [Issue 002](002-broker-api-accounts.md) - Broker API Accounts
9. [Issue 003](003-broker-api-funding.md) - Broker API Funding
10. [Issue 009](009-broker-api-events-sse.md) - Broker API Events (SSE)

### Phase 4: Crypto & Portfolio (Medium Priority)

11. [Issue 004](004-crypto-trading-enhanced.md) - Enhanced Crypto Trading
12. [Issue 011](011-portfolio-management.md) - Portfolio Management
13. [Issue 016](016-rate-limiting.md) - Rate Limiting

### Phase 5: Additional Features (Medium Priority)

14. [Issue 006](006-news-api.md) - News API Enhancement
15. [Issue 008](008-oauth-support.md) - OAuth Support
16. [Issue 010](010-account-activities-enhanced.md) - Enhanced Account Activities
17. [Issue 013](013-assets-enhanced.md) - Enhanced Assets API
18. [Issue 021](021-margin-short-selling.md) - Margin & Short Selling
19. [Issue 022](022-fractional-trading.md) - Fractional Trading
20. [Issue 023](023-paper-trading.md) - Paper Trading

### Phase 6: Specialized Features (Done)

21. [Issue 014](014-calendar-clock.md) - Calendar & Clock Enhancement
22. [Issue 019](019-fix-protocol.md) - FIX Protocol Support
23. [Issue 020](020-statements-confirms.md) - Statements & Confirmations
24. [Issue 024](024-local-currency-trading.md) - Local Currency Trading
25. [Issue 025](025-ira-accounts.md) - IRA Account Support

### Phase 7: Comprehensive Example Suite (Planned)

26. [Examples Roadmap](examples.md) - 66 planned examples across all crates.

## Issue Summary

| Issue | Title | Priority | Status |
|-------|-------|----------|--------|
| 001 | Options Trading API | High | Done |
| 002 | Broker API - Accounts | High | Done |
| 003 | Broker API - Funding | High | Done |
| 004 | Enhanced Crypto Trading | High | Done |
| 005 | Enhanced Stock Market Data | High | Done |
| 006 | News API Enhancement | Medium | Done |
| 007 | Enhanced WebSocket Streaming | High | Done |
| 008 | OAuth Support | Medium | Done |
| 009 | Broker API Events (SSE) | Medium | Done |
| 010 | Enhanced Account Activities | Medium | Done |
| 011 | Portfolio Management | Medium | Done |
| 012 | Advanced Order Types | High | Done |
| 013 | Enhanced Assets API | Medium | Done |
| 014 | Calendar & Clock Enhancement | Low | Done |
| 015 | Comprehensive Error Handling | High | Done |
| 016 | Rate Limiting | Medium | Done |
| 017 | Testing Infrastructure | High | Done |
| 018 | Comprehensive Documentation | High | Done |
| 019 | FIX Protocol Support | Low | Done |
| 020 | Statements & Confirmations | Low | Done |
| 021 | Margin & Short Selling | Medium | Done |
| 022 | Fractional Trading | Medium | Done |
| 023 | Paper Trading Support | Medium | Done |
| 024 | Local Currency Trading | Low | Done |
| 025 | IRA Account Support | Low | Done |
| 026 | Comprehensive Example Suite | High | Planned |

## Contributing

When implementing an issue:
1. Create a feature branch
2. Implement the feature following existing code patterns
3. Add unit tests
4. Add documentation
5. Run `make lint-fix` and `make pre-push`
6. Submit a pull request

## API Documentation Reference

The offline API documentation is available in `doc/api/docs.alpaca.markets/docs/`.
