use matches2::option_match;
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use quote::{quote, };
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(client);
    syn::custom_keyword!(server);
    syn::custom_keyword!(mutual);

    syn::custom_keyword!(message);
    syn::custom_keyword!(query);
}

pub fn main(ts: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<Input>(ts)?;

    let proto_attrs = &input.attrs;
    let proto_name = &input.name;
    let defs = &input.defs;

    let client_message_names = defs
        .iter()
        .filter(|def| def.client())
        .filter_map(|def| option_match!(def, Def::Message(msg) => &msg.name))
        .map(|ident| quote!(#ident(#ident)));
    let server_message_names = defs
        .iter()
        .filter(|def| def.server())
        .filter_map(|def| option_match!(def, Def::Message(msg) => &msg.name))
        .map(|ident| quote!(#ident(#ident)));

    let client_query_requests = defs
        .iter()
        .filter(|def| def.client())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Request", ident);
            quote!(#ident(#ident))
        });
    let client_query_responses = defs
        .iter()
        .filter(|def| def.client())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Response", ident);
            quote!(#ident(#ident))
        });

    let server_query_requests = defs
        .iter()
        .filter(|def| def.server())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Request", ident);
            quote!(#ident(#ident))
        });
    let server_query_responses = defs
        .iter()
        .filter(|def| def.server())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Response", ident);
            quote!(#ident(#ident))
        });

    fn client_message(some: bool, ident: &syn::Ident) -> TokenStream {
        if !some {
            return quote!();
        }
        quote! {
            impl crate::proto::ClientMessage for #ident {
                fn to_enum(self) -> FromClient {
                    FromClient::#ident(self)
                }
            }
        }
    }

    fn server_message(some: bool, ident: &syn::Ident) -> TokenStream {
        if !some{return quote!();}
        quote! {
            impl crate::proto::ServerMessage for #ident {
                fn to_enum(self) -> FromServer {
                    FromServer::#ident(self)
                }
            }
        }
    }

    let messages = defs.iter().filter_map(|def| {
        option_match!(def, Def::Message(msg) => {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let fields = msg.fields.iter();

            let client = client_message(msg.client, name);
            let server = server_message(msg.server, name);

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
        })
    });

    let queries = defs.iter().filter_map(|def| {
        option_match!(def, Def::Query(msg) => {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let req_name = &format_ident!("{}Request", name);
            let res_name = &format_ident!("{}Response", name);

            let request = msg.request.iter();
            let client_req = client_message(msg.client, req_name);
            let client_res = server_message(msg.client, res_name);
            let server_req = server_message(msg.server, req_name);
            let server_res = client_message(msg.server, res_name);
            let response = msg.response.iter();

            quote! {
                #(#attrs)*
                pub struct #name;
                impl crate::proto::Query for #name {
                    type Request = #req_name;
                    type Response = #res_name;
                }

                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #req_name { query_id: crate::proto::QueryId, #(#request),* }
                impl crate::proto::Message for #req_name {
                    type Protocol = Proto;
                }
                #client_req
                #server_req
                impl crate::proto::QueryRequest for #req_name {
                    type Query = #name;

                    fn query_id(&self) -> crate::proto::QueryId {
                        self.query_id
                    }

                    fn set_query_id(&mut self, id: crate::proto::QueryId) {
                        self.query_id = id;
                    }
                }
                #client_res
                #server_res

                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #res_name { query_id: crate::proto::QueryId, #(#response),* }
                impl crate::proto::Message for #res_name {
                    type Protocol = Proto;
                }
                impl crate::proto::QueryResponse for #res_name {
                    type Query = #name;

                    fn query_id(&self) -> crate::proto::QueryId {
                        self.query_id
                    }

                    fn set_query_id(&mut self, id: crate::proto::QueryId) {
                        self.query_id = id;
                    }
                }
            }
        })
    });

    let ret = quote! {
        #(#proto_attrs)*
        pub struct Proto;

        impl crate::proto::Protocol for Proto {
            type FromClient = FromClient;
            type FromServer = FromServer;

            fn name() -> &'static str {
                #proto_name
            }
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub enum FromClient {
            #(#client_message_names,)*
            #(#client_query_requests,)*
            #(#server_query_responses,)*
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub enum FromServer {
            #(#server_message_names,)*
            #(#server_query_requests,)*
            #(#client_query_responses,)*
        }

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

        let name = input.parse::<syn::LitStr>()?.value();
        let _ = input.parse::<syn::Token![;]>().unwrap();

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
