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

//! Code generation crate for eviov

extern crate proc_macro;

use proc_macro::TokenStream;

#[path = "proto.rs"]
mod proto_;

/// Generates a protocol definition.
///
/// This macro generates hardcoded symbols like `Client`, `Proto` and `Server`.
/// Each `codegen::proto!` should have its dedicated module.
///
/// # Syntax
/// The macro block can start with inner attributes, which are going to be applied on the generated `Proto` struct.
///
/// The content then starts with the protocol name: `name = "xxx";`.
///
/// Then there can be any number of messages of queries, with the following format:
/// ```text
/// ITEM := OUTER_ATTRIBUTES* (MESSAGE | QUERY)
///
/// # The IDENT is the message name.
/// # The FIELD_LIST contains the fields of the message.
/// MESSAGE := DIRECTION "message" IDENT FIELD_LIST
///
/// # The IDENT is the query name. The request and response structs start with the query name,
/// followed by "Request" or "Response".
/// # The first FIELD_LIST contains the fields of the request.
/// # The second FIELD_LIST contains the fields of the request.
/// QUERY := DIRECTION "query" IDENT FIELD_LIST "->" FIELD_LIST
///
/// # DIRECTION indicates the *source* of the message or the *sender* of the query request
/// (i.e. the *receiver* of the query *response*)
/// DIRECTION := "client" | "server" | "mutual"
///
/// FIELD_LIST := "{" ( FIELD )* "}"
///
/// # The outer attributes are applied to the message struct, or the request struct for queries
/// # The IDENT is the name of the field.
/// # The TYPE is the type of the field.
/// FIELD := OUTER_ATTRIBUTES* IDENT ":" TYPE ","
/// ```
#[proc_macro]
pub fn proto(ts: TokenStream) -> TokenStream {
    proto_::main(ts.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
