# alpaca-fix

FIX (Financial Information eXchange) protocol client for the Alpaca trading platform.

## Overview

`alpaca-fix` provides a FIX protocol implementation for high-frequency trading applications with Alpaca. It supports FIX 4.2 and 4.4 versions for order routing and market data.

## Features

- **FIX Session Management**: Session initiation, heartbeat handling, sequence number management
- **Order Routing**: New Order Single, Cancel, Cancel/Replace requests
- **Execution Reports**: Real-time order status updates
- **Market Data**: Market data requests and snapshots
- **Session Recovery**: Automatic sequence number recovery
- **Message Logging**: Configurable FIX message logging

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca-fix = "0.1.0"
```

## Usage

```rust
use alpaca_fix::{FixClient, FixConfig, FixVersion};
use alpaca_base::Credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::from_env()?;
    
    let config = FixConfig::builder()
        .version(FixVersion::Fix44)
        .sender_comp_id("YOUR_SENDER_ID")
        .target_comp_id("ALPACA")
        .host("fix.alpaca.markets")
        .port(5001)
        .build();
    
    let mut client = FixClient::new(credentials, config);
    client.connect().await?;
    
    // Send a new order
    let order = NewOrderSingle::market("AAPL", Side::Buy, 100);
    client.send_order(&order).await?;
    
    Ok(())
}
```

## API Reference

### Main Types

- `FixClient` - The primary client for FIX protocol connections
- `FixConfig` - Configuration for FIX sessions
- `FixVersion` - FIX protocol version (4.2, 4.4)

### Message Types

#### Order Messages
- `NewOrderSingle` (MsgType D) - New order request
- `OrderCancelRequest` (MsgType F) - Cancel order request
- `OrderCancelReplaceRequest` (MsgType G) - Modify order request

#### Execution Messages
- `ExecutionReport` (MsgType 8) - Order execution status
- `OrderCancelReject` (MsgType 9) - Cancel rejection

#### Market Data Messages
- `MarketDataRequest` (MsgType V) - Request market data
- `MarketDataSnapshot` (MsgType W) - Market data response

#### Session Messages
- `Logon` (MsgType A) - Session initiation
- `Logout` (MsgType 5) - Session termination
- `Heartbeat` (MsgType 0) - Keep-alive
- `TestRequest` (MsgType 1) - Connection test
- `ResendRequest` (MsgType 2) - Message recovery
- `SequenceReset` (MsgType 4) - Sequence reset

### Main Modules

- `client` - FIX client implementation
- `config` - Configuration types
- `messages` - FIX message types
- `session` - Session management
- `codec` - FIX message encoding/decoding

## FIX Field Tags

Common FIX tags used:

| Tag | Name | Description |
|-----|------|-------------|
| 8 | BeginString | FIX version |
| 9 | BodyLength | Message body length |
| 35 | MsgType | Message type |
| 49 | SenderCompID | Sender identifier |
| 56 | TargetCompID | Target identifier |
| 34 | MsgSeqNum | Message sequence number |
| 52 | SendingTime | Message timestamp |
| 10 | CheckSum | Message checksum |

## Configuration

```rust
use alpaca_fix::FixConfig;

let config = FixConfig::builder()
    .version(FixVersion::Fix44)
    .sender_comp_id("YOUR_SENDER_ID")
    .target_comp_id("ALPACA")
    .host("fix.alpaca.markets")
    .port(5001)
    .heartbeat_interval_secs(30)
    .reconnect_enabled(true)
    .reconnect_max_attempts(5)
    .message_logging(true)
    .build();
```

## Changelog

### v0.1.0 (latest)
- Initial FIX protocol implementation
- FIX 4.2 and 4.4 support
- Order routing messages
- Market data messages
- Session management
- Message encoding/decoding

## Notes

FIX protocol is intended for high-frequency trading applications. Most users should use the REST API (`alpaca-http`) or WebSocket API (`alpaca-websocket`) instead.

## License

MIT
