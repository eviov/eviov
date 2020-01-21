use derive_more::{Add, Neg, Sub};
use num_traits::{Float, Num};
use operator_sugar::operator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Neg, Serialize, Deserialize, Add, Sub)]
pub struct Vector<T: Num>(pub T, pub T);

impl<T: Num + Copy> Vector<T> {
    pub fn modulus_sq(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T: Num + Float> Vector<T> {
    pub fn modulus(self) -> T {
        self.modulus_sq().sqrt()
    }
}

impl<T: Num> Default for Vector<T> {
    fn default() -> Self {
        Vector(T::zero(), T::zero())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScaleVector<T: Num>(pub T, pub T);

impl<T: Num + Copy> From<T> for ScaleVector<T> {
    fn from(t: T) -> ScaleVector<T> {
        ScaleVector(t, t)
    }
}

impl<T: Num> Default for ScaleVector<T> {
    fn default() -> Self {
        ScaleVector(T::one(), T::one())
    }
}

operator!({T: Num}
    Vector<T>, ScaleVector<T>: a * b -> Vector<T> {
        Vector(a.0 * b.0, a.1 * b.1)
    }
);

operator!({T: Num + Copy}
    Vector<T>, T: a * b -> Vector<T> {
        Vector(a.0 * b, a.1 * b)
    }
);
operator!({T: Num + Copy}
    ScaleVector<T>, T: a * b -> ScaleVector<T> {
        ScaleVector(a.0 * b, a.1 * b)
    }
);
