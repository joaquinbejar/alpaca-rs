# Issue 003: Broker API - Funding & Transfers

## Priority: High

## Description

Implement Alpaca Broker API funding and transfer endpoints including ACH relationships, bank transfers, and wire transfers.

## Current State

- No funding/transfer support exists

## Required Features

### Types (`alpaca-base`)

- [ ] `AchRelationship` struct: id, account_id, status, account_owner_name, bank_account_type, bank_account_number, bank_routing_number, nickname, processor_token
- [ ] `AchRelationshipStatus` enum: QUEUED, APPROVED, PENDING, CANCEL_REQUESTED, CANCELED
- [ ] `BankAccountType` enum: CHECKING, SAVINGS
- [ ] `Transfer` struct: id, relationship_id, account_id, type, status, amount, direction, created_at, updated_at, expires_at, reason
- [ ] `TransferType` enum: ach, wire
- [ ] `TransferDirection` enum: INCOMING, OUTGOING
- [ ] `TransferStatus` enum: QUEUED, PENDING, SENT_TO_CLEARING, APPROVED, COMPLETE, RETURNED, CANCELED
- [ ] `WireBank` struct for wire transfer bank details
- [ ] `InstantFunding` struct for instant funding details

### HTTP Endpoints (`alpaca-http`)

#### ACH Relationships
- [ ] `POST /v1/accounts/{account_id}/ach_relationships` - Create ACH relationship
- [ ] `GET /v1/accounts/{account_id}/ach_relationships` - List ACH relationships
- [ ] `DELETE /v1/accounts/{account_id}/ach_relationships/{relationship_id}` - Delete ACH relationship

#### Transfers
- [ ] `POST /v1/accounts/{account_id}/transfers` - Create transfer
- [ ] `GET /v1/accounts/{account_id}/transfers` - List transfers
- [ ] `GET /v1/accounts/{account_id}/transfers/{transfer_id}` - Get transfer
- [ ] `DELETE /v1/accounts/{account_id}/transfers/{transfer_id}` - Cancel transfer

#### Wire Transfers
- [ ] `GET /v1/accounts/{account_id}/recipient_banks` - List recipient banks
- [ ] `POST /v1/accounts/{account_id}/recipient_banks` - Create recipient bank
- [ ] `DELETE /v1/accounts/{account_id}/recipient_banks/{bank_id}` - Delete recipient bank

#### Instant Funding
- [ ] `POST /v1/accounts/{account_id}/instant_funding` - Create instant funding

### Journals (Internal Transfers)
- [ ] `POST /v1/journals` - Create journal entry
- [ ] `GET /v1/journals` - List journals
- [ ] `POST /v1/journals/batch` - Batch journal entries
- [ ] `DELETE /v1/journals/{journal_id}` - Delete journal

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/ach-funding.html`
- `doc/api/docs.alpaca.markets/docs/funding-accounts.html`
- `doc/api/docs.alpaca.markets/docs/funding-via-journals.html`
- `doc/api/docs.alpaca.markets/docs/instant-funding-1.html`

## Acceptance Criteria

- ACH relationship management (create, list, delete)
- Transfer creation and management
- Wire transfer support
- Journal entries for internal transfers
- Instant funding support
- Proper status tracking and error handling
- Unit tests for all types and endpoints
- Documentation for all public items
