use derive_more::{Add, Sub};
use serde::{Deserialize, Serialize};

use getset::*;

use super::*;

/// Denotes the ECI position and velocity of an object.
///
/// While `Eci` itself does not store data about its relative parent,
/// code using `Eci` should document clearly what this `Eci` corresponds to.
#[derive(Debug, Clone, Serialize, Deserialize, Add, Sub, CopyGetters)]
pub struct Eci {
    /// The position value
    #[get_copy = "pub"]
    position: Vector<Length>,
    /// The velocity value
    #[get_copy = "pub"]
    velocity: Vector<Length>,
}
