/// A monotonic 100Hz clock in the game system.
///
/// This clock can represent the time for about 32 months from epoch.
///
/// Typically, the epoch is reset every time the universe is reloaded.
/// This means all values in RAM using an epoch-dependent form
/// need to be serialized as an epoch-independent form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameInstant(pub u32);

impl GameInstant {
    /// The epoch refrence frame.
    ///
    /// All game instants must be beyond this epoch.
    pub const EPOCH: GameInstant = GameInstant(0);
}

/// A non-negative difference between two `GameInstant`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GameDuration(pub u32);

impl GameDuration {
    /// The smallest discrete duration unit.
    pub const UNIT: GameDuration = GameDuration(1);

    /// Expresses the duration in seconds.
    pub fn as_secs(self) -> f64 {
        (self.0 as f64) / 100.
    }
}

impl From<GameDuration> for std::time::Duration {
    fn from(gd: GameDuration) -> Self {
        Self::from_millis((gd.0 as u64) * 10)
    }
}

add_newtype!(GameDuration, GameDuration);
sub_newtype!(GameDuration, GameDuration);
add_newtype!(GameInstant, GameDuration);
sub_newtype!(GameInstant, GameDuration);
mul_raw!(GameDuration, u32);

impl std::ops::Sub<GameInstant> for GameInstant {
    type Output = GameDuration;

    fn sub(self, other: Self) -> GameDuration {
        GameDuration(self.0 - other.0)
    }
}
