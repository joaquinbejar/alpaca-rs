# Issue 022: Fractional Trading Support

## Priority: Medium

## Description

Implement full fractional trading support with notional orders and fractional position handling.

## Current State

- Basic notional order field exists
- Limited fractional position support

## Required Features

### Types (`alpaca-base`)

- [ ] `FractionalPosition` handling in Position struct
- [ ] `NotionalOrder` - Order by dollar amount
- [ ] `FractionalQty` type for precise fractional quantities

### HTTP Endpoints (`alpaca-http`)

#### Fractional Orders
- [ ] `POST /v2/orders` with notional field
- [ ] Fractional quantity validation
- [ ] Extended hours fractional trading

#### Fractional Positions
- [ ] Position with fractional qty display
- [ ] Close fractional position

### Validation

- [ ] Check asset is fractionable
- [ ] Minimum notional amount ($1)
- [ ] Maximum notional amount
- [ ] Fractional order type restrictions (market only)

### Asset Attributes

- [ ] `fractionable` field on Asset
- [ ] `min_order_size` for fractional
- [ ] `min_trade_increment`

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/fractional-trading.html`

## Acceptance Criteria

- Notional order creation
- Fractional position display
- Proper validation for fractional orders
- Asset fractionability check
- Unit tests for fractional handling
- Documentation for fractional trading
