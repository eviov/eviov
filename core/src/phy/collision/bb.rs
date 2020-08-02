//! Bounding boxes

use amethyst::ecs;

use crate::units;

/// One of the bounding boxes of a body.
///
/// A body may have multiple bounding boxes,
/// e.g. atmosphere and land.
/// Each such bounding box is represented by an entity
/// with a `BoundingBox` component.
pub struct BoundingBox {
    shape: Shape,
    variant: Variant,
}

/// The shape of a bounding box (BB).
///
/// BB shape affects the following:
/// - collision between BBs
/// - drag effect of fluids on the body of this BB
#[derive(Debug, Clone)]
pub enum Shape {
    /// A point BB.
    ///
    /// Never collides with another point BB.
    /// Collides with any other BB when strictly inside.
    Point,

    /// A circle BB.
    ///
    /// Collides with another circle BB when distance is strictly less than sum of radii.
    Circle {
        /// Radius of the circle
        radius: units::Length,
    },
}

impl Shape {
    /// Computes the inscribing circular BB for quick collision filtering.
    pub fn radius(&self) -> units::Length {
        match self {
            Self::Point => units::Length::default(),
            Self::Circle { radius } => *radius,
        }
    }

    /// Computes the drag effect of this shape against the given direction
    pub fn drag(&self, direction: impl units::Direction) -> f32 {
        unimplemented!()
    }
}

/// The variant of a bounding box (BB).
///
/// This determines how the bounding box can affect the colliding object.
#[derive(Debug, Clone)]
pub enum Variant {
    /// A solid bounding box.
    ///
    /// Colliding objects are immediately deflected using collision physics.
    Solid(units::Elasticity),

    /// A fluid bounding box.
    ///
    /// Colliding objects are constantly dragged
    /// in a direction based on the velocity of the fluid body.
    Fluid(units::Drag),
    // maybe magnetic field etc in the future?
}

impl ecs::Component for BoundingBox {
    type Storage = ecs::storage::VecStorage<Self>;
}
