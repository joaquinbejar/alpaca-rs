# Issue 020: Statements and Trade Confirmations

## Priority: Low

## Description

Implement support for retrieving account statements and trade confirmations.

## Current State

- No statements support
- No trade confirmations support

## Required Features

### Types (`alpaca-base`)

- [ ] `Statement` struct: id, account_id, date, type, url
- [ ] `StatementType` enum: account_statement, trade_confirmation, tax_document
- [ ] `TradeConfirmation` struct
- [ ] `TaxDocument` struct: form_type (1099, etc.)

### HTTP Endpoints (`alpaca-http`)

#### Statements (Broker API)
- [ ] `GET /v1/accounts/{account_id}/documents` - List documents
- [ ] `GET /v1/accounts/{account_id}/documents/{document_id}` - Get document
- [ ] `GET /v1/accounts/{account_id}/documents/{document_id}/download` - Download PDF

#### Trading API Statements
- [ ] `GET /v2/account/documents` - List account documents

### Document Types

- [ ] Monthly statements
- [ ] Trade confirmations
- [ ] Tax documents (1099-B, 1099-DIV, 1099-INT)
- [ ] Year-end statements

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/statements-and-confirms.html`

## Acceptance Criteria

- List all document types
- Download documents as PDF
- Filter by date range and type
- Unit tests for document types
- Documentation for document retrieval
