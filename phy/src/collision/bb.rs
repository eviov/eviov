//! Bounding boxes

/// One of the bounding boxes of a body.
///
/// A body may have multiple BBs,
/// e.g. atmosphere and land.
/// Each such BB is represented by an entity
/// with a `BoundingBox` component.
#[derive(Debug)]
pub struct BoundingBox {
    /// The shape of the BB.
    shape: Shape,
    /// The behavioural variant of the BB.
    variant: Variant,
    /// The parent entity of the BB.
    ///
    /// The parent must have a `Body` component.
    parent: specs::Entity,
    /// The position of the BB relative to the parent.
    offset: units::Displace,
}

/// The shape of a bounding box (BB).
///
/// BB shape affects the following:
/// - collision between BBs
/// - drag effect of fluids on the body of this BB
#[derive(Debug)]
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
        todo!()
    }
}

/// The variant of a bounding box (BB).
///
/// This determines how the BB can affect the colliding object.
#[derive(Debug)]
pub enum Variant {
    /// A solid BB.
    ///
    /// Colliding objects are immediately deflected using collision physics.
    Solid(units::Elasticity),

    /// A fluid BB.
    ///
    /// Colliding objects are constantly dragged
    /// in a direction based on the velocity of the fluid body.
    Fluid(units::Drag),
    // maybe magnetic field etc in the future?
}

impl specs::Component for BoundingBox {
    type Storage = specs::storage::VecStorage<Self>;
}
