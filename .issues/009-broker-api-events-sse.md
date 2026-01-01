# Issue 009: Broker API Events (SSE)

## Priority: Medium

## Description

Implement Server-Sent Events (SSE) support for Broker API real-time event streaming.

## Current State

- No SSE support exists
- No event streaming for Broker API

## Required Features

### Types (`alpaca-base`)

- [ ] `AccountStatusEvent` struct
- [ ] `TransferStatusEvent` struct
- [ ] `TradeEvent` struct (Broker API specific)
- [ ] `JournalStatusEvent` struct
- [ ] `NonTradeActivityEvent` struct

### SSE Client (`alpaca-http` or new crate)

- [ ] SSE connection management
- [ ] Event parsing
- [ ] Automatic reconnection
- [ ] Event filtering

### Event Streams

- [ ] Account status events
- [ ] Transfer status events
- [ ] Trade events
- [ ] Journal events
- [ ] Non-trade activity events

### Endpoints

- [ ] `GET /v1/events/accounts/status` - Account status SSE
- [ ] `GET /v1/events/transfers/status` - Transfer status SSE
- [ ] `GET /v1/events/trades` - Trade events SSE
- [ ] `GET /v1/events/journals/status` - Journal status SSE
- [ ] `GET /v2beta1/events/nta` - Non-trade activity SSE

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/draft-sse-events.html`
- `doc/api/docs.alpaca.markets/docs/account-status-events-for-kycaas.html`

## Acceptance Criteria

- SSE connection with proper headers
- Event stream parsing
- Automatic reconnection on disconnect
- Event type filtering
- Async stream interface
- Unit tests for event parsing
- Documentation for SSE usage
