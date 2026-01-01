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

// ============================================================================
// Enhanced WebSocket Message Types
// ============================================================================

/// Crypto trade message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoTradeMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Trade size.
    #[serde(rename = "s")]
    pub size: f64,
    /// Taker side (buy or sell).
    #[serde(rename = "tks")]
    pub taker_side: String,
    /// Trade ID.
    #[serde(rename = "i")]
    pub id: u64,
}

/// Crypto quote message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoQuoteMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid price.
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size.
    #[serde(rename = "bs")]
    pub bid_size: f64,
    /// Ask price.
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size.
    #[serde(rename = "as")]
    pub ask_size: f64,
}

/// Crypto bar message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBarMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Open price.
    #[serde(rename = "o")]
    pub open: f64,
    /// High price.
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price.
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price.
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume.
    #[serde(rename = "v")]
    pub volume: f64,
    /// Number of trades.
    #[serde(rename = "n")]
    pub trade_count: Option<u64>,
    /// Volume-weighted average price.
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

/// Options trade message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionTradeMessage {
    /// Option symbol (OCC format).
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Trade size (number of contracts).
    #[serde(rename = "s")]
    pub size: u32,
    /// Exchange.
    #[serde(rename = "x")]
    pub exchange: String,
    /// Trade conditions.
    #[serde(rename = "c", default)]
    pub conditions: Option<String>,
}

/// Options quote message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionQuoteMessage {
    /// Option symbol (OCC format).
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid price.
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size.
    #[serde(rename = "bs")]
    pub bid_size: u32,
    /// Ask price.
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size.
    #[serde(rename = "as")]
    pub ask_size: u32,
    /// Bid exchange.
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    /// Ask exchange.
    #[serde(rename = "ax")]
    pub ask_exchange: String,
}

/// News message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsMessage {
    /// News ID.
    pub id: u64,
    /// Headline.
    pub headline: String,
    /// Summary.
    pub summary: Option<String>,
    /// Author.
    pub author: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Update timestamp.
    pub updated_at: DateTime<Utc>,
    /// URL to full article.
    pub url: Option<String>,
    /// Related symbols.
    pub symbols: Vec<String>,
    /// Source.
    pub source: String,
}

/// Limit Up Limit Down (LULD) message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuldMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// LULD indicator.
    #[serde(rename = "i")]
    pub indicator: String,
    /// Limit up price.
    #[serde(rename = "u")]
    pub limit_up_price: f64,
    /// Limit down price.
    #[serde(rename = "d")]
    pub limit_down_price: f64,
}

/// Trading status message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingStatusMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Status code.
    #[serde(rename = "sc")]
    pub status_code: String,
    /// Status message.
    #[serde(rename = "sm")]
    pub status_message: String,
    /// Reason code.
    #[serde(rename = "rc")]
    pub reason_code: String,
    /// Reason message.
    #[serde(rename = "rm")]
    pub reason_message: String,
}

/// Trade correction message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Original trade ID.
    #[serde(rename = "x")]
    pub original_id: u64,
    /// Original price.
    #[serde(rename = "op")]
    pub original_price: f64,
    /// Original size.
    #[serde(rename = "os")]
    pub original_size: u32,
    /// Corrected price.
    #[serde(rename = "cp")]
    pub corrected_price: f64,
    /// Corrected size.
    #[serde(rename = "cs")]
    pub corrected_size: u32,
}

/// Cancel error message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelErrorMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade ID that was canceled in error.
    #[serde(rename = "i")]
    pub id: u64,
    /// Original price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Original size.
    #[serde(rename = "s")]
    pub size: u32,
}

/// Daily bar message from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyBarMessage {
    /// Symbol.
    #[serde(rename = "S")]
    pub symbol: String,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Open price.
    #[serde(rename = "o")]
    pub open: f64,
    /// High price.
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price.
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price.
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume.
    #[serde(rename = "v")]
    pub volume: u64,
    /// Volume-weighted average price.
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_builder() {
        let sub = SubscriptionBuilder::new()
            .trades(["AAPL", "MSFT"])
            .quotes(["GOOGL"])
            .trade_updates()
            .build();

        assert_eq!(
            sub.trades,
            Some(vec!["AAPL".to_string(), "MSFT".to_string()])
        );
        assert_eq!(sub.quotes, Some(vec!["GOOGL".to_string()]));
        assert_eq!(sub.trade_updates, Some(true));
    }

    #[test]
    fn test_trade_update_event_serialization() {
        let event = TradeUpdateEvent::Fill;
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, "\"fill\"");
    }

    #[test]
    fn test_connection_status_serialization() {
        let status = ConnectionStatus::Connected;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"connected\"");
    }
}
