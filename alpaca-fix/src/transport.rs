//! TCP transport layer for FIX protocol.

use crate::codec::{FixDecoder, FixMessage, SOH};
use crate::error::{FixError, Result};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

/// Buffer size for reading FIX messages.
const READ_BUFFER_SIZE: usize = 8192;

/// TCP transport for FIX messages.
#[derive(Debug)]
pub struct FixTransport {
    /// TCP stream reader.
    reader: Arc<Mutex<BufReader<tokio::net::tcp::OwnedReadHalf>>>,
    /// TCP stream writer.
    writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    /// Message decoder.
    decoder: FixDecoder,
    /// Read buffer.
    buffer: Arc<Mutex<String>>,
}

impl FixTransport {
    /// Create a new transport from a TCP stream.
    #[must_use]
    pub fn new(stream: TcpStream) -> Self {
        let (read_half, write_half) = stream.into_split();
        Self {
            reader: Arc::new(Mutex::new(BufReader::with_capacity(
                READ_BUFFER_SIZE,
                read_half,
            ))),
            writer: Arc::new(Mutex::new(write_half)),
            decoder: FixDecoder::new(),
            buffer: Arc::new(Mutex::new(String::with_capacity(READ_BUFFER_SIZE))),
        }
    }

    /// Send a FIX message.
    ///
    /// # Arguments
    /// * `message` - Raw FIX message string
    ///
    /// # Errors
    /// Returns error if write fails.
    pub async fn send(&self, message: &str) -> Result<()> {
        let mut writer = self.writer.lock().await;
        writer
            .write_all(message.as_bytes())
            .await
            .map_err(|e| FixError::Connection(format!("write error: {}", e)))?;
        writer
            .flush()
            .await
            .map_err(|e| FixError::Connection(format!("flush error: {}", e)))?;

        tracing::debug!("Sent FIX message: {} bytes", message.len());
        Ok(())
    }

    /// Receive a FIX message.
    ///
    /// # Errors
    /// Returns error if read fails or message is invalid.
    pub async fn receive(&self) -> Result<FixMessage> {
        let mut reader = self.reader.lock().await;
        let mut buffer = self.buffer.lock().await;

        // Read until we have a complete message (ends with checksum field)
        loop {
            let mut line = String::new();
            let bytes_read = reader
                .read_line(&mut line)
                .await
                .map_err(|e| FixError::Connection(format!("read error: {}", e)))?;

            if bytes_read == 0 {
                return Err(FixError::Connection("connection closed".to_string()));
            }

            buffer.push_str(&line);

            // Check if we have a complete message (contains checksum tag 10=)
            if buffer.contains(&format!("{}10=", SOH)) {
                // Find the end of the message (after checksum)
                if let Some(pos) = buffer.find(&format!("{}10=", SOH)) {
                    // Find the SOH after checksum value
                    if let Some(end_pos) = buffer[pos + 4..].find(SOH) {
                        let msg_end = pos + 4 + end_pos + 1;
                        let raw_msg = buffer[..msg_end].to_string();
                        buffer.drain(..msg_end);

                        tracing::debug!("Received FIX message: {} bytes", raw_msg.len());
                        return self.decoder.decode(&raw_msg);
                    }
                }
            }

            // Prevent buffer from growing too large
            if buffer.len() > READ_BUFFER_SIZE * 4 {
                buffer.clear();
                return Err(FixError::Decoding("message too large".to_string()));
            }
        }
    }

    /// Check if the transport is connected.
    ///
    /// Note: This is a best-effort check and may not detect all disconnection scenarios.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        // We can't easily check TCP connection state without trying to use it
        // This is a placeholder - actual connection state is determined by I/O operations
        true
    }

    /// Close the transport.
    ///
    /// # Errors
    /// Returns error if shutdown fails.
    pub async fn close(&self) -> Result<()> {
        let mut writer = self.writer.lock().await;
        writer
            .shutdown()
            .await
            .map_err(|e| FixError::Connection(format!("shutdown error: {}", e)))?;
        Ok(())
    }
}

/// Connect to a FIX server.
///
/// # Arguments
/// * `host` - Server hostname
/// * `port` - Server port
///
/// # Errors
/// Returns error if connection fails.
pub async fn connect(host: &str, port: u16) -> Result<FixTransport> {
    let addr = format!("{}:{}", host, port);
    tracing::info!("Connecting to FIX server at {}", addr);

    let stream = TcpStream::connect(&addr)
        .await
        .map_err(|e| FixError::Connection(format!("failed to connect to {}: {}", addr, e)))?;

    // Set TCP options
    stream
        .set_nodelay(true)
        .map_err(|e| FixError::Connection(format!("failed to set nodelay: {}", e)))?;

    tracing::info!("Connected to FIX server at {}", addr);
    Ok(FixTransport::new(stream))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_transport_send_receive() {
        // Start a mock server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn server task
        let server_handle = tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let (read_half, mut write_half) = socket.into_split();
            let mut reader = BufReader::new(read_half);

            // Read message from client
            let mut buf = String::new();
            reader.read_line(&mut buf).await.unwrap();

            // Echo back a response
            let response = "8=FIX.4.4\x0135=0\x0110=000\x01";
            write_half.write_all(response.as_bytes()).await.unwrap();
        });

        // Connect client
        let transport = connect("127.0.0.1", addr.port()).await.unwrap();

        // Send a message
        let msg = "8=FIX.4.4\x0135=0\x0110=000\x01\n";
        transport.send(msg).await.unwrap();

        // Wait for server
        server_handle.await.unwrap();
    }
}
