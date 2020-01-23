use std::ops::{Deref, DerefMut};

pub trait Lock<T> {
    fn new(value: T) -> Self;

    type ReadGuard: Deref<Target = T>;
    fn read(&self) -> Self::ReadGuard;

    type WriteGuard: DerefMut<Target = T>;
    fn write(&self) -> Self::WriteGuard;
}
