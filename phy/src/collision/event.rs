use getset::*;

use super::BoundingBox;

/// The data for collision events.
///
/// Use `Read<'a, EventChannel<CollisionEvent>>` to handle collision events.
pub enum Event {
    /// Two bounding boxes (BBs) intersect.
    ///
    /// This event is only dispatched for BBs belonging to different parent bodies.
    Intersect {
        /// The entity of the star system in which this collision is handled within.
        ///
        /// All positions in this event are relative to this star entity.
        star: specs::Entity,
        /// The parties participating in the collision.
        parties: [CollisionParty; 2],
    },
    /// An entity escapes the g-field of a star.
    Void {
        /// The entity entering void zone
        ///
        /// This entity must contain a `Body` component.
        subject: specs::Entity,

        /// Position of the entity
        position: units::Position,
        /// Velocity of the entity
        velocity: units::Velocity,
    },
}

/// A participating party in a collision.
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct CollisionParty {
    /// The bounding box in this party.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    bb: BoundingBox,
    /// The position of this party relative to the star.
    #[getset(get_copy = "pub", set = "pub")]
    position: units::Position,
    /// The velocity of this party relative to the star.
    #[getset(get_copy = "pub", set = "pub")]
    velocity: units::Velocity,
    /// The site of collision relative to `self.position`.
    #[getset(get_copy = "pub", set = "pub")]
    collision: units::Displace,
}
