use crate::messages::*;
use alpaca_base::types::*;
use futures_util::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// Stream of market data updates
pub struct MarketDataStream {
    receiver: mpsc::UnboundedReceiver<MarketDataUpdate>,
}

/// Market data update enum
#[derive(Debug, Clone)]
pub enum MarketDataUpdate {
    Trade { symbol: String, trade: Trade },
    Quote { symbol: String, quote: Quote },
    Bar { symbol: String, bar: Bar },
}

impl MarketDataStream {
    /// Create a new market data stream
    pub fn new(receiver: mpsc::UnboundedReceiver<MarketDataUpdate>) -> Self {
        Self { receiver }
    }
}

impl Stream for MarketDataStream {
    type Item = MarketDataUpdate;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Stream of trading updates
pub struct TradingStream {
    receiver: mpsc::UnboundedReceiver<TradeUpdateMessage>,
}

impl TradingStream {
    /// Create a new trading stream
    pub fn new(receiver: mpsc::UnboundedReceiver<TradeUpdateMessage>) -> Self {
        Self { receiver }
    }
}

impl Stream for TradingStream {
    type Item = TradeUpdateMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Stream of connection status updates
pub struct StatusStream {
    receiver: mpsc::UnboundedReceiver<ConnectionStatus>,
}

impl StatusStream {
    /// Create a new status stream
    pub fn new(receiver: mpsc::UnboundedReceiver<ConnectionStatus>) -> Self {
        Self { receiver }
    }
}

impl Stream for StatusStream {
    type Item = ConnectionStatus;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Combined stream of all WebSocket messages
pub struct AlpacaStream {
    receiver: mpsc::UnboundedReceiver<WebSocketMessage>,
}

impl AlpacaStream {
    /// Create a new Alpaca stream
    pub fn new(receiver: mpsc::UnboundedReceiver<WebSocketMessage>) -> Self {
        Self { receiver }
    }

    /// Filter for market data updates only
    pub fn market_data(self) -> impl Stream<Item = MarketDataUpdate> + Unpin {
        Box::pin(futures_util::stream::StreamExt::filter_map(
            self,
            |msg| async move {
                match msg {
                    WebSocketMessage::Trade(trade_msg) => Some(MarketDataUpdate::Trade {
                        symbol: trade_msg.symbol.clone(),
                        trade: trade_msg.into(),
                    }),
                    WebSocketMessage::Quote(quote_msg) => Some(MarketDataUpdate::Quote {
                        symbol: quote_msg.symbol.clone(),
                        quote: quote_msg.into(),
                    }),
                    WebSocketMessage::Bar(bar_msg) => Some(MarketDataUpdate::Bar {
                        symbol: bar_msg.symbol.clone(),
                        bar: bar_msg.into(),
                    }),
                    _ => None,
                }
            },
        ))
    }

    /// Filter for trading updates only
    pub fn trading_updates(self) -> impl Stream<Item = TradeUpdateMessage> + Unpin {
        Box::pin(futures_util::stream::StreamExt::filter_map(
            self,
            |msg| async move {
                match msg {
                    WebSocketMessage::TradeUpdate(update) => Some(*update),
                    _ => None,
                }
            },
        ))
    }

    /// Filter for status updates only
    pub fn status_updates(self) -> impl Stream<Item = ConnectionStatus> + Unpin {
        Box::pin(futures_util::stream::StreamExt::filter_map(
            self,
            |msg| async move {
                match msg {
                    WebSocketMessage::Connection(conn) => Some(conn.status),
                    _ => None,
                }
            },
        ))
    }
}

impl Stream for AlpacaStream {
    type Item = WebSocketMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}
