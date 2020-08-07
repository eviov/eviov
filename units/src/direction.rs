use std::f64::consts::PI;

use super::angle::Bearing;
use super::length::Displace;
use nalgebra::Vector2;

/// A value that represents a direction.
pub trait Direction {
    /// Expresses this direction as a bearing.
    fn to_normal_bearing(&self) -> Bearing;

    /// Expresses this direction as a unit vector.
    fn to_unit_vector(&self) -> Displace;
}

impl Direction for Displace {
    fn to_normal_bearing(&self) -> Bearing {
        let mut angle = self.0[1].atan2(self.0[0]);
        if angle >= PI {
            angle -= PI * 2.;
        }
        Bearing(angle)
    }

    fn to_unit_vector(&self) -> Displace {
        Displace(self.0.normalize())
    }
}

impl Direction for Bearing {
    fn to_normal_bearing(&self) -> Bearing {
        self.clone().normalize()
    }

    fn to_unit_vector(&self) -> Displace {
        Displace(Vector2::new(self.0.cos(), self.0.sin()))
    }
}
