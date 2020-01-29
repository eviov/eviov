#![warn(unused_results)]

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
