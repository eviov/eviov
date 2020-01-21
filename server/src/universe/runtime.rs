use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crossbeam::sync::ShardedLock;
use derive_more::Into;
use eviov::math::{Time, MILLIS_PER_TICK};
use eviov::CalibratedClock;
use futures::future::Future;

use super::*;

#[derive(Debug)]
pub struct Runtime<X: system::Extra>(Arc<Inner<X>>);

impl<X: system::Extra> Clone for Runtime<X> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[derive(Debug)]
struct Inner<X: system::Extra> {
    id: CurrentRuntimeId,
    counter: AtomicU32,
    db: Db,
    systems: ShardedLock<HashMap<system::Tag, system::Handle<X::Message>>>,
    time: ShardedLock<CalibratedClock>,
}

impl<X: system::Extra> Runtime<X> {
    pub fn new() -> Self {
        Self(Arc::new(Inner {
            id: CurrentRuntimeId(RuntimeId(rand::random())),
            counter: AtomicU32::default(),
            db: Db,
            systems: Default::default(),
            time: unimplemented!(),
        }))
    }

    pub fn id(&self) -> CurrentRuntimeId {
        self.0.id
    }

    pub fn next_id(&self) -> u32 {
        self.0.counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn current_time(&self) -> Time {
        self.0
            .time
            .read() // TODO check if the blocking logic here is correct
            .unwrap() // if a write panicked, we can't really do anything; relying on a partially updated value is even more dangerous
            .now()
    }

    pub async fn time_future<T>(&self, time: Time, task: impl Future<Output = T>) -> T {
        let remain = time - self.current_time();
        // TODO assess whether we need to take the cost calibrating the sleep
        let millis = remain.0 * MILLIS_PER_TICK;
        tokio::time::delay_for(Duration::from_millis(millis as u64)).await;
        task.await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RuntimeId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Into)]
pub struct CurrentRuntimeId(RuntimeId);
