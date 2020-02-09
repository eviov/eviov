#![feature(type_alias_impl_trait, option_expect_none)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(unused_results, unused_qualifications, variant_size_differences)]
#![deny(anonymous_parameters, bare_trait_objects)]

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
