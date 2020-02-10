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

//! Platform-dependent (wasm/native) adapters.

use std::time::Duration;

use async_trait::async_trait;
use derive_more::From;
use futures::future::{self, Either, Future, FutureExt};

/// Implementation for client crates compiling to WASM.
#[cfg(feature = "wasm")]
mod wasm;
#[cfg(feature = "wasm")]
pub use wasm::*;

#[cfg(feature = "not-wasm")]
mod tokio;
#[cfg(feature = "not-wasm")]
pub use self::tokio::*;

/// Context-dependent primitives.
#[async_trait]
pub trait ContextImpl: Sized + Send + Sync + 'static {
    /// Schedules a future to be executed non-blocking.
    fn spawn_future<F: Future<Output = ()> + Send + 'static>(&self, fut: F);

    /// Returns a future that sleeps for the specified duration.
    async fn sleep(&self, duration: Duration);
}

/// User wrapper for `ContextImpl`.
#[derive(From)]
pub struct Context<C: ContextImpl>(C);

impl<C: ContextImpl> Context<C> {
    /// Schedules a future to be executed non-blocking.
    pub fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        self.0.spawn_future(fut);
    }

    /// Returns a future that sleeps for the specified duration.
    pub async fn sleep(&self, duration: Duration) {
        self.0.sleep(duration).await;
    }

    /// Executes a future within the specified duration and returns its result, or return the
    /// future if the duration expires.
    pub async fn timeout<T, F: Future<Output = T> + Unpin>(
        &self,
        duration: Duration,
        fut: F,
    ) -> Result<T, F> {
        match future::select(self.sleep(duration).boxed(), fut).await {
            Either::Left((_, fut)) => Err(fut),
            Either::Right((t, _)) => Ok(t),
        }
    }
}
