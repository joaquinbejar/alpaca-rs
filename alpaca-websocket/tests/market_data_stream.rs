//! Integration tests for the market-data streaming path, driven by a local
//! mock WebSocket server: reconnection, bounded-channel backpressure, and
//! handshake failure reporting (#76).

mod common;

use common::*;

use std::time::Duration;

use alpaca_websocket::{MarketDataEvent, MarketDataUpdate, WebSocketConfig};
use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

/// #76: after the connection drops, the client reconnects with backoff,
/// re-issues the same subscription set, and reports lifecycle events; when
/// retries are exhausted it emits `Disconnected` and ends the stream.
#[tokio::test]
async fn reconnects_and_resubscribes_then_gives_up() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        // First connection: handshake, one trade, then drop.
        let mut ws = accept_ws(&listener).await;
        let (_, first_sub) = server_handshake(&mut ws).await;
        ws.send(trade_frame(1)).await.unwrap();
        ws.close(None).await.unwrap();
        drop(ws);

        // Second connection: handshake again, one more trade, then drop
        // the listener so further reconnect attempts fail.
        let mut ws = accept_ws(&listener).await;
        let (_, second_sub) = server_handshake(&mut ws).await;
        ws.send(trade_frame(2)).await.unwrap();
        ws.close(None).await.unwrap();
        (first_sub, second_sub)
    });

    let config = WebSocketConfig::new()
        .max_reconnect_attempts(3)
        .reconnect_base_delay(50);
    let stream = test_client(addr)
        .subscribe_market_data_with_config(test_subscription(), config)
        .await
        .expect("subscribe should succeed");

    let events = collect_events(stream).await;
    let (first_sub, second_sub) = server.await.unwrap();

    // The subscription set is re-issued verbatim after reconnecting.
    assert_eq!(first_sub, second_sub);
    assert!(second_sub.contains("AAPL"));

    let updates = events
        .iter()
        .filter(|e| matches!(e, MarketDataEvent::Update(MarketDataUpdate::Trade { .. })))
        .count();
    assert_eq!(updates, 2, "expected both trades, got {events:?}");

    let reconnected_at = events
        .iter()
        .position(|e| matches!(e, MarketDataEvent::Reconnected))
        .expect("expected a Reconnected event");
    assert!(
        events[..reconnected_at]
            .iter()
            .any(|e| matches!(e, MarketDataEvent::Reconnecting { attempt: 1, .. })),
        "expected Reconnecting before Reconnected, got {events:?}"
    );
    assert!(
        matches!(events.last(), Some(MarketDataEvent::Disconnected { reason })
            if reason.contains("3 reconnect attempts")),
        "expected final Disconnected after exhausting retries, got {events:?}"
    );
}

/// #76: a slow consumer on the bounded channel drops updates and observes
/// the loss via a `Lagged` event instead of unbounded memory growth.
#[tokio::test]
async fn slow_consumer_gets_lagged_event() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        server_handshake(&mut ws).await;
        for id in 1..=10 {
            ws.send(trade_frame(id)).await.unwrap();
        }
        ws.close(None).await.unwrap();
    });

    let config = WebSocketConfig::new().no_reconnect().buffer_size(2);
    let stream = test_client(addr)
        .subscribe_market_data_with_config(test_subscription(), config)
        .await
        .expect("subscribe should succeed");

    // Let the server flood the channel before the consumer reads anything.
    tokio::time::sleep(Duration::from_millis(300)).await;

    let events = collect_events(stream).await;
    server.await.unwrap();

    let updates = events
        .iter()
        .filter(|e| matches!(e, MarketDataEvent::Update(_)))
        .count();
    assert_eq!(updates, 2, "only the buffered updates should survive");
    assert!(
        events
            .iter()
            .any(|e| matches!(e, MarketDataEvent::Lagged { missed: 8 })),
        "expected Lagged {{ missed: 8 }}, got {events:?}"
    );
    assert!(
        matches!(events.last(), Some(MarketDataEvent::Disconnected { .. })),
        "expected final Disconnected, got {events:?}"
    );
}

/// An auth rejection during the initial handshake surfaces as an `Err` from
/// `subscribe_market_data`, not as a silent hang.
#[tokio::test]
async fn initial_auth_failure_returns_error() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        ws.send(Message::Text(
            r#"[{"T":"success","msg":"connected"}]"#.into(),
        ))
        .await
        .unwrap();
        let _auth = next_text(&mut ws).await;
        ws.send(Message::Text(
            r#"[{"T":"error","code":402,"msg":"auth failed"}]"#.into(),
        ))
        .await
        .unwrap();
        ws.close(None).await.unwrap();
    });

    let config = WebSocketConfig::new().no_reconnect();
    let result = test_client(addr)
        .subscribe_market_data_with_config(test_subscription(), config)
        .await;
    server.await.unwrap();

    let err = match result {
        Ok(_) => panic!("auth rejection must fail the subscribe call"),
        Err(e) => e,
    };
    assert!(
        err.to_string().contains("auth failed"),
        "unexpected error: {err}"
    );
}
