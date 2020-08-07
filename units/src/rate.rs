use std::ops;

use serde::{Deserialize, Serialize};

use super::time::GameDuration;

/// The rate value of the wrapped unit `T`.
///
/// `T` is a relative value (such as `Displace` or `Theta`)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Rate<T>(
    /// The amount of change per `GameDuration` tick.
    pub T,
);

impl<T> Rate<T> {
    /// Constructs a Rate with `unit` per tick.
    pub fn of(unit: T) -> Self {
        Self(unit)
    }
}

impl<T> ops::Add<Rate<T>> for Rate<T>
where
    T: ops::Add<T, Output = T>,
{
    type Output = Rate<T>;

    fn add(self, other: Rate<T>) -> Rate<T> {
        Self(self.0 + other.0)
    }
}

impl<T> ops::Sub<Rate<T>> for Rate<T>
where
    T: ops::Sub<T, Output = T>,
{
    type Output = Rate<T>;

    fn sub(self, other: Rate<T>) -> Rate<T> {
        Self(self.0 - other.0)
    }
}

impl<T> Rate<T>
where
    T: ops::Div<f64, Output = T>,
{
    /// Computes the average rate over time.
    pub fn average(sum: T, time: GameDuration) -> Self {
        Self(sum / time.as_float())
    }
}

impl<T> Rate<T>
where
    T: ops::Mul<f64, Output = T>,
{
    /// Computes the summed value after `duration` has elapsed.
    pub fn after(self, duration: GameDuration) -> T {
        self.0 * (duration.0 as f64)
    }
}

impl<T> ops::Mul<f64> for Rate<T>
where
    T: ops::Mul<f64, Output = T>,
{
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other)
    }
}

impl<T> ops::Div<f64> for Rate<T>
where
    T: ops::Div<f64, Output = T>,
{
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other)
    }
}
