#![feature(option_expect_none)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(
    missing_docs,
    unused_results,
    unused_qualifications,
    variant_size_differences,
    clippy::checked_conversions,
    clippy::needless_borrow,
    clippy::shadow_unrelated,
    clippy::wrong_pub_self_convention
)]
#![deny(
    anonymous_parameters,
    bare_trait_objects,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::float_cmp_const,
    clippy::if_not_else,
    clippy::indexing_slicing,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![cfg_attr(not(debug_assertions), deny(warnings, clippy::dbg_macro,))]

//! eviov communications framework

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

/// The maximum query pool size for each connection.
pub const MAX_QUERY_POOL_SIZE: usize = 1000;

/// The duration to wait for before a query pool timeouts.
pub const OPEN_CONN_TIMEOUT: Duration = Duration::from_secs(10);

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
