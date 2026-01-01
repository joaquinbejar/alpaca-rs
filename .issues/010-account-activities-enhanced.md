# Issue 010: Enhanced Account Activities

## Priority: Medium

## Description

Enhance account activities support with all activity types and proper filtering.

## Current State

- Basic account activities endpoint exists
- Limited activity types defined

## Required Features

### Types (`alpaca-base`)

- [ ] `TradeActivity` struct: specific fields for trade activities
- [ ] `NonTradeActivity` struct: specific fields for non-trade activities
- [ ] Complete `ActivityType` enum with all types:
  - FILL, TRANS, MISC, ACATC, ACATS
  - CSD, CSR, CSW (cash deposits/receipts/withdrawals)
  - DIV, DIVCGL, DIVCGS, DIVFEE, DIVFT, DIVNRA, DIVROC, DIVTW, DIVTXEX
  - INT (interest)
  - JNLC, JNLS (journal cash/securities)
  - MA (merger/acquisition)
  - NC (name change)
  - OPASN, OPEXP, OPXRC (options assignment/expiration/exercise)
  - PTC, PTA (pass-through charge/adjustment)
  - REORG (reorganization)
  - SC (symbol change)
  - SSO (stock spinoff)
  - SSP (stock split)

### HTTP Endpoints (`alpaca-http`)

- [ ] `GET /v2/account/activities` - All activities with filters
- [ ] `GET /v2/account/activities/{activity_type}` - Activities by type
- [ ] Enhanced query parameters:
  - activity_types (multiple)
  - date
  - until
  - after
  - direction (asc/desc)
  - page_size
  - page_token

### Broker API Activities

- [ ] `GET /v1/accounts/activities` - All accounts activities
- [ ] `GET /v1/accounts/{account_id}/activities` - Specific account activities

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/account-activities.html`
- `doc/api/docs.alpaca.markets/docs/non-trade-activities-for-option-events.html`

## Acceptance Criteria

- All activity types properly modeled
- Trade vs non-trade activity distinction
- Proper filtering and pagination
- Broker API activities support
- Unit tests for all activity types
- Documentation for all public items
