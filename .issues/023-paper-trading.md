# Issue 023: Paper Trading Support

## Priority: Medium

## Description

Implement proper paper trading environment support with account reset and simulation features.

## Current State

- Environment enum exists with Paper variant
- No paper-specific features

## Required Features

### Types (`alpaca-base`)

- [ ] `PaperTradingConfig` struct
- [ ] Environment URL handling for paper vs live

### HTTP Endpoints (`alpaca-http`)

#### Paper Trading Specific
- [ ] `POST /v2/account/reset` - Reset paper trading account
- [ ] Paper trading account configuration

### Client Configuration

- [ ] `AlpacaHttpClient::paper()` constructor
- [ ] `AlpacaHttpClient::live()` constructor
- [ ] Environment detection from API keys
- [ ] Prevent accidental live trading

### Features

- [ ] Account reset to initial state
- [ ] Simulated fills
- [ ] Paper trading indicators in responses

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/paper-trading.html`

## Acceptance Criteria

- Easy paper/live environment switching
- Account reset functionality
- Clear environment indication
- Safety checks for live trading
- Unit tests for environment handling
- Documentation for paper trading
