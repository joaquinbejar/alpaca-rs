# Issue 004: Enhanced Crypto Trading Support

## Priority: High

## Description

Enhance crypto trading support with full API coverage including crypto wallets, crypto transfers, and improved crypto market data.

## Current State

- Basic crypto bars, quotes, and trades endpoints exist
- No crypto wallet support
- No crypto transfer support

## Required Features

### Types (`alpaca-base`)

- [ ] `CryptoWallet` struct enhancement: address, chain, status
- [ ] `CryptoTransfer` struct: id, wallet_id, direction, amount, status, fee, tx_hash, created_at
- [ ] `CryptoTransferStatus` enum: PENDING, APPROVED, PENDING_SEND, SENT, COMPLETE, REJECTED, FAILED
- [ ] `CryptoChain` enum: ETH, BTC, SOL, etc.
- [ ] `CryptoSnapshot` struct with current price data
- [ ] `CryptoOrderbook` struct: bids, asks, timestamp
- [ ] `CryptoFundingWallet` struct

### HTTP Endpoints (`alpaca-http`)

#### Crypto Wallets (Broker API)
- [ ] `GET /v1/accounts/{account_id}/wallets` - List crypto wallets
- [ ] `POST /v1/accounts/{account_id}/wallets` - Create wallet
- [ ] `GET /v1/accounts/{account_id}/wallets/{asset}` - Get wallet by asset
- [ ] `GET /v1/accounts/{account_id}/wallets/transfers` - List wallet transfers
- [ ] `POST /v1/accounts/{account_id}/wallets/transfers` - Create wallet transfer
- [ ] `GET /v1/accounts/{account_id}/wallets/whitelists` - List whitelisted addresses
- [ ] `POST /v1/accounts/{account_id}/wallets/whitelists` - Add whitelisted address

#### Enhanced Crypto Market Data
- [ ] `GET /v1beta1/crypto/latest/bars` - Latest crypto bars (multi-symbol)
- [ ] `GET /v1beta1/crypto/latest/quotes` - Latest crypto quotes (multi-symbol)
- [ ] `GET /v1beta1/crypto/latest/trades` - Latest crypto trades (multi-symbol)
- [ ] `GET /v1beta1/crypto/snapshots` - Crypto snapshots
- [ ] `GET /v1beta1/crypto/latest/orderbooks` - Crypto orderbook

### WebSocket Streams (`alpaca-websocket`)

- [ ] Crypto trades stream
- [ ] Crypto quotes stream
- [ ] Crypto bars stream
- [ ] Crypto orderbook stream

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/crypto-trading-1.html`
- `doc/api/docs.alpaca.markets/docs/crypto-wallets-api.html`
- `doc/api/docs.alpaca.markets/docs/crypto-pricing-data.html`
- `doc/api/docs.alpaca.markets/docs/real-time-crypto-pricing-data.html`

## Acceptance Criteria

- Full crypto wallet management
- Crypto transfers (deposit/withdraw)
- Enhanced crypto market data endpoints
- Real-time crypto WebSocket streams
- Orderbook data support
- Unit tests for all types and endpoints
- Documentation for all public items
