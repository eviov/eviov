#![feature(never_type, stmt_expr_attributes)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(
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

use std::io;
use std::sync::Arc;

mod plugin;
pub use plugin::*;

pub mod universe;

mod ws;

fn create_clock() -> ! {
    unimplemented!()
}

#[tokio::main]
pub async fn start<X: Plugin>() -> io::Result<()> {
    pretty_env_logger::init();

    let runtime = {
        #![allow(clippy::diverging_sub_expression)]
        universe::Runtime::<X::SystemExtra>::new(create_clock())
    };
    let plugin = X::init(universe::Runtime::clone(&runtime));

    let plugin = Arc::new(plugin);
    ws::start(plugin).await?;

    Ok(())
}
