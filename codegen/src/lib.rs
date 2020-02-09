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

extern crate proc_macro;

use proc_macro::TokenStream;

#[path = "proto.rs"]
mod proto_;

#[proc_macro]
pub fn proto(ts: TokenStream) -> TokenStream {
    proto_::main(ts.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
