use std::ops;

use serde::{Deserialize, Serialize};

use super::time::GameDuration;

/// The rate value of the wrapped unit `T`.
///
/// `T` is a relative value (such as `Displace` or `Theta`)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Rate<T> {
    /// The amount of change per `GameDuration` tick.
    pub unit: T,
}

impl<T> Rate<T> {
    /// Constructs a Rate with `unit` per tick.
    pub fn of(unit: T) -> Self {
        Self { unit }
    }
}

impl<T> Rate<T>
where
    T: ops::Div<f64, Output = T>,
{
    /// Computes the average rate over time.
    pub fn average(sum: T, time: GameDuration) -> Self {
        Self {
            unit: sum / time.as_float(),
        }
    }
}

impl<T> Rate<T>
where
    T: ops::Mul<f64, Output = T>,
{
    /// Computes the summed value after `duration` has elapsed.
    pub fn after(self, duration: GameDuration) -> T {
        self.unit * (duration.0 as f64)
    }
}

impl<T> ops::Mul<f64> for Rate<T>
where
    T: ops::Mul<f64, Output = T>,
{
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            unit: self.unit * other,
        }
    }
}
