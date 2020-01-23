use std::ops::{Deref, DerefMut};

use eviov::Lock;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

impl<'a, T: ?Sized> Lock<T> for RwLock<T>
where
    Self: 'a,
{
    fn new(value: T) -> Self {
        RwLock::new(value)
    }

    type ReadGuard = Read<'a, T>;
    fn read(&self) -> Read<'a, T> {
        Read(self.read())
    }

    type WriteGuard = Write<'a, T>;
    fn write(&self) -> Write<'a, T> {
        Write(self.write())
    }
}

struct Read<'a, T: ?Sized>(RwLockReadGuard<'a, T>);

impl<'a, T: ?Sized + 'a> Deref for Read<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref().unwrap()
    }
}

struct Write<'a, T: ?Sized>(RwLockWriteGuard<'a, T>);

impl<'a, T: ?Sized + 'a> Deref for Write<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref().unwrap()
    }
}

impl<'a, T: ?Sized + 'a> DerefMut for Write<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }
}
