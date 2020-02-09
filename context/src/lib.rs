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

use std::time::Duration;

use async_trait::async_trait;
use derive_more::From;
use futures::future::{self, Either, Future, FutureExt};

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "not-wasm")]
pub mod tokio;

#[async_trait]
pub trait ContextImpl: Sized + Send + Sync + 'static {
    fn spawn_future<F: Future<Output = ()> + Send + 'static>(&self, fut: F);

    async fn sleep(&self, duration: Duration);
}

#[derive(From)]
pub struct Context<C: ContextImpl>(C);

impl<C: ContextImpl> Context<C> {
    pub fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        self.0.spawn_future(fut);
    }

    pub async fn sleep(&self, duration: Duration) {
        self.0.sleep(duration).await;
    }

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
