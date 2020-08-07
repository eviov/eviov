use getset::*;

/// The camera resource.
#[derive(Getters, Setters, MutGetters, CopyGetters)]
pub struct Camera {
    /// The star system the camera is focused in.
    #[getset(get_copy = "pub", set = "pub")]
    star: specs::Entity,
    /// The camera position in the star system.
    #[getset(get_copy = "pub", set = "pub")]
    position: units::Position,
    /// The bearing of the camera.
    #[getset(get_copy = "pub", set = "pub")]
    bearing: units::Bearing,
    /// The in-game width of the screen.
    #[getset(get_copy = "pub", set = "pub")]
    width: units::Length,
}
