use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use futures::future::Future;

use super::Lock;
use crate::math::{Time, MILLIS_PER_TICK};

#[derive(Debug, Clone)]
pub struct ClockInner(Instant, Time);

impl ClockInner {
    fn average(send: Instant, recv: Instant, time: Time) -> Self {
        Self(send + (recv - send) / 2, time)
    }

    pub fn now(&self) -> Time {
        let start = self.0;
        let millis = start.elapsed().as_millis();
        let ticks = millis / MILLIS_PER_TICK as u128; // 50 ticks per second
        let base_time = self.1;
        Time(base_time.0 + ticks as i32) // a big truncation, can suffice 497 days
    }
}

#[derive(Debug)]
pub struct Clock<L: for<'t> Lock<'t, ClockInner>> {
    clock: Arc<L>,
}

impl<L: for<'t> Lock<'t, ClockInner>> Clone for Clock<L> {
    fn clone(&self) -> Self {
        Self {
            clock: Arc::clone(&self.clock),
        }
    }
}

impl<L: for<'t> Lock<'t, ClockInner>> Clock<L> {
    pub async fn new(src: &mut impl TimeSource) -> Option<Self> {
        let send = Instant::now();
        let time = src.fetch_time().await?;
        let recv = Instant::now();
        let clock = ClockInner::average(send, recv, time);
        let lock = <L as Lock<ClockInner>>::new(clock);
        Some(Self {
            clock: Arc::new(lock),
        })
    }

    pub fn now(&self) -> Time {
        let clock = {
            let read = self.clock.read();
            read.clone() // minimize locking time by first copying out the clock
        };
        clock.now()
    }

    pub async fn maintain<F: Future<Output = LoopAction>>(
        &self,
        mut src: impl TimeSource,
        mut sleep: impl FnMut() -> F,
    ) -> ClockMaintain {
        loop {
            let send = Instant::now();
            let time = match src.fetch_time().await {
                Some(time) => time,
                None => return ClockMaintain::Error,
            };
            let recv = Instant::now();
            let clock = ClockInner::average(send, recv, time);
            {
                let mut guard = self.clock.write();
                *guard = clock;
            }

            let action = sleep().await;
            if action == LoopAction::Break {
                return ClockMaintain::Break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockMaintain {
    Break,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopAction {
    Break,
    Continue,
}

#[async_trait]
pub trait TimeSource: Sized + Send + Sync {
    async fn fetch_time(&mut self) -> Option<Time>;
}

#[derive(Debug)]
pub struct AlwaysZeroTimeSource;

#[async_trait]
impl TimeSource for AlwaysZeroTimeSource {
    async fn fetch_time(&mut self) -> Option<Time> {
        Some(Time(0))
    }
}

pub mod time_proto {
    use serde::{Deserialize, Serialize};

    use crate::math::Time;

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub id: u64,
        pub time: Time,
    }
}
