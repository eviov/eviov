use amethyst::ecs;

use crate::units;

/// A unique address identifier for a world.
pub struct Address {
    /// The entity of the star this addres resides in
    pub star: ecs::Entity,
    /// The position in the `star` star system
    pub position: units::Position,
}
