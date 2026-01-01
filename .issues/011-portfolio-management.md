# Issue 011: Portfolio Management API

## Priority: Medium

## Description

Implement portfolio management features including portfolio history, portfolio rebalancing, and position management enhancements.

## Current State

- Basic portfolio history endpoint exists
- No rebalancing support
- Limited position management

## Required Features

### Types (`alpaca-base`)

- [ ] `PortfolioHistory` enhancement with all fields
- [ ] `RebalanceRequest` struct
- [ ] `RebalanceResult` struct
- [ ] `TargetAllocation` struct: symbol, percent, notional
- [ ] `RebalanceStatus` enum

### HTTP Endpoints (`alpaca-http`)

#### Portfolio History
- [ ] `GET /v2/account/portfolio/history` - Enhanced with all params:
  - period (1D, 1W, 1M, 3M, 1A, all)
  - timeframe (1Min, 5Min, 15Min, 1H, 1D)
  - date_end
  - extended_hours
  - intraday_reporting
  - pnl_reset

#### Position Management
- [ ] `GET /v2/positions` - List all positions
- [ ] `GET /v2/positions/{symbol_or_asset_id}` - Get position
- [ ] `DELETE /v2/positions/{symbol_or_asset_id}` - Close position with params
- [ ] `DELETE /v2/positions` - Close all positions
- [ ] `PATCH /v2/positions/{symbol_or_asset_id}` - Modify position (exercise options)

#### Portfolio Rebalancing (Broker API)
- [ ] `POST /v1/rebalancing/portfolios` - Create portfolio
- [ ] `GET /v1/rebalancing/portfolios` - List portfolios
- [ ] `GET /v1/rebalancing/portfolios/{portfolio_id}` - Get portfolio
- [ ] `PATCH /v1/rebalancing/portfolios/{portfolio_id}` - Update portfolio
- [ ] `DELETE /v1/rebalancing/portfolios/{portfolio_id}` - Delete portfolio
- [ ] `POST /v1/rebalancing/runs` - Execute rebalance
- [ ] `GET /v1/rebalancing/runs` - List rebalance runs

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/working-with-positions.html`
- `doc/api/docs.alpaca.markets/docs/portfolio-rebalancing.html`
- `doc/api/docs.alpaca.markets/docs/position-average-entry-price-calculation.html`

## Acceptance Criteria

- Enhanced portfolio history with all parameters
- Position close with quantity/percentage options
- Portfolio rebalancing for Broker API
- Proper calculation of P&L
- Unit tests for all types and endpoints
- Documentation for all public items
