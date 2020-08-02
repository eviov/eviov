//! Handles collision of bounding boxes
//! and maintains `OrbitIndex`s.

pub mod bb;
pub use bb::BoundingBox;

mod event;
pub use event::Event;

mod system;
pub use system::System;
