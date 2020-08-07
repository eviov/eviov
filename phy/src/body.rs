use getset::*;

use super::{BodyIndex, Orbit};

/// A gravity-dependent object in a solar system.
#[derive(Debug)]
pub enum Body {
    /// The root star, which has no parent star
    Root(RootBody),
    /// An orbiting object.
    Orbiting(OrbitingBody),
    /// An object accelerating due to forces other than gravitation.
    Accelerating(AccelBody),
    /// An object standing on a star.
    Standing(StandingBody),
}

/// A root star, which has no parent star.
///
/// There might be multiple root stars in a
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct RootBody;

/// An orbiting object.
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct OrbitingBody {
    /// The orbit of the body in its parent system.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    orbit: Orbit,
    /// The entity of the star this body orbits about.
    #[getset(get_copy = "pub", set = "pub")]
    parent: specs::Entity,
}

/// An object accelerating due to forces other than gravitation.
///
/// The acceleration of an object is represented by other components,
/// which modify the velocity every `GameDuration` tick.
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct AccelBody {
    /// Position of the object relative to `parent`.
    #[getset(get_copy = "pub", set = "pub")]
    position: units::Position,
    /// Velocity of the object relative to `parent`.
    #[getset(get_copy = "pub", set = "pub")]
    velocity: units::Velocity,
    /// The entity of the star that manages the gravity on this object.
    #[getset(get_copy = "pub", set = "pub")]
    parent: specs::Entity,
}

/// An object standing on a star.
///
/// If the object is moving, it is represented as `Accelerating`.
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct StandingBody {
    /// The current position of the body.
    #[getset(get_copy = "pub", set = "pub")]
    position: units::Position,
    /// The entity of the star this body stands on.
    #[getset(get_copy = "pub", set = "pub")]
    parent: specs::Entity,
}

impl Body {
    /// Retrieves the parent of this body.
    ///
    /// Returns `None` if this body is a root star.
    pub fn parent(&self) -> Option<specs::Entity> {
        match self {
            Self::Root(_) => None,
            Self::Orbiting(body) => Some(body.parent()),
            Self::Accelerating(body) => Some(body.parent()),
            Self::Standing(body) => Some(body.parent()),
        }
    }

    /// Computes the position of the entity.
    ///
    /// Panics if the body is a root star.
    pub fn position(&self, t: units::GameInstant) -> units::Position {
        match self {
            Self::Root(_) => unreachable!("A root star has no position"),
            Self::Orbiting(body) => body.orbit().approx_position(t, 0.),
            Self::Accelerating(body) => body.position(),
            Self::Standing(body) => body.position(),
        }
    }

    /// Computes the velocity of this body.
    ///
    /// Panics if the body is a root star.
    pub fn velocity(&self, t: units::GameInstant, mass: units::Mass) -> units::Velocity {
        match self {
            Self::Root(_) => unreachable!("A root star has no velocity"),
            Self::Orbiting(body) => body.orbit().approx_velocity(t, mass, 0.),
            Self::Accelerating(body) => body.velocity(),
            Self::Standing(body) => units::Velocity::default(),
        }
    }
}

impl specs::Component for Body {
    type Storage = specs::storage::VecStorage<Self>;
}

/// An object with a non-negligible gravitational field.
///
/// An entity with `Star` also has a `Body` component iff it is not a root star.
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct Star {
    /// Radius of the effective gravitational field.
    ///
    /// If an object moves beyond this radius, it is regarded as "out of" the current star system.
    ///
    /// The root star system also has a finite field radius.
    /// Bodies moving out of the field radius would trigger a `BodyEvent::Void`.
    #[getset(get_copy = "pub", set = "pub")]
    field_radius: units::Length,

    /// Mass of the star, used for orbit calculation.
    #[getset(get_copy = "pub", set = "pub")]
    strength: units::Mass,

    /// Index of bodies in this star.
    #[getset(get = "pub", get_mut = "pub")]
    index: BodyIndex,
}

impl specs::Component for Star {
    type Storage = specs::storage::BTreeStorage<Self>;
}
