use crate::units;

/// The camera resource.
pub struct Camera {
    /// The location of the camera.
    pub address: units::Address,
    /// The bearing of the camera.
    pub bearing: units::Bearing,
    /// The in-game width of the screen.
    pub width: units::Length,
}
