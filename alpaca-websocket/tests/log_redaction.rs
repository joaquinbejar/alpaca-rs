//! #75: the connect + authenticate + subscribe flow must never log the API
//! key or secret.
//!
//! This test lives in its own binary (and therefore its own process) so the
//! captured tracing output cannot be affected by other tests running in
//! parallel threads.

mod common;

use common::*;

use alpaca_websocket::{MarketDataEvent, WebSocketConfig};
use futures_util::SinkExt;
use tokio::net::TcpListener;

#[tokio::test]
async fn credentials_never_appear_in_logs() {
    let logs = LogBuffer::default();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(logs.clone())
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        let (auth, _) = server_handshake(&mut ws).await;
        // Sanity check: the auth frame itself does carry the credentials.
        assert!(auth.contains(TEST_KEY) && auth.contains(TEST_SECRET));
        ws.send(trade_frame(1)).await.unwrap();
        ws.close(None).await.unwrap();
    });

    let config = WebSocketConfig::new().no_reconnect();
    let stream = test_client(addr)
        .subscribe_market_data_with_config(test_subscription(), config)
        .await
        .expect("subscribe should succeed");

    let events = collect_events(stream).await;
    server.await.unwrap();

    assert!(
        events
            .iter()
            .any(|e| matches!(e, MarketDataEvent::Update(_))),
        "expected at least one update, got {events:?}"
    );

    let output = logs.contents();
    assert!(!output.is_empty(), "expected some log output");
    assert!(
        !output.contains(TEST_SECRET),
        "secret leaked into logs:\n{output}"
    );
    assert!(
        !output.contains(TEST_KEY),
        "API key leaked into logs:\n{output}"
    );
    assert!(
        output.contains("****7890"),
        "expected redacted key marker in logs:\n{output}"
    );
}
