//! WebSocket message types for Alpaca streaming.

#![allow(missing_docs)]

use alpaca_base::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "T")]
pub enum WebSocketMessage {
    /// Authentication message
    #[serde(rename = "auth")]
    Auth(AuthMessage),

    /// Subscription message
    #[serde(rename = "subscribe")]
    Subscribe(SubscribeMessage),

    /// Unsubscription message
    #[serde(rename = "unsubscribe")]
    Unsubscribe(UnsubscribeMessage),

    /// Market data messages
    #[serde(rename = "t")]
    Trade(TradeMessage),

    #[serde(rename = "q")]
    Quote(QuoteMessage),

    #[serde(rename = "b")]
    Bar(BarMessage),

    /// Trading messages
    #[serde(rename = "trade_updates")]
    TradeUpdate(Box<TradeUpdateMessage>),

    /// Status messages
    #[serde(rename = "success")]
    Success(SuccessMessage),

    #[serde(rename = "error")]
    Error(ErrorMessage),

    /// Connection status
    #[serde(rename = "connection")]
    Connection(ConnectionMessage),
}

/// Authentication message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMessage {
    pub key: String,
    pub secret: String,
}

/// Subscription message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMessage {
    pub trades: Option<Vec<String>>,
    pub quotes: Option<Vec<String>>,
    pub bars: Option<Vec<String>>,
    pub trade_updates: Option<bool>,
}

/// Unsubscription message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeMessage {
    pub trades: Option<Vec<String>>,
    pub quotes: Option<Vec<String>>,
    pub bars: Option<Vec<String>>,
    pub trade_updates: Option<bool>,
}

/// Trade message from WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeMessage {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: u32,
    #[serde(rename = "x")]
    pub exchange: String,
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    #[serde(rename = "i")]
    pub id: u64,
}

/// Quote message from WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteMessage {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "bp")]
    pub bid_price: f64,
    #[serde(rename = "bs")]
    pub bid_size: u32,
    #[serde(rename = "ap")]
    pub ask_price: f64,
    #[serde(rename = "as")]
    pub ask_size: u32,
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    #[serde(rename = "ax")]
    pub ask_exchange: String,
}

/// Bar message from WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarMessage {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: u64,
    #[serde(rename = "n")]
    pub trade_count: Option<u64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

/// Trade update message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeUpdateMessage {
    pub event: TradeUpdateEvent,
    pub order: Order,
    pub timestamp: DateTime<Utc>,
    pub position_qty: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
}

/// Trade update event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeUpdateEvent {
    New,
    Fill,
    PartialFill,
    Canceled,
    Expired,
    DoneForDay,
    Replaced,
    Rejected,
    PendingNew,
    Stopped,
    PendingCancel,
    PendingReplace,
    Calculated,
    Suspended,
    OrderReplacePending,
    OrderCancelPending,
}

/// Success message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMessage {
    pub msg: String,
}

/// Error message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub msg: String,
}

/// Connection status message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMessage {
    pub status: ConnectionStatus,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Connected,
    Authenticated,
    AuthenticationFailed,
    Disconnected,
    Reconnecting,
}

/// Subscription request builder
#[derive(Debug, Default)]
pub struct SubscriptionBuilder {
    trades: Vec<String>,
    quotes: Vec<String>,
    bars: Vec<String>,
    trade_updates: bool,
}

impl SubscriptionBuilder {
    /// Create a new subscription builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscribe to trades for symbols
    pub fn trades<I, S>(mut self, symbols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.trades.extend(symbols.into_iter().map(|s| s.into()));
        self
    }

    /// Subscribe to quotes for symbols
    pub fn quotes<I, S>(mut self, symbols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.quotes.extend(symbols.into_iter().map(|s| s.into()));
        self
    }

    /// Subscribe to bars for symbols
    pub fn bars<I, S>(mut self, symbols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.bars.extend(symbols.into_iter().map(|s| s.into()));
        self
    }

    /// Subscribe to trade updates
    pub fn trade_updates(mut self) -> Self {
        self.trade_updates = true;
        self
    }

    /// Build the subscription message
    pub fn build(self) -> SubscribeMessage {
        SubscribeMessage {
            trades: if self.trades.is_empty() {
                None
            } else {
                Some(self.trades)
            },
            quotes: if self.quotes.is_empty() {
                None
            } else {
                Some(self.quotes)
            },
            bars: if self.bars.is_empty() {
                None
            } else {
                Some(self.bars)
            },
            trade_updates: if self.trade_updates { Some(true) } else { None },
        }
    }
}

impl From<TradeMessage> for Trade {
    fn from(msg: TradeMessage) -> Self {
        Trade {
            timestamp: msg.timestamp,
            price: msg.price,
            size: msg.size,
            exchange: msg.exchange,
            conditions: msg.conditions,
            id: msg.id,
        }
    }
}

impl From<QuoteMessage> for Quote {
    fn from(msg: QuoteMessage) -> Self {
        Quote {
            timestamp: msg.timestamp,
            timeframe: "real-time".to_string(),
            bid_price: msg.bid_price,
            bid_size: msg.bid_size,
            ask_price: msg.ask_price,
            ask_size: msg.ask_size,
            bid_exchange: msg.bid_exchange,
            ask_exchange: msg.ask_exchange,
        }
    }
}

impl From<BarMessage> for Bar {
    fn from(msg: BarMessage) -> Self {
        Bar {
            timestamp: msg.timestamp,
            open: msg.open,
            high: msg.high,
            low: msg.low,
            close: msg.close,
            volume: msg.volume,
            trade_count: msg.trade_count,
            vwap: msg.vwap,
        }
    }
}
