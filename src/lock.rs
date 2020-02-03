use std::cell;
use std::ops::{Deref, DerefMut};

pub trait Lock<'t, T> {
    fn new(value: T) -> Self;

    type ReadGuard: Deref<Target = T>;
    fn read(&'t self) -> Self::ReadGuard;

    type WriteGuard: DerefMut<Target = T>;
    fn write(&'t self) -> Self::WriteGuard;
}

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

        impl<'t, T: 't> Lock<'t, T> for RwLock<T> {
            fn new(value: T) -> Self {
                RwLock::new(value)
            }

            type ReadGuard = RwLockReadGuard<'t, T>;
            fn read(&'t self) -> Self::ReadGuard {
                self.read()
            }

            type WriteGuard = RwLockWriteGuard<'t, T>;
            fn write(&'t self) -> Self::WriteGuard {
                self.write()
            }
        }
    }
}

impl<'t, T: 't> Lock<'t, T> for cell::RefCell<T> {
    fn new(value: T) -> Self {
        Self::new(value)
    }

    type ReadGuard = cell::Ref<'t, T>;
    fn read(&'t self) -> Self::ReadGuard {
        self.borrow()
    }

    type WriteGuard = cell::RefMut<'t, T>;
    fn write(&'t self) -> Self::WriteGuard {
        self.borrow_mut()
    }
}
