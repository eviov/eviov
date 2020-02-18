use std::fmt;
use std::time::Duration;

use async_trait::async_trait;

/// The abstraction for a network-capable websocket client.
#[async_trait]
pub trait WsClient: Sized + fmt::Debug {
    /// Opens the connection to the server.
    async fn open(server: &str) -> Result<Self, String>;

    /// Sends a message to the peer.
    async fn send_message(&self, message: &[u8]) -> Result<(), String>;

    /// Receives a message from the peer for the specified duration.
    async fn await_message(&self, time: Duration) -> Result<Option<Vec<u8>>, String>;

    /// Closes the conneciton.
    async fn close(&self);
}

#[cfg(feature = "not-wasm")]
mod tung;
#[cfg(feature = "not-wasm")]
pub use tung::*;

#[cfg(feature = "wasm")]
mod stdweb;
#[cfg(feature = "not-wasm")]
pub use stdweb::*;
