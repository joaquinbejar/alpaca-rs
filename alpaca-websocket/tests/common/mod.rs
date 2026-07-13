//! Shared helpers for market-data streaming integration tests: a mock
//! Alpaca WebSocket server and an in-memory log sink.

#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use alpaca_base::auth::Credentials;
use alpaca_base::types::Environment;
use alpaca_websocket::{
    AlpacaWebSocketClient, MarketDataEvent, MarketDataStream, SubscribeMessage, SubscriptionBuilder,
};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};

pub const TEST_KEY: &str = "PKTESTKEY1234567890";
pub const TEST_SECRET: &str = "SUPERSECRETVALUE9876";

pub type ServerWs = WebSocketStream<TcpStream>;

pub fn test_client(addr: SocketAddr) -> AlpacaWebSocketClient {
    let credentials = Credentials::new(TEST_KEY.to_string(), TEST_SECRET.to_string());
    AlpacaWebSocketClient::with_url(credentials, Environment::Paper, format!("ws://{addr}"))
}

pub fn test_subscription() -> SubscribeMessage {
    SubscriptionBuilder::new().trades(["AAPL"]).build()
}

pub async fn accept_ws(listener: &TcpListener) -> ServerWs {
    let (tcp, _) = listener.accept().await.expect("accept");
    accept_async(tcp).await.expect("websocket handshake")
}

pub async fn next_text(ws: &mut ServerWs) -> String {
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(text))) => return text.to_string(),
            Some(Ok(_)) => continue,
            other => panic!("expected text frame, got {other:?}"),
        }
    }
}

/// Server side of the Alpaca handshake: hello, auth, subscription.
/// Returns the raw auth and subscribe frames received from the client.
pub async fn server_handshake(ws: &mut ServerWs) -> (String, String) {
    ws.send(Message::Text(
        r#"[{"T":"success","msg":"connected"}]"#.into(),
    ))
    .await
    .unwrap();
    let auth = next_text(ws).await;
    ws.send(Message::Text(
        r#"[{"T":"success","msg":"authenticated"}]"#.into(),
    ))
    .await
    .unwrap();
    let subscribe = next_text(ws).await;
    ws.send(Message::Text(
        r#"[{"T":"subscription","trades":["AAPL"],"quotes":[],"bars":[]}]"#.into(),
    ))
    .await
    .unwrap();
    (auth, subscribe)
}

pub fn trade_frame(id: u64) -> Message {
    Message::Text(
        format!(
            r#"[{{"T":"t","S":"AAPL","t":"2026-07-13T10:00:00Z","p":190.5,"s":100,"x":"V","c":[],"i":{id}}}]"#
        )
        .into(),
    )
}

pub async fn collect_events(mut stream: MarketDataStream) -> Vec<MarketDataEvent> {
    let mut events = Vec::new();
    while let Some(event) = stream.next().await {
        events.push(event);
    }
    events
}

/// In-memory log sink for asserting on emitted tracing output.
#[derive(Clone, Default)]
pub struct LogBuffer(Arc<Mutex<Vec<u8>>>);

impl LogBuffer {
    pub fn contents(&self) -> String {
        String::from_utf8_lossy(&self.0.lock().unwrap()).into_owned()
    }
}

impl std::io::Write for LogBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for LogBuffer {
    type Writer = LogBuffer;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}
