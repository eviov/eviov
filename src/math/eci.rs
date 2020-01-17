use derive_more::{Add, Sub};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Add, Sub)]
pub struct Eci {
    position: Vector<Length>,
    velocity: Vector<Length>,
}
