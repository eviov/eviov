#![feature(option_expect_none)]
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

//! eviov communications framework

use std::time::Duration;

mod ws;
pub use ws::*;

/// The maximum query pool size for each connection.
pub const MAX_QUERY_POOL_SIZE: usize = 1000;

/// The duration to wait for before a query pool timeouts.
pub const OPEN_CONN_TIMEOUT: Duration = Duration::from_secs(10);
