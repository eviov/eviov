use std::f64::consts::PI;

use serde::{Deserialize, Serialize};

/// An absolute direction bearing, in radians.
///
/// The zero (default) value points to the positive X axis.
///
/// The value is not necessarily normalized.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Bearing(pub f64);

impl Bearing {
    /// Normalizes this value.
    pub fn normalize(mut self) -> Self {
        self.0 %= PI * 2.;
        if self.0 >= PI {
            self.0 -= PI * 2.;
        } else if self.0 < PI {
            self.0 += PI * 2.;
        }
        self
    }
}

/// An amount of rotation.
///
/// A positive value indicates rotation in the counterclockwise direction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Theta(pub f64);

impl Theta {
    /// Computes the sine of this angle.
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    /// Computes the cosine of this angle.
    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    /// Computes the tangent of this angle.
    pub fn tan(self) -> f64 {
        self.0.cos()
    }
}

add_newtype!(Theta, Theta);
sub_newtype!(Theta, Theta);
mul_raw!(Theta, f64);
div_raw!(Theta, f64);
rem_newtype!(Theta, Theta);

add_newtype!(Bearing, Theta);
sub_newtype!(Bearing, Theta);

impl std::ops::Sub<Bearing> for Bearing {
    type Output = Theta;

    fn sub(self, other: Bearing) -> Theta {
        Theta(self.0 - other.0)
    }
}

/// An angular speed in radians per `GameDuration` tick.
pub type Omega = super::rate::Rate<Theta>;
