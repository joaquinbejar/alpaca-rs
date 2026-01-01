# Issue 025: IRA Account Support

## Priority: Low

## Description

Implement IRA (Individual Retirement Account) support for Broker API partners.

## Current State

- No IRA account support

## Required Features

### Types (`alpaca-base`)

- [ ] `IraAccountType` enum: traditional, roth, sep, simple
- [ ] `IraContribution` struct
- [ ] `IraDistribution` struct
- [ ] `IraBeneficiary` struct

### HTTP Endpoints (`alpaca-http`)

#### IRA Account Management
- [ ] Create IRA account (POST /v1/accounts with IRA type)
- [ ] IRA-specific account fields

#### Contributions
- [ ] `POST /v1/accounts/{account_id}/ira/contributions`
- [ ] `GET /v1/accounts/{account_id}/ira/contributions`

#### Distributions
- [ ] `POST /v1/accounts/{account_id}/ira/distributions`
- [ ] `GET /v1/accounts/{account_id}/ira/distributions`

#### Beneficiaries
- [ ] `POST /v1/accounts/{account_id}/ira/beneficiaries`
- [ ] `GET /v1/accounts/{account_id}/ira/beneficiaries`
- [ ] `DELETE /v1/accounts/{account_id}/ira/beneficiaries/{id}`

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/ira-accounts-overview.html`

## Acceptance Criteria

- IRA account creation
- Contribution tracking
- Distribution management
- Beneficiary management
- Unit tests for IRA types
- Documentation for IRA accounts
