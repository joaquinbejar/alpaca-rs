# Issue 006: News API Enhancement

## Priority: Medium

## Description

Enhance the News API with full support for historical news, real-time news streaming, and news images.

## Current State

- Basic news endpoint exists
- No real-time news streaming
- No news images support

## Required Features

### Types (`alpaca-base`)

- [ ] `NewsArticle` enhancement: add images, source, content_type
- [ ] `NewsImage` struct: size, url, thumbnail_url
- [ ] `NewsSource` struct: name, url, favicon_url
- [ ] `NewsContentType` enum: article, video, audio

### HTTP Endpoints (`alpaca-http`)

- [ ] `GET /v1beta1/news` - Enhanced with all query parameters
  - symbols filter
  - start/end date range
  - sort order
  - include_content flag
  - exclude_contentless flag
  - limit and pagination

### WebSocket Streams (`alpaca-websocket`)

- [ ] Real-time news stream subscription
- [ ] News message type handling
- [ ] Symbol-based news filtering

## API Documentation Reference

- `doc/api/docs.alpaca.markets/docs/historical-news-data.html`
- `doc/api/docs.alpaca.markets/docs/streaming-real-time-news.html`

## Acceptance Criteria

- Full news article retrieval with all fields
- News images support
- Real-time news streaming via WebSocket
- Symbol-based filtering
- Proper pagination
- Unit tests for all types and endpoints
- Documentation for all public items
