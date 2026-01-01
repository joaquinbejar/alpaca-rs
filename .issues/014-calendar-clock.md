# Issue 014: Calendar and Clock API Enhancement

## Priority: Low

## Description

Enhance calendar and clock API with full market hours information and trading day utilities.

## Current State

- Basic calendar and clock endpoints exist
- Limited session information

## Required Features

### Types (`alpaca-base`)

- [ ] `Calendar` enhancement:
  - settlement_date
  - session_open (pre-market)
  - session_close (after-hours)
- [ ] `Clock` enhancement:
  - is_open
  - next_open
  - next_close
- [ ] `MarketSession` enum: pre_market, regular, after_hours, closed
- [ ] `TradingDay` helper struct with utility methods

### HTTP Endpoints (`alpaca-http`)

- [ ] `GET /v2/calendar` - Market calendar with date range
- [ ] `GET /v2/clock` - Current market clock

### Utility Functions

- [ ] `is_market_open()` - Check if market is currently open
- [ ] `next_market_open()` - Get next market open time
- [ ] `next_market_close()` - Get next market close time
- [ ] `is_trading_day(date)` - Check if date is a trading day
- [ ] `trading_days_between(start, end)` - Get trading days in range

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/getting-started-with-trading-api.html`

## Acceptance Criteria

- Full calendar data with sessions
- Clock with accurate market status
- Utility functions for common operations
- Timezone handling (Eastern Time)
- Unit tests for utilities
- Documentation for all public items
