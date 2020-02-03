use std::fmt;
use std::time::Duration;

use async_trait::async_trait;

mod router;
pub use router::*;

#[cfg(not(target_arch = "wasm32"))]
mod tung;
#[cfg(not(target_arch = "wasm32"))]
pub use tung::*;

#[cfg(target_arch = "wasm32")]
mod stdweb;
#[cfg(target_arch = "wasm32")]
pub use self::stdweb::*;

#[async_trait]
pub trait WsClient: Sized + fmt::Debug {
    async fn open(server: &str) -> Result<Self, String>;

    async fn send_message(&self, message: &[u8]) -> Result<(), String>;

    async fn await_message(&self, time: Duration) -> Result<Option<Vec<u8>>, String>;

    async fn close(&self);
}
