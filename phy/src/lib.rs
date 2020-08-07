//! The physics bundle.
//!
//! This bundle provides orbital logic and collision event dispatching.

#![cfg_attr(debug_assertions, allow(unused_variables, dead_code, unreachable_code))]
#![warn(missing_docs)]

mod body;
pub use body::{Body, Star};
pub mod collision;
mod orbit;
pub use orbit::{Orbit, OrbitalState};
mod index;
pub use index::BodyIndex;
