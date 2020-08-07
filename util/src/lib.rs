//! Miscellaneous util functions

use std::fmt;
use std::ops;

use nalgebra::Vector2;

/// Perform Newton's method on the input.
///
/// # Parameters
/// - `f`: the function to invert
/// - `df`: the first-order derivative of `f` w.r.t. its parameter
/// - `x`: the initial guess value
pub fn newton_method<X, Y, Dy, Q, F, Df, T>(mut f: F, mut df: Df, mut x: X, mut tolerable: T) -> X
where
    X: ops::Sub<Q, Output = X> + Copy,
    Y: ops::Div<Dy, Output = Q>,
    F: FnMut(X) -> Y,
    Df: FnMut(X) -> Dy,
    T: FnMut(X, X) -> bool,
{
    loop {
        let y: Y = f(x);
        let dy: Dy = df(x);
        let q: Q = y / dy;
        let new = x - q;
        if tolerable(x, new) {
            return new;
        }
        x = new;
    }
}

/// Perform Newton's method on the input for up to `iterations` times.
pub fn newton_method_iterations<X, Y, Dy, Q, F, Df, T>(
    f: F,
    df: Df,
    x: X,
    mut iterations: u32,
    mut tolerable: T,
) -> X
where
    X: ops::Sub<Q, Output = X> + Copy,
    Y: ops::Div<Dy, Output = Q>,
    F: FnMut(X) -> Y,
    Df: FnMut(X) -> Dy,
    T: FnMut(X, X) -> bool,
{
    newton_method(f, df, x, |x1, x2| {
        if iterations == 0 {
            return true;
        }
        iterations -= 1;
        tolerable(x1, x2)
    })
}

/// Compute the magnitude of the cross product of two 2D vectors extended to 3D.
pub fn cross2d<T>(a: Vector2<T>, b: Vector2<T>) -> T
where
    T: fmt::Debug + Copy + PartialEq + 'static,
    T: ops::Mul<T, Output = T>,
    T: ops::Sub<T, Output = T>,
{
    a[0] * b[1] - b[0] * a[1]
}
