use derive_more::{Add, Sub};
use num_derive::{Num, NumOps, One, Zero};
use serde::{Deserialize, Serialize};

dirmod::all!(default file pub use; default dir pub);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Add, Sub)]
pub struct Time(pub i32);

pub const MILLIS_PER_TICK: i32 = 20;

macro_rules! float_unit {
    ($($name:ident,)*) => { $(
        #[derive(
            Debug, Clone, Copy, PartialEq,
            Serialize, Deserialize,
            Num, NumOps, Zero, One,
            //Add, Sub, Mul, Div, Rem, Num
        )]
        pub struct $name(f32);

        impl From<$name> for f32 {
            fn from(from: $name) -> f32 {
                from.0
            }
        }
    )* };
}

float_unit! {
    Length,
    Mass,
}
