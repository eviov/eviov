/// The mass of an object.
///
/// This unit has a standard scale in the whole world.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Mass(pub f64);

add_newtype!(Mass, Mass);
sub_newtype!(Mass, Mass);

/// The hardness of a solid.
///
/// This is used to calculate momentum transfer after a collision.
///
/// TODO: find a rigorous definition for elasticity.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Elasticity(pub f64);

add_newtype!(Elasticity, Elasticity);
sub_newtype!(Elasticity, Elasticity);

/// The drag effect of a fluid.
///
/// This multiplies with the peer velocity squared
/// and the drag area of the peer BB shape (with respect to the correct angle)
/// to give the drag force.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Drag(pub f64);
