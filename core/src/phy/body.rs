use amethyst::ecs;

use super::{Orbit, OrbitIndex};
use crate::units;

/// A gravity-dependent object in a solar system.
#[derive(Debug, Clone)]
pub enum Body {
    /// The root star, which has no parent star
    Root,
    /// An orbiting object.
    Orbiting {
        /// The orbit of the body in its parent system.
        initial: Orbit,
        /// The entity of the star this body orbits about.
        parent: ecs::Entity,
    },
    /// An object standing on a star.
    Standing {
        /// The current position of the body.
        position: units::Position,
        /// The entity of the star this body stands on.
        parent: ecs::Entity,
    },
}

impl Body {
    /// Retrieves the parent of this body.
    ///
    /// Returns `None` if this body is the root star.
    pub fn parent(&self) -> Option<ecs::Entity> {
        match self {
            Self::Root => None,
            Self::Orbiting { parent, .. } => Some(*parent),
            Self::Standing { parent, .. } => Some(*parent),
        }
    }
}

impl ecs::Component for Body {
    type Storage = ecs::storage::VecStorage<Self>;
}

/// An object with a non-negligible gravitational field.
#[derive(Debug)]
pub struct Star {
    /// Radius of the effective gravitational field.
    ///
    /// If an object moves beyond this radius, it is regarded as "out of" the current star system.
    ///
    /// The root star system also has a finite field radius.
    /// Bodies moving out of the field radius would trigger a `BodyEvent::Void`.
    pub field_radius: units::Length,

    /// Mass of the star, used for orbit calculation.
    pub strength: units::Mass,

    /// Index of bodies in this star.
    index: OrbitIndex,
}

impl ecs::Component for Star {
    type Storage = ecs::storage::BTreeStorage<Self>;
}
