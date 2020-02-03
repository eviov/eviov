use matches2::option_match;
use proc_macro2::TokenStream;
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

    fn impl_message_from(some: bool, ident: &syn::Ident, server: bool) -> TokenStream {
        if !some {
            return quote!();
        }
        let from = if server {
            quote!(Server)
        } else {
            quote!(Client)
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
        ($me:ident, $peer:ident, $FromMe:ident, $FromPeer:ident) => {{

            let msg = defs.iter().filter(|def| def.$me())
                        .filter_map(|def| option_match!(def, Def::Message(msg) => &msg.name))
                        .map(|ident| quote!(#ident(#ident)));
            let req = query_req_res_idents!($me, Request);
            let req_qid = req.clone().map(|ident| quote!($FromMe::#ident(msg) => Some(msg.query_id)));
            let req_idents = req.map(|ident| quote!(#ident(#ident)));
            let res = query_req_res_idents!($peer, Response);
            let res_qid = res.clone().map(|ident| quote!($FromMe::#ident(msg) => Some(msg.query_id)));
            let res_idents = res.map(|ident| quote!(#ident(#ident)));
            quote! {
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub enum $FromMe {
                    #(#msg,)*
                    #(#req_idents,)*
                    #(#res_idents,)*
                }

                impl crate::proto::Endpoint for $FromMe {
                    type Protocol = Proto;
                    type Peer = $FromPeer;

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
            }
        }};
    }
    let from_client = enum_from!(client, server, Client, Server);
    let from_server = enum_from!(server, client, Server, Client);

    let messages = defs
        .iter()
        .filter_map(|def| option_match!(def, Def::Message(msg) => msg))
        .map(|msg| {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let fields = msg.fields.iter();

            let client = impl_message_from(msg.client, name, false);
            let server = impl_message_from(msg.server, name, true);

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
            let client_req = impl_message_from(msg.client, req_name, false);
            let client_res = impl_message_from(msg.client, res_name, true);
            let server_req = impl_message_from(msg.server, req_name, true);
            let server_res = impl_message_from(msg.server, res_name, false);
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
