use amethyst::ecs::{Entity, WriteStorage};

use super::collision::BoundingBox;
use crate::units;

/// A data structure to store a set of bodies.
///
/// This data structure needs to be `update()`d every tick.
/// For the ordinary case, updating is NOT O(n).
#[derive(Debug)]
pub struct BodyIndex {
    // orbiting
    radial: Vec<Entity>,
    angular: Vec<Entity>,

    // standing
    standing: Vec<Entity>,
}

impl BodyIndex {
    /// Updates the orbit index every tick.
    pub fn update(&mut self) {
        todo!()
    }

    /// Inserts an entity into the index.
    pub fn insert(&mut self, entity: Entity, store_bb: WriteStorage<'_, BoundingBox>) {
        todo!()
    }

    /// Removes an entity from the index.
    pub fn remove(&mut self, entity: Entity) {
        todo!()
    }

    /// Returns all entities in this index.
    pub fn all(&self) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }

    /// Returns all entities in this index that are orbiting.
    pub fn all_orbiting(&self) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }

    /// Returns all entities in this index that are accelerating.
    pub fn all_accelerating(&self) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }

    /// Returns all entities in this index that are standing.
    pub fn all_standing(&self) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }

    /// Returns all entities in this index within the bearing.
    pub fn between(
        &self,
        from: units::Bearing,
        to: units::Bearing,
    ) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }

    /// Returns all entities in this index below the radius.
    pub fn below(&self, radius: units::Length) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }

    /// Returns all entities in this index above the radius.
    pub fn above(&self, radius: units::Length) -> impl Iterator<Item = Entity> {
        std::iter::once(todo!())
    }
}
