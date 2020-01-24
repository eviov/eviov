use std::ops::{Deref, DerefMut};

pub trait Lock<'t, T> {
    fn new(value: T) -> Self;

    type ReadGuard: Deref<Target = T>;
    fn read(&'t self) -> Self::ReadGuard;

    type WriteGuard: DerefMut<Target = T>;
    fn write(&'t self) -> Self::WriteGuard;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "trait-rwlock")] {
        use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

        impl<'t, T: 't> Lock<'t, T> for RwLock<T> {
            fn new(value: T) -> Self {
                RwLock::new(value)
            }

            type ReadGuard = Read<'t, T>;
            fn read(&'t self) -> Read<'t, T> {
                Read(self.read())
            }

            type WriteGuard = Write<'t, T>;
            fn write(&'t self) -> Write<'t, T> {
                Write(self.write())
            }
        }

        pub struct Read<'t, T: 't>(RwLockReadGuard<'t, T>);

        impl<'t, T: 't> Deref for Read<'t, T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        pub struct Write<'t, T: 't>(RwLockWriteGuard<'t, T>);

        impl<'t, T: 't> Deref for Write<'t, T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        impl<'t, T: 't> DerefMut for Write<'t, T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }
    }
}
