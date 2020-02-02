#![feature(type_alias_impl_trait, option_expect_none)]
#![allow(dead_code, unused_variables)]
#![warn(unused_results)]

mod id;
pub use id::*;
mod time;
pub use time::*;
mod lock;
pub mod timeout;
pub use lock::Lock;

pub mod hardcode;
pub mod math;
pub mod proto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopAction {
    Break,
    Continue,
}
