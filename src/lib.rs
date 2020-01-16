#![allow(dead_code, unused_variables)]

#[macro_export]
macro_rules! noop {
    ($($tt:tt)*) => {}
}

#[macro_export]
macro_rules! if_else {
    (if ($($if:tt)+) {$($then:tt)*} else {$($else:tt)*}) => { $($then:tt)* };
    (if () {$($then:tt)*} else {$($else:tt)*}) => { $($else:tt)* };
}

dirmod::all!(default file pub use; default dir pub);
