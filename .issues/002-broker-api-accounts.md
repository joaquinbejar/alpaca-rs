# Issue 002: Broker API - Account Management

## Priority: High

## Description

Implement Alpaca Broker API account management endpoints. The Broker API allows partners to create and manage customer accounts programmatically.

## Current State

- Only Trading API account endpoints exist
- No Broker API support

## Required Features

### Types (`alpaca-base`)

- [ ] `BrokerAccount` struct with full KYC fields
- [ ] `Contact` struct: email, phone, street_address, city, state, postal_code, country
- [ ] `Identity` struct: given_name, family_name, date_of_birth, tax_id, country_of_citizenship, funding_source, etc.
- [ ] `Disclosures` struct: is_control_person, is_affiliated_exchange_or_finra, is_politically_exposed, etc.
- [ ] `Agreement` struct: agreement type, signed_at, ip_address, revision
- [ ] `TrustedContact` struct
- [ ] `Document` struct for KYC documents
- [ ] `AccountStatus` enum with all Broker API statuses
- [ ] `AgreementType` enum: margin_agreement, account_agreement, customer_agreement, crypto_agreement, options_agreement
- [ ] `FundingSource` enum: employment_income, investments, inheritance, business_income, savings, family
- [ ] `TaxIdType` enum: USA_SSN, ARG_AR_CUIT, AUS_TFN, etc.

### HTTP Endpoints (`alpaca-http`)

- [ ] `POST /v1/accounts` - Create new account
- [ ] `GET /v1/accounts` - List all accounts
- [ ] `GET /v1/accounts/{account_id}` - Get account by ID
- [ ] `PATCH /v1/accounts/{account_id}` - Update account
- [ ] `DELETE /v1/accounts/{account_id}` - Close account
- [ ] `GET /v1/accounts/{account_id}/trading` - Get trading account details
- [ ] `PATCH /v1/accounts/{account_id}/trading` - Update trading configurations

### CIP (Customer Identification Program) Endpoints

- [ ] `POST /v1/accounts/{account_id}/cip` - Submit CIP data
- [ ] `GET /v1/accounts/{account_id}/cip` - Get CIP status

### Document Endpoints

- [ ] `POST /v1/accounts/{account_id}/documents/upload` - Upload document
- [ ] `GET /v1/accounts/{account_id}/documents` - List documents
- [ ] `GET /v1/accounts/{account_id}/documents/{document_id}` - Get document
- [ ] `DELETE /v1/accounts/{account_id}/documents/{document_id}` - Delete document

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/about-broker-api.html`
- `doc/api/docs.alpaca.markets/docs/account-opening.html`
- `doc/api/docs.alpaca.markets/docs/accounts-statuses.html`

## Acceptance Criteria

- Full account lifecycle management (create, read, update, close)
- KYC data submission and validation
- Document upload and management
- CIP data handling
- Proper error handling for all Broker API error codes
- Unit tests for all types and endpoints
- Documentation for all public items
