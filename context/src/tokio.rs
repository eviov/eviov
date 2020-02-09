use std::time::Duration;

use async_trait::async_trait;
use futures::future::Future;

use crate::ContextImpl;

pub struct TokioContext;

#[async_trait]
impl ContextImpl for TokioContext {
    fn spawn_future<F: Future<Output = ()> + Send + 'static>(&self, fut: F) {
        let _ = tokio::spawn(fut); // we don't need to join the result
    }

    async fn sleep(&self, duration: Duration) {
        unimplemented!()
    }
}
