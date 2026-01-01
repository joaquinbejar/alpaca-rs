# Issue 015: Comprehensive Error Handling

## Priority: High

## Description

Implement comprehensive error handling with typed errors for all API error responses.

## Current State

- Basic error types exist
- Limited error detail parsing
- No retry logic for transient errors

## Required Features

### Error Types (`alpaca-base`)

- [ ] `AlpacaError` enhancement with:
  - HTTP status code
  - Error code (Alpaca-specific)
  - Error message
  - Request ID for debugging
- [ ] `ApiErrorCode` enum with all Alpaca error codes:
  - 40010000: Malformed request
  - 40110000: Invalid credentials
  - 40310000: Forbidden
  - 40410000: Not found
  - 42210000: Unprocessable entity
  - 42910000: Rate limit exceeded
  - 50010000: Internal server error
- [ ] `ValidationError` struct for field-level errors
- [ ] `RateLimitError` struct with retry_after

### HTTP Error Handling (`alpaca-http`)

- [ ] Parse error response body for details
- [ ] Extract request ID from headers
- [ ] Rate limit detection and retry-after parsing
- [ ] Automatic retry for 5xx errors (configurable)
- [ ] Timeout handling

### WebSocket Error Handling (`alpaca-websocket`)

- [ ] Connection error types
- [ ] Authentication error handling
- [ ] Subscription error handling
- [ ] Message parsing error handling

### Error Display

- [ ] Implement `std::fmt::Display` for all errors
- [ ] Implement `std::error::Error` for all errors
- [ ] Human-readable error messages

## Acceptance Criteria

- All API errors properly typed
- Error codes mapped to enums
- Rate limit handling with retry-after
- Automatic retry for transient errors
- Request ID tracking for debugging
- Unit tests for error parsing
- Documentation for error handling
