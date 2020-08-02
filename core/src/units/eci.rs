use super::{Position, Theta, Velocity};

/// Indicates that the wrapped type is Earth-centered Earth-fixed (ECEF).
///
/// Position and angle objects are assumed to be Earth-centered inertial (ECI)
/// unless explicitly wrapped with this type.
pub struct Ecef<T: Eci> {
    /// the original type.
    pub inner: T,
}

/// Structs implementing this trait may represent either ECI or ECEF attributes.
///
/// By default, the struct represents ECI attributes.
/// Wrap with the `Ecef` newtype struct if it is intended to represent ECEF data.
pub trait Eci {}

impl Eci for Position {}

impl Eci for Velocity {}

impl Eci for Theta {}
