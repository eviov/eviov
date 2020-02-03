use heck::*;
use matches2::option_match;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::{format_ident, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(client);
    syn::custom_keyword!(server);
    syn::custom_keyword!(mutual);

    syn::custom_keyword!(message);
    syn::custom_keyword!(query);

    syn::custom_keyword!(name);
}

pub fn main(ts: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<Input>(ts)?;

    let defs = &input.defs;

    macro_rules! query_req_res_idents {
        ($cs:ident, $suffix:ident) => {
            defs.iter().filter(|def| def.$cs())
                .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
                .map(|ident| {
                    format_ident!(concat!("{}", stringify!($suffix)), ident)
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
            quote!(Server)
        } else {
            quote!(Client)
        };

        let resp = if let Some(response) = response {
            quote! {
                impl crate::proto::QueryRequestFrom<#from> for #ident {
                    type Response = #response;
                }
            }
        } else {
            quote!()
        };
        quote! {
            impl crate::proto::MessageFrom<#from> for #ident {
                fn to_enum(self) -> #from {
                    #from::#ident(self)
                }

                fn from_enum(e: #from) -> Option<Self> {
                    match e {
                        #from::#ident(msg) => Some(msg),
                        _ => None,
                    }
                }
            }

            #resp
        }
    }

    let proto_attrs = &input.attrs;
    let proto_name = &input.name;
    let proto = quote! {
        #(#proto_attrs)*
        pub struct Proto;

        impl crate::proto::Protocol for Proto {
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
                        .filter_map(|def| option_match!(def, Def::Message(msg) => &msg.name));
            let msg_arms = msg.clone().map(|ident| quote!(#ident(#ident)));
            let req = query_req_res_idents!($me, Request);
            let req_qid = req.clone().map(|ident| quote!(#me_camel_ident::#ident(msg) => Some(msg.query_id)));
            let req_idents = req.clone().map(|ident| quote!(#ident(#ident)));
            let res = query_req_res_idents!($peer, Response);
            let res_qid = res.clone().map(|ident| quote!(#me_camel_ident::#ident(msg) => Some(msg.query_id)));
            let res_idents = res.map(|ident| quote!(#ident(#ident)));

            let endpoint = quote! {
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub enum #me_camel_ident {
                    #(#msg_arms,)*
                    #(#req_idents,)*
                    #(#res_idents,)*
                }

                impl crate::proto::Endpoint for #me_camel_ident {
                    type Protocol = Proto;
                    type Peer = #peer_camel_ident;

                    fn request_query_id(&self) -> Option<crate::proto::QueryId> {
                        match self {
                            #(#req_qid,)*
                            _ => None,
                        }
                    }

                    fn response_query_id(&self) -> Option<crate::proto::QueryId> {
                        match self {
                            #(#res_qid,)*
                            _ => None,
                        }
                    }
                }
            };

            let peer_handler_wrapper = &format_ident!("{}HandlerWrapper", peer_camel);
            let peer_handler = &format_ident!("{}Handler", peer_camel);

            let (handle_message_arms, handle_message_methods): (Vec<_>, Vec<_>) = msg.map(|ident| {
                let ident = &ident;
                let method_name = &syn::Ident::new(&format!("Handle{}", ident).to_snake_case(), ident.span());
                (quote! {
                    #me_camel_ident::#ident(message) => {
                        (self.0).#method_name(message).await;
                        None
                    }
                }, quote! {
                    async fn #method_name(&self, message: #ident);
                })
            }).unzip();
            let (handle_request_arms, handle_request_methods): (Vec<_>, Vec<_>) = req.map(|ident| {
                let ident = &ident;
                let method_name = &syn::Ident::new(&format!("Handle{}", ident).to_snake_case(), ident.span());
                (quote! {
                    #me_camel_ident::#ident(request) => {
                        use crate::proto::MessageFrom;

                        let query_id = request.query_id;
                        let mut response = (self.0).#method_name(request).await;
                        response.query_id = query_id;
                        Some(response.to_enum())
                    }
                }, quote! {
                    async fn #method_name(&self, request: #ident) -> <#ident as crate::proto::QueryRequestFrom<#me_camel_ident>>::Response;
                })
            }).unzip();

            let handler = quote! {
                #[derive(Debug)]
                pub struct #peer_handler_wrapper<H: #peer_handler>(pub H);

                #[::async_trait::async_trait]
                impl<H: #peer_handler>  crate::proto::Handler for #peer_handler_wrapper<H>{
                    type Endpoint = #peer_camel_ident;

                    async fn handle_message(&self, e: #me_camel_ident) -> Option<Self::Endpoint> {
                        match e {
                            #(#handle_request_arms,)*
                            #(#handle_message_arms,)*
                            _ => unreachable!("handle_message() should not handle response messages"),
                        }
                    }
                }

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

            quote! {
                #(#attrs)*
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #name { #(#fields),* }

                impl crate::proto::Message for #name {
                    type Protocol = Proto;
                }

                impl crate::proto::Single for #name {}

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
            let response = msg.response.iter();

            quote! {
                #(#attrs)*
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #req_name {
                    /// The query ID of this request.
                    ///
                    /// Any value can be passed, since this value is immediately overwritten by
                    /// the connection handler, but conventionally `Default::default()` is used.
                    pub query_id: crate::proto::QueryId,
                    #(#request),*
                }
                impl crate::proto::Message for #req_name {
                    type Protocol = Proto;
                }
                impl crate::proto::QueryRequest for #req_name {
                    fn query_id(&self) -> crate::proto::QueryId {
                        self.query_id
                    }

                    fn set_query_id(&mut self, id: crate::proto::QueryId) {
                        self.query_id = id;
                    }
                }
                #client_req
                #server_req

                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #res_name {
                    /// The query ID that this response responds to.
                    ///
                    /// Any value can be passed, since this value is immediately overwritten by
                    /// the connection handler, but conventionally `Default::default()` is used.
                    pub query_id: crate::proto::QueryId, #(#response),*
                }
                impl crate::proto::Message for #res_name {
                    type Protocol = Proto;
                }
                impl crate::proto::QueryResponse for #res_name {
                    fn query_id(&self) -> crate::proto::QueryId {
                        self.query_id
                    }

                    fn set_query_id(&mut self, id: crate::proto::QueryId) {
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
    name: String,
    defs: Vec<Def>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = syn::Attribute::parse_outer(input)?;

        let _ = input.parse::<kw::name>()?;
        let _ = input.parse::<syn::Token![=]>().unwrap();
        let name = input.parse::<syn::LitStr>()?.value();
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
        if input.peek(kw::client) {
            let _ = input.parse::<kw::client>().unwrap();
            client = true;
        } else if input.peek(kw::server) {
            let _ = input.parse::<kw::server>().unwrap();
            server = true;
        } else if input.peek(kw::mutual) {
            let _ = input.parse::<kw::mutual>().unwrap();
            client = true;
            server = true;
        } else {
            return Err(input.error("Expected `client`, `server` or `mutual`"));
        }

        let ret = if input.peek(kw::message) {
            let _ = input.parse::<kw::message>().unwrap();
            let name = input.parse::<syn::Ident>()?;
            let fields = parse_fields(input)?;
            Def::Message(MessageDef {
                attrs,
                client,
                server,
                name,
                fields,
            })
        } else if input.peek(kw::query) {
            let _ = input.parse::<kw::query>().unwrap();
            let name = input.parse::<syn::Ident>()?;
            let request = parse_fields(input)?;
            let _ = input.parse::<syn::Token![->]>()?;
            let response = parse_fields(input)?;
            Def::Query(QueryDef {
                attrs,
                client,
                server,
                name,
                request,
                response,
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
}

struct QueryDef {
    attrs: Vec<syn::Attribute>,
    client: bool,
    server: bool,
    name: syn::Ident,
    request: Punctuated<Field, syn::Token![,]>,
    response: Punctuated<Field, syn::Token![,]>,
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
        tokens.extend(quote!(#(#attrs)* pub #name: #ty));
    }
}
