use std::time::Instant;

use crate::math::{Time, MILLIS_PER_TICK};

#[derive(Debug)] // This type deliberately does not implement Clone+Copy because of its highly mutable characteristics
pub struct CalibratedClock(Instant, Time);

impl CalibratedClock {
    pub fn average(send: Instant, recv: Instant, time: Time) -> Self {
        Self(send + (recv - send) / 2, time)
    }

    pub fn set_average(&mut self, send: Instant, recv: Instant, time: Time) {
        *self = Self::average(send, recv, time);
    }

    pub fn now(&self) -> Time {
        let start = self.0;
        let millis = start.elapsed().as_millis();
        let ticks = millis / MILLIS_PER_TICK as u128; // 50 ticks per second
        let base_time = self.1;
        Time(base_time.0 + ticks as i32) // a big truncation, can suffice 497 days
    }
}

// TODO sync with time signalling server
