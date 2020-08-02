use super::time::GameDuration;

/// The rate value of the wrapped unit `T`.
///
/// `T` is a relative value (such as `Displace` or `Theta`)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
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
    T: std::ops::Mul<f64, Output = T>,
{
    /// Computes the summed value after `duration` has elapsed.
    pub fn after(self, duration: GameDuration) -> T {
        self.unit * (duration.0 as f64)
    }
}
