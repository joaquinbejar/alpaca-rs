# Issue 024: Local Currency Trading (LCT)

## Priority: Low

## Description

Implement Local Currency Trading support for international users trading in their local currency.

## Current State

- No LCT support

## Required Features

### Types (`alpaca-base`)

- [ ] `Currency` enum: USD, EUR, GBP, etc.
- [ ] `ExchangeRate` struct
- [ ] `LctOrder` struct with currency field
- [ ] `LctPosition` struct with local currency values

### HTTP Endpoints (`alpaca-http`)

#### LCT Orders
- [ ] Order creation with currency specification
- [ ] Currency conversion handling

#### Exchange Rates
- [ ] `GET /v1/fx/rates` - Get exchange rates
- [ ] `GET /v1/fx/rates/{currency_pair}` - Get specific rate

#### LCT Positions
- [ ] Position values in local currency
- [ ] P&L in local currency

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/local-currency-trading-lct.html`

## Acceptance Criteria

- Order placement in local currency
- Exchange rate retrieval
- Position display in local currency
- Currency conversion utilities
- Unit tests for currency handling
- Documentation for LCT
