#![feature(type_alias_impl_trait, option_expect_none)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(
    unused_results,
    unused_qualifications,
    variant_size_differences,
    clippy::checked_conversions,
    clippy::needless_borrow,
    clippy::shadow_unrelated,
    clippy::wrong_pub_self_convention,
)]
#![deny(
    anonymous_parameters,
    bare_trait_objects,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::float_cmp_const,
    clippy::if_not_else,
    clippy::indexing_slicing,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
)]
#![cfg_attr(not(debug_assertions), deny(
    warnings,
    clippy::dbg_macro,
))]

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
