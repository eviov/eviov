//! The core library for eviov.
//!
//! This library provides basic simulation and rendering features.

#![cfg_attr(debug_assertions, allow(unused_variables, dead_code, unreachable_code))]
#![warn(missing_docs)]

pub use amethyst::{self, core::math as nalgebra, ecs};

pub mod phy;
pub mod render;
pub mod save;
pub mod units;
pub mod util;
