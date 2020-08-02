use crate::nalgebra::Vector2;
use crate::units;

/// A measure of length in a star system.
///
/// The unit of length is *relative to the star system*.
/// When using length values from two different star systems,
/// scaling must first be performed.
///
/// TODO change to newtype
pub type Length = f64;

/// A Cartesian position, typically using the position of the parent star as the origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub Vector2<Length>);

impl Position {
    /// The origin Position, usually indicating the parent star.
    pub fn origin() -> Self {
        Self(Vector2::new(0., 0.))
    }
}

/// Represents the signed displacement between two positions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Displace(pub Vector2<Length>);

impl Default for Displace {
    fn default() -> Self {
        Self(Vector2::new(0., 0.))
    }
}

add_newtype!(Displace, Displace);
sub_newtype!(Displace, Displace);

macro_rules! impl_displace {
    ($op:ident, $snake_op:ident) => {
        impl std::ops::$op<f64> for Displace {
            type Output = Self;

            fn $snake_op(self, other: f64) -> Self {
                let Self(mut vec) = self;
                for &i in &[0, 1] {
                    vec[i] = std::ops::$op::$snake_op(vec[i], other);
                }
                Self(vec)
            }
        }
    };
}

impl_displace!(Mul, mul);
impl_displace!(Div, div);

add_newtype!(Position, Displace);
sub_newtype!(Position, Displace);

impl std::ops::Sub<Position> for Position {
    type Output = Displace;

    fn sub(self, other: Position) -> Displace {
        Displace(self.0 - other.0)
    }
}

/// A velocity in displacement per `GameDuration` tick.
pub type Velocity = super::rate::Rate<Displace>;

/// Extension trait for length-specific methods.
pub trait LengthExt: Sized + seal::Sealed {
    #[doc(hidden)]
    fn into_length(self) -> Length;

    /// Computes arcsin(self, hyp)
    fn arcsin(self, hyp: Length) -> units::Theta {
        units::Theta((self.into_length() / hyp).sin())
    }

    /// Computes arccos(self, hyp)
    fn arccos(self, hyp: Length) -> units::Theta {
        units::Theta((self.into_length() / hyp).cos())
    }

    /// Computes arctan(self, hyp)
    fn arctan(self, adj: Length) -> units::Theta {
        units::Theta(self.into_length().atan2(adj))
    }
}

impl LengthExt for Length {
    fn into_length(self) -> Length {
        self
    }
}

mod seal {
    pub trait Sealed {}
    impl Sealed for super::Length {}
}
