# Issue 013: Enhanced Assets API

## Priority: Medium

## Description

Enhance assets API with full filtering, asset attributes, and corporate actions data.

## Current State

- Basic assets endpoint exists
- Limited filtering options
- No asset attributes support

## Required Features

### Types (`alpaca-base`)

- [ ] `Asset` enhancement with all fields:
  - maintenance_margin_requirement
  - min_order_size
  - min_trade_increment
  - price_increment
  - attributes (list)
- [ ] `AssetAttributes` enum:
  - ptp_no_exception
  - ptp_with_exception
  - ipo
  - options_enabled
  - fractional_eh_enabled
- [ ] `AssetExchange` enum with all exchanges
- [ ] `OptionContractAsset` struct for options assets

### HTTP Endpoints (`alpaca-http`)

#### Assets
- [ ] `GET /v2/assets` - Enhanced with filters:
  - status (active, inactive)
  - asset_class (us_equity, crypto, us_option)
  - exchange
  - attributes
- [ ] `GET /v2/assets/{symbol_or_asset_id}` - Get asset details
- [ ] `GET /v2/assets/{symbol}/options` - Get options for underlying

#### Announcements
- [ ] `GET /v1beta1/corporate-actions/announcements` - Corporate actions
  - ca_types filter
  - since/until dates
  - symbol filter
  - cusip filter

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/working-with-assets.html`
- `doc/api/docs.alpaca.markets/docs/mandatory-corporate-actions.html`

## Acceptance Criteria

- Full asset data with all attributes
- Asset filtering by multiple criteria
- Options assets support
- Corporate actions announcements
- Unit tests for all types and endpoints
- Documentation for all public items
