# Issue 021: Margin and Short Selling Support

## Priority: Medium

## Description

Implement margin trading and short selling support with proper buying power calculations.

## Current State

- Basic account fields for margin exist
- No explicit short selling support
- Limited buying power calculation

## Required Features

### Types (`alpaca-base`)

- [ ] `MarginInfo` struct:
  - buying_power
  - regt_buying_power
  - daytrading_buying_power
  - non_marginable_buying_power
  - initial_margin
  - maintenance_margin
  - last_maintenance_margin
  - sma (Special Memorandum Account)
- [ ] `ShortPosition` struct with short-specific fields
- [ ] `BorrowRate` struct: symbol, rate, available_qty
- [ ] `MarginRequirement` struct: initial, maintenance

### HTTP Endpoints (`alpaca-http`)

#### Margin Information
- [ ] `GET /v2/account` - Enhanced margin fields
- [ ] Account configurations for margin settings

#### Short Selling
- [ ] `GET /v2/assets/{symbol}` - Check shortable, easy_to_borrow
- [ ] Short position tracking in positions endpoint

#### Locate/Borrow (Broker API)
- [ ] `GET /v1/locate/stocks` - Get locate availability
- [ ] `POST /v1/locate/stocks` - Request locate

### Calculations

- [ ] Buying power calculation helpers
- [ ] Margin requirement calculation
- [ ] Pattern day trader detection

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/margin-and-short-selling.html`

## Acceptance Criteria

- Full margin information retrieval
- Short selling order support
- Buying power calculations
- Locate request support (Broker API)
- Unit tests for calculations
- Documentation for margin trading
