use derive_more::{Add, Sub};
use num_derive::{Num, NumOps, One, Zero};
use serde::{Deserialize, Serialize};

/// The coordinated time in the game universe.
///
/// The coordinated time increments at a constant rate of one tick per `MILLIS_PER_TICK`
/// milliseconds.
/// Nodes should communicate via `proto::time` to synchronize the common game time in the universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Add, Sub)]
pub struct Time(pub i32);

/// The number of milliseconds per tick.
pub const MILLIS_PER_TICK: u32 = 20;

macro_rules! float_unit {
    ($($(#[$doc:meta])* $name:ident,)*) => { $(
        $(#[$doc])*
        #[derive(
            Debug, Clone, Copy, PartialEq,
            Serialize, Deserialize,
            Num, NumOps, Zero, One,
            //Add, Sub, Mul, Div, Rem, Num
        )]
        pub struct $name(pub f32);

        impl From<$name> for f32 {
            fn from(from: $name) -> f32 {
                from.0
            }
        }
    )* };
}

float_unit! {
    /// Represents a length value.
    Length,
    /// Represents a mass value.
    Mass,
}
