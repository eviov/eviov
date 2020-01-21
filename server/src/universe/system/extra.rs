use std::fmt;

use futures::future::Future;

use super::*;

pub trait Extra: Sized {
    type Message: Sized + fmt::Debug;

    type NextEvent: Future<Output = TickAction>;
    fn next_event(&mut self) -> Self::NextEvent;

    type SetupSystem: Future<Output = ()>;
    fn setup_system(&mut self, add_entity: impl FnMut()) -> Self::SetupSystem;
}
