//! The eviov rendering bundle.

#![cfg_attr(debug_assertions, allow(unused_variables, dead_code, unreachable_code))]
#![warn(missing_docs)]

pub use amethyst;

mod camera;
pub use camera::Camera;

mod draw;
pub use draw::DrawSystem;
