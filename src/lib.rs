#![feature(type_alias_impl_trait, option_expect_none)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(unused_results)]

mod id;
pub use id::*;
mod context;
pub use context::*;

pub mod hardcode;
pub mod math;
pub mod proto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopAction {
    Break,
    Continue,
}
