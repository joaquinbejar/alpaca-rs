//! Integration tests for the trading-updates streaming path, driven by a
//! local mock WebSocket server: reconnection with re-authentication,
//! bounded-channel backpressure, and auth rejection reporting.

mod common;

use common::*;

use std::time::Duration;

use alpaca_websocket::{TradingEvent, WebSocketConfig};
use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

/// After the connection drops, the client reconnects with backoff and
/// re-authenticates; when retries are exhausted it emits `Disconnected`
/// and ends the stream.
#[tokio::test]
async fn trading_reconnects_and_reauths_then_gives_up() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        // First connection: handshake, one update, then drop.
        let mut ws = accept_ws(&listener).await;
        let first_auth = trading_handshake(&mut ws).await;
        ws.send(trade_update_frame()).await.unwrap();
        ws.close(None).await.unwrap();
        drop(ws);

        // Second connection: handshake again, one more update, then drop
        // the listener so further reconnect attempts fail.
        let mut ws = accept_ws(&listener).await;
        let second_auth = trading_handshake(&mut ws).await;
        ws.send(trade_update_frame()).await.unwrap();
        ws.close(None).await.unwrap();
        (first_auth, second_auth)
    });

    let config = WebSocketConfig::new()
        .max_reconnect_attempts(3)
        .reconnect_base_delay(50);
    let stream = test_client(addr)
        .subscribe_trading_updates_with_config(config)
        .await
        .expect("subscribe should succeed");

    let events = collect_events(stream).await;
    let (first_auth, second_auth) = server.await.unwrap();

    // Authentication is re-issued on reconnect.
    assert!(first_auth.contains(r#""action":"auth""#));
    assert_eq!(first_auth, second_auth);

    let updates = events
        .iter()
        .filter(|e| matches!(e, TradingEvent::Update(_)))
        .count();
    assert_eq!(updates, 2, "expected both updates, got {events:?}");

    let reconnected_at = events
        .iter()
        .position(|e| matches!(e, TradingEvent::Reconnected))
        .expect("expected a Reconnected event");
    assert!(
        events[..reconnected_at]
            .iter()
            .any(|e| matches!(e, TradingEvent::Reconnecting { attempt: 1, .. })),
        "expected Reconnecting before Reconnected, got {events:?}"
    );
    assert!(
        matches!(events.last(), Some(TradingEvent::Disconnected { reason })
            if reason.contains("3 reconnect attempts")),
        "expected final Disconnected after exhausting retries, got {events:?}"
    );
}

/// A slow consumer on the bounded channel drops updates and observes the
/// loss via a `Lagged` event instead of unbounded memory growth.
#[tokio::test]
async fn trading_slow_consumer_gets_lagged_event() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        trading_handshake(&mut ws).await;
        for _ in 0..10 {
            ws.send(trade_update_frame()).await.unwrap();
        }
        ws.close(None).await.unwrap();
    });

    let config = WebSocketConfig::new().no_reconnect().buffer_size(2);
    let stream = test_client(addr)
        .subscribe_trading_updates_with_config(config)
        .await
        .expect("subscribe should succeed");

    // Let the server flood the channel before the consumer reads anything.
    tokio::time::sleep(Duration::from_millis(300)).await;

    let events = collect_events(stream).await;
    server.await.unwrap();

    let updates = events
        .iter()
        .filter(|e| matches!(e, TradingEvent::Update(_)))
        .count();
    assert_eq!(updates, 2, "only the buffered updates should survive");
    assert!(
        events
            .iter()
            .any(|e| matches!(e, TradingEvent::Lagged { missed: 8 })),
        "expected Lagged {{ missed: 8 }}, got {events:?}"
    );
    assert!(
        matches!(events.last(), Some(TradingEvent::Disconnected { .. })),
        "expected final Disconnected, got {events:?}"
    );
}

/// An authorization rejection during the initial handshake surfaces as an
/// `Err` from `subscribe_trading_updates`, not as a silent hang.
#[tokio::test]
async fn trading_auth_rejection_returns_error() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        let _auth = next_text(&mut ws).await;
        ws.send(Message::Text(
            r#"{"stream":"authorization","data":{"status":"unauthorized","action":"authenticate"}}"#
                .into(),
        ))
        .await
        .unwrap();
        ws.close(None).await.unwrap();
    });

    let config = WebSocketConfig::new().no_reconnect();
    let result = test_client(addr)
        .subscribe_trading_updates_with_config(config)
        .await;
    server.await.unwrap();

    let err = match result {
        Ok(_) => panic!("authorization rejection must fail the subscribe call"),
        Err(e) => e,
    };
    assert!(
        err.to_string()
            .contains("authorization status: unauthorized"),
        "unexpected error: {err}"
    );
}
