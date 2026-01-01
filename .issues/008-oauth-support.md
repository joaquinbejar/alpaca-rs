# Issue 008: OAuth 2.0 Support

## Priority: Medium

## Description

Implement OAuth 2.0 authentication flow for third-party applications integrating with Alpaca.

## Current State

- Only API key authentication exists
- No OAuth support

## Required Features

### Types (`alpaca-base`)

- [ ] `OAuthConfig` struct: client_id, client_secret, redirect_uri, scopes
- [ ] `OAuthToken` struct: access_token, refresh_token, token_type, expires_in, scope
- [ ] `OAuthScope` enum: account:write, trading, data, etc.
- [ ] `OAuthError` enum for OAuth-specific errors

### OAuth Flow Implementation

- [ ] Authorization URL generation
- [ ] Authorization code exchange
- [ ] Token refresh
- [ ] Token revocation

### HTTP Client Integration

- [ ] `AlpacaHttpClient::with_oauth(token)` constructor
- [ ] Automatic token refresh on 401
- [ ] Token storage interface (trait for custom storage)

### Endpoints

- [ ] `POST /oauth/token` - Exchange code for token
- [ ] `POST /oauth/token` - Refresh token
- [ ] `POST /oauth/revoke` - Revoke token

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/using-oauth2-and-trading-api.html`
- `doc/api/docs.alpaca.markets/docs/registering-your-app.html`

## Acceptance Criteria

- Complete OAuth 2.0 authorization code flow
- Token refresh mechanism
- Token revocation
- Integration with HTTP client
- Secure token handling
- Unit tests for OAuth flow
- Documentation for OAuth usage
