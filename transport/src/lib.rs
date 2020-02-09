#![feature(option_expect_none)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(
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

pub const MAX_QUERY_POOL_SIZE: usize = 1000;

pub const OPEN_CONN_TIMEOUT: Duration = Duration::from_secs(10);

#[async_trait]
pub trait WsClient: Sized + fmt::Debug {
    async fn open(server: &str) -> Result<Self, String>;

    async fn send_message(&self, message: &[u8]) -> Result<(), String>;

    async fn await_message(&self, time: Duration) -> Result<Option<Vec<u8>>, String>;

    async fn close(&self);
}
