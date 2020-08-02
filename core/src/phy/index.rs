use amethyst::ecs::Entity;

/// A data structure to store a set of orbits.
///
/// This data structure needs to be `update()`d every tick.
/// For the ordinary case, updating is NOT O(n).
///
/// It supports the following operations efficiently
/// (n is the number of orbits in the set, m is the number of results returned)
/// - Insert an orbit from the set, O(log n)
/// - Remove an orbit from the set, O(log n)
/// - Query a subset of orbits in the range between two bearings, O(log n)
#[derive(Debug)]
pub struct OrbitIndex {
    radial: Vec<Entity>,
    angular: Vec<Entity>,
}
