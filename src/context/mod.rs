use std::time::Duration;

use async_trait::async_trait;
use derive_more::From;
use futures::future::{self, Either, Future, FutureExt};

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
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
