#![allow(clippy::result_unwrap_used)] // Allow result.unwrap(), since it is used very extensively in parsing

use heck::*;
use matches2::option_match;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

mod kw {
    syn::custom_keyword!(client);
    syn::custom_keyword!(server);
    syn::custom_keyword!(mutual);

    syn::custom_keyword!(message);
    syn::custom_keyword!(query);

    syn::custom_keyword!(name);
}

#[allow(clippy::cognitive_complexity)]
pub fn main(ts: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<Input>(ts)?;

    let defs = &input.defs;

    macro_rules! query_req_res_idents {
        ($cs:ident, $suffix:ident) => {
            defs.iter()
                .filter(|def| def.$cs())
                .filter_map(|def| match def {
                    Def::Query(msg) => Some((
                        msg.span,
                        &msg.attrs,
                        format_ident!("{}{}", &msg.name, stringify!($suffix)),
                    )),
                    #[allow(unreachable_patterns)]
                    _ => None,
                })
        };
    }

    fn impl_message_from(
        some: bool,
        ident: &syn::Ident,
        server: bool,
        response: Option<&syn::Ident>,
    ) -> TokenStream {
        if !some {
            return quote!();
        }
        let from = if server {
            quote_spanned!(ident.span() => Server)
        } else {
            quote_spanned!(ident.span() => Client)
        };

        let resp = if let Some(response) = response {
            quote_spanned! { response.span() =>
                impl crate::QueryRequestFrom<#from> for #ident {
                    type Response = #response;
                }
            }
        } else {
            quote!()
        };
        quote_spanned! { ident.span() =>
            impl crate::MessageFrom<#from> for #ident {
                fn to_enum(self) -> #from {
                    #from::#ident(self)
                }

                fn from_enum(e: #from) -> Option<Self> {
                    match e {
                        #from::#ident(msg) => Some(msg),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            #resp
        }
    }

    let proto_attrs = &input.attrs;
    let proto_name = &input.name.value();
    let proto = quote! {
        #(#proto_attrs)*
        pub struct Proto;

        impl crate::Protocol for Proto {
            type Client = Client;
            type Server = Server;

            fn name() -> &'static str {
                #proto_name
            }
        }
    };

    macro_rules! enum_from {
        ($me:ident, $peer:ident) => {{
            let me_camel = stringify!($me).to_camel_case();
            let me_camel_ident = syn::Ident::new(&me_camel, Span::call_site());
            let peer_camel = stringify!($peer).to_camel_case();
            let peer_camel_ident = syn::Ident::new(&peer_camel, Span::call_site());

            let msg = defs.iter().filter(|def| def.$me())
                        .filter_map(|def| option_match!(def, Def::Message(msg) => (msg.span, &msg.attrs, &msg.name)));
            let msg_arms = msg.clone().map(|(item_span, attrs, ident)| quote_spanned! { item_span =>
                #(#attrs)*
                #ident(#ident),
            });
            let req = query_req_res_idents!($me, Request);
            let req_qid = req.clone().map(|(item_span, _, ident)| quote_spanned!( item_span => #me_camel_ident::#ident(msg) => Some(msg.query_id)));
            let req_idents = req.clone().map(|(item_span, attrs, ident)| quote_spanned! { item_span =>
                #(#attrs)*
                #ident(#ident),
            });
            let res = query_req_res_idents!($peer, Response);
            let res_qid = res.clone().map(|(item_span, _, ident)| quote_spanned!( item_span => #me_camel_ident::#ident(msg) => Some(msg.query_id)));
            let res_idents = res.map(|(item_span, attrs, ident)| quote_spanned! { item_span =>
                #(#attrs)*
                #ident(#ident),
            });

            let endpoint = quote! {
                /// An enum of packets sent from the
                #[doc = #me_camel]
                /// endpoint.
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                #[allow(variant_size_differences)]
                pub enum #me_camel_ident {
                    #(#msg_arms)*
                    #(#req_idents)*
                    #(#res_idents)*
                }

                impl crate::Endpoint for #me_camel_ident {
                    type Protocol = Proto;
                    type Peer = #peer_camel_ident;

                    fn request_query_id(&self) -> Option<crate::QueryId> {
                        match self {
                            #(#req_qid,)*
                            #[allow(unreachable_patterns)]
                            _ => None,
                        }
                    }

                    fn response_query_id(&self) -> Option<crate::QueryId> {
                        match self {
                            #(#res_qid,)*
                            #[allow(unreachable_patterns)]
                            _ => None,
                        }
                    }
                }
            };

            let peer_handler_wrapper = &format_ident!("{}HandlerWrapper", peer_camel);
            let peer_handler = &format_ident!("{}Handler", peer_camel);

            let (handle_message_arms, handle_message_methods): (Vec<_>, Vec<_>) = msg.map(|(item_span, _, ident)| {
                let ident = &ident;
                let method_name = &syn::Ident::new(&format!("Handle{}", ident).to_snake_case(), ident.span());
                let method_doc = format!("Handles the message [`{name}`](struct.{name}.html).", name = ident);
                (quote_spanned! { item_span =>
                    #me_camel_ident::#ident(message) => {
                        (self.0).#method_name(message).await;
                        None
                    }
                }, quote_spanned! { item_span =>
                    #[doc = #method_doc]
                    async fn #method_name(&self, message: #ident);
                })
            }).unzip();
            let (handle_request_arms, handle_request_methods): (Vec<_>, Vec<_>) = req.map(|(item_span, _, ident)| {
                let ident = &ident;
                let method_name = &syn::Ident::new(&format!("Handle{}", ident).to_snake_case(), ident.span());
                let method_doc = format!("Handles the query [`{name}`](struct.{name}.html).", name = ident);
                (quote_spanned! { item_span =>
                    #me_camel_ident::#ident(request) => {
                        use crate::MessageFrom;

                        let query_id = request.query_id;
                        let mut response = (self.0).#method_name(request).await;
                        response.query_id = query_id;
                        Some(response.to_enum())
                    }
                }, quote_spanned! { item_span =>
                    #[doc = #method_doc]
                    async fn #method_name(&self, request: #ident) -> <#ident as crate::QueryRequestFrom<#me_camel_ident>>::Response;
                })
            }).unzip();

            let handler = quote! {
                /// Implementation of `eviov_proto::Handler` for the
                #[doc = #peer_camel]
                /// endpoint.
                #[derive(Debug)]
                pub struct #peer_handler_wrapper<H: #peer_handler>(pub H);

                #[::async_trait::async_trait]
                impl<H: #peer_handler>  crate::Handler for #peer_handler_wrapper<H>{
                    type Endpoint = #peer_camel_ident;

                    async fn handle_message(&self, e: #me_camel_ident) -> Option<Self::Endpoint> {
                        match e {
                            #(#handle_request_arms,)*
                            #(#handle_message_arms,)*
                            #[allow(unreachable_patterns)]
                            _ => unreachable!("handle_message() should not handle response messages"),
                        }
                    }
                }

                /// Message-specific handler methods for the
                #[doc = #peer_camel]
                /// endpoint
                #[::async_trait::async_trait]
                pub trait #peer_handler: Sized + Send + Sync + 'static {
                    #(#handle_request_methods)*
                    #(#handle_message_methods)*
                }
            };

            quote! {
                #endpoint
                #handler
            }
        }};
    }
    let from_client = enum_from!(client, server);
    let from_server = enum_from!(server, client);

    let messages = defs
        .iter()
        .filter_map(|def| option_match!(def, Def::Message(msg) => msg))
        .map(|msg| {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let fields = msg.fields.iter();

            let client = impl_message_from(msg.client, name, false, None);
            let server = impl_message_from(msg.server, name, true, None);

            quote_spanned! { msg.span =>
                #(#attrs)*
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #name { #(#fields),* }

                impl crate::Message for #name {
                    type Protocol = Proto;
                }

                impl crate::Single for #name {}

                #client
                #server
            }
        });
    let queries = defs
        .iter()
        .filter_map(|def| option_match!(def, Def::Query(msg) => msg))
        .map(|msg| {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let req_name = &format_ident!("{}Request", name);
            let res_name = &format_ident!("{}Response", name);

            let request = msg.request.iter();
            let client_req = impl_message_from(msg.client, req_name, false, Some(res_name));
            let client_res = impl_message_from(msg.client, res_name, true, None);
            let server_req = impl_message_from(msg.server, req_name, true, Some(res_name));
            let server_res = impl_message_from(msg.server, res_name, false, None);
            let response_doc = format!("Response type for {}", res_name);
            let response = msg.response.iter();

            quote_spanned! { msg.span =>
                #(#attrs)*
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #req_name {
                    /// The query ID of this request.
                    ///
                    /// Any value can be passed, since this value is immediately overwritten by
                    /// the connection handler, but conventionally `Default::default()` is used.
                    pub query_id: crate::QueryId,
                    #(#request),*
                }
                impl crate::Message for #req_name {
                    type Protocol = Proto;
                }
                impl crate::QueryRequest for #req_name {
                    fn query_id(&self) -> crate::QueryId {
                        self.query_id
                    }

                    fn set_query_id(&mut self, id: crate::QueryId) {
                        self.query_id = id;
                    }
                }
                #client_req
                #server_req

                #[doc = #response_doc]
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #res_name {
                    /// The query ID that this response responds to.
                    ///
                    /// Any value can be passed, since this value is immediately overwritten by
                    /// the connection handler, but conventionally `Default::default()` is used.
                    pub query_id: crate::QueryId, #(#response),*
                }
                impl crate::Message for #res_name {
                    type Protocol = Proto;
                }
                impl crate::QueryResponse for #res_name {
                    fn query_id(&self) -> crate::QueryId {
                        self.query_id
                    }

                    fn set_query_id(&mut self, id: crate::QueryId) {
                        self.query_id = id;
                    }
                }
                #client_res
                #server_res
            }
        });

    let ret = quote! {
        #proto
        #from_client
        #from_server
        #(#messages)*
        #(#queries)*
    };
    //println!("=================\n=============\n{}", &ret);
    Ok(ret)
}

struct Input {
    attrs: Vec<syn::Attribute>,
    name: syn::LitStr,
    defs: Vec<Def>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = syn::Attribute::parse_outer(input)?;

        let _ = input.parse::<kw::name>()?;
        let _ = input.parse::<syn::Token![=]>().unwrap();
        let name = input.parse::<syn::LitStr>()?;
        let _ = input.parse::<syn::Token![;]>()?;

        let mut defs = vec![];
        while !input.is_empty() {
            defs.push(input.parse()?);
        }
        Ok(Self { attrs, name, defs })
    }
}

enum Def {
    Message(MessageDef),
    Query(QueryDef),
}

impl Def {
    fn client(&self) -> bool {
        match self {
            Def::Message(inner) => inner.client,
            Def::Query(inner) => inner.client,
        }
    }
    fn server(&self) -> bool {
        match self {
            Def::Message(inner) => inner.server,
            Def::Query(inner) => inner.server,
        }
    }
}

impl Parse for Def {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        fn parse_fields(input: ParseStream) -> syn::Result<Punctuated<Field, syn::Token![,]>> {
            let fields;
            syn::braced!(fields in input);
            fields.parse_terminated(Field::parse)
        }

        let attrs = input.call(syn::Attribute::parse_outer)?;

        let mut client = false;
        let mut server = false;
        let dir_span;
        if input.peek(kw::client) {
            dir_span = input.parse::<kw::client>().unwrap().span();
            client = true;
        } else if input.peek(kw::server) {
            dir_span = input.parse::<kw::server>().unwrap().span();
            server = true;
        } else if input.peek(kw::mutual) {
            dir_span = input.parse::<kw::mutual>().unwrap().span();
            client = true;
            server = true;
        } else {
            return Err(input.error("Expected `client`, `server` or `mutual`"));
        }

        let ret = if input.peek(kw::message) {
            let message_kw = input.parse::<kw::message>().unwrap();
            let name = input.parse::<syn::Ident>()?;
            let fields = parse_fields(input)?;

            let mut span = name.span();
            span = span.join(message_kw.span()).unwrap_or(span);
            span = span.join(fields.span()).unwrap_or(span);

            Def::Message(MessageDef {
                attrs,
                client,
                server,
                name,
                fields,
                span,
            })
        } else if input.peek(kw::query) {
            let query_kw = input.parse::<kw::query>().unwrap();
            let name = input.parse::<syn::Ident>()?;
            let request = parse_fields(input)?;
            let arrow = input.parse::<syn::Token![->]>()?;
            let response = parse_fields(input)?;

            let mut span = name.span();
            span = span.join(dir_span).unwrap_or(span);
            span = span.join(query_kw.span()).unwrap_or(span);
            span = span.join(request.span()).unwrap_or(span);
            span = span.join(arrow.span()).unwrap_or(span);
            span = span.join(response.span()).unwrap_or(span);

            Def::Query(QueryDef {
                attrs,
                client,
                server,
                name,
                request,
                response,
                span,
            })
        } else {
            return Err(input.error("Expected `message` or `query`"));
        };

        Ok(ret)
    }
}

struct MessageDef {
    attrs: Vec<syn::Attribute>,
    client: bool,
    server: bool,
    name: syn::Ident,
    fields: Punctuated<Field, syn::Token![,]>,
    span: Span,
}

struct QueryDef {
    attrs: Vec<syn::Attribute>,
    client: bool,
    server: bool,
    name: syn::Ident,
    request: Punctuated<Field, syn::Token![,]>,
    response: Punctuated<Field, syn::Token![,]>,
    span: Span,
}

struct Field {
    attrs: Vec<syn::Attribute>,
    name: syn::Ident,
    ty: syn::Type,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name = input.parse()?;
        let _ = input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        Ok(Self { attrs, name, ty })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let name = &self.name;
        let ty = &self.ty;
        let mut span = name.span();
        span = span.join(ty.span()).unwrap_or(span);
        tokens.extend(quote_spanned!(span => #(#attrs)* pub #name: #ty));
    }
}
