use num_derive::{Num, NumOps, Zero, One};
use serde::{Deserialize, Serialize};

dirmod::all!(default file pub use; default dir pub);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time(pub i32);

macro_rules! float_unit {
    ($($name:ident,)*) => { $(
        #[derive(
            Debug, Clone, Copy, PartialEq,
            Serialize, Deserialize,
            Num, NumOps, Zero, One
            //Add, Sub, Mul, Div, Rem, Num
        )]
        pub struct $name(f32);
    )* };
}

float_unit! {
    Length,
    Mass,
}
