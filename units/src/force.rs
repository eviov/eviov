use serde::{Deserialize, Serialize};

use super::{Accel, Displace, Mass, Velocity};

/// A force vector type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Force(pub Accel);

add_newtype!(Force, Force);
sub_newtype!(Force, Force);

impl Force {
    /// Computes the acceleration when this force is applied on an object of the specified mass.
    pub fn on(self, mass: Mass) -> Accel {
        self.0 / mass.0
    }
}

/// Represents torque in the counterclockwise direction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Torque(pub f64);

impl From<(Displace, Force)> for Torque {
    fn from((displace, force): (Displace, Force)) -> Self {
        Self(util::cross2d(displace.0, force.0 .0 .0 .0))
    }
}

/// A momentum vector type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Momentum(pub Velocity);

add_newtype!(Momentum, Momentum);
sub_newtype!(Momentum, Momentum);
