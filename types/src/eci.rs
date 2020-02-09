use derive_more::{Add, Sub};
use serde::{Deserialize, Serialize};

use getset::*;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Add, Sub, CopyGetters)]
pub struct Eci {
    #[get_copy = "pub"]
    position: Vector<Length>,
    #[get_copy = "pub"]
    velocity: Vector<Length>,
}
