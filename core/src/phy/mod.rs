//! The physics bundle.
//!
//! This bundle provides orbital logic and collision event dispatching.

mod body;
pub use body::{Body, Star};
pub mod collision;
mod orbit;
pub use orbit::{Orbit, OrbitalState};
mod index;
pub use index::BodyIndex;
