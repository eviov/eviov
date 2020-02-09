use std::fmt;
use std::time::Duration;

use async_trait::async_trait;

mod router;
pub use router::*;

#[cfg(feature = "not-wasm")]
mod tung;
#[cfg(feature = "not-wasm")]
pub use tung::*;

#[cfg(feature = "wasm")]
mod stdweb;
#[cfg(feature = "wasm")]
pub use self::stdweb::*;

#[async_trait]
pub trait WsClient: Sized + fmt::Debug {
    async fn open(server: &str) -> Result<Self, String>;

    async fn send_message(&self, message: &[u8]) -> Result<(), String>;

    async fn await_message(&self, time: Duration) -> Result<Option<Vec<u8>>, String>;

    async fn close(&self);
}
