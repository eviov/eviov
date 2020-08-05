use serde::{Deserialize, Serialize};

use super::{Mass, Accel, Velocity, Displace};
use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct Force(pub Accel);

add_newtype!(Force, Force);
sub_newtype!(Force, Force);

mul_newtype!(Accel, Mass -> Force);

impl Force {
    pub fn on(self, mass: Mass) -> Self {
        Self(self.0 / mass.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct Torque(pub f64);

impl From<(Displace, Force)> for Torque {
    fn from((displace, force): (Displace, Force)) -> Self {
        Self(util::cross2d(displace, force))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct Momentum(pub Velocity);

add_newtype!(Momentum, Momentum);
sub_newtype!(Momentum, Momentum);

mul_newtype!(Mass, Velocity -> Momentum);
