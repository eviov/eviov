//! The main common crate for the client and servers.
//!
//! This crate reexports items defined in other subcrates.

#![feature(type_alias_impl_trait, option_expect_none)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(
    missing_docs,
    unused_results,
    unused_qualifications,
    variant_size_differences,
    clippy::checked_conversions,
    clippy::needless_borrow,
    clippy::shadow_unrelated,
    clippy::wrong_pub_self_convention
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
    clippy::result_unwrap_used
)]
#![cfg_attr(not(debug_assertions), deny(warnings, clippy::dbg_macro,))]

pub use eviov_context::*;
pub use eviov_types::*;
pub use eviov_proto as proto;
pub use eviov_transport as transport;

mod orbit;
pub use orbit::*;

/// General-purpose enum to denote the termination action of a looping function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopAction {
    /// Stop executing the loop
    Break,
    /// Continue executing the loop
    Continue,
}
