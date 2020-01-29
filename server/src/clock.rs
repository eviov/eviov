use std::env;

use async_trait::async_trait;
use eviov::math::Time;
use eviov::{AlwaysZeroTimeSource, TimeSource};
use parking_lot::RwLock;

pub type Clock = eviov::Clock<RwLock<eviov::ClockInner>>;

pub async fn create_clock() -> (Clock, Option<UrlTimeSource<(), ()>>) {
    match env::var("TIME_SRC") {
        Ok(url) => unimplemented!(),
        Err(_) => (Clock::new(&mut AlwaysZeroTimeSource).await.unwrap(), None),
    }
}

pub struct UrlTimeSource<A, B>(A, B)
where
    A: Unpin + Send + Sync,
    B: Unpin + Send + Sync;

#[async_trait]
impl<A, B> TimeSource for UrlTimeSource<A, B>
where
    A: Unpin + Send + Sync,
    B: Unpin + Send + Sync,
{
    async fn fetch_time(&mut self) -> Option<Time> {
        unimplemented!()
    }
}
