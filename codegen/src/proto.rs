use matches2::option_match;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
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
    let input = syn::parse2::<Input>(ts)?.defs;

    let client_message_names = input
        .iter()
        .filter(|def| def.client())
        .filter_map(|def| option_match!(def, Def::Message(msg) => &msg.name))
        .map(|ident| quote!(#ident(#ident)));
    let server_message_names = input
        .iter()
        .filter(|def| def.server())
        .filter_map(|def| option_match!(def, Def::Message(msg) => &msg.name))
        .map(|ident| quote!(#ident(#ident)));

    let client_query_requests = input
        .iter()
        .filter(|def| def.client())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Request", ident);
            quote!(#ident(#ident))
        });
    let client_query_responses = input
        .iter()
        .filter(|def| def.client())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Response", ident);
            quote!(#ident(#ident))
        });

    let server_query_requests = input
        .iter()
        .filter(|def| def.server())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Request", ident);
            quote!(#ident(#ident))
        });
    let server_query_responses = input
        .iter()
        .filter(|def| def.server())
        .filter_map(|def| option_match!(def, Def::Query(msg) => &msg.name))
        .map(|ident| {
            let ident = &format_ident!("{}Response", ident);
            quote!(#ident(#ident))
        });

    let messages = input.iter().filter_map(|def| {
        option_match!(def, Def::Message(msg) => {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let fields = msg.fields.iter();

            quote! {
                #(#attrs)*
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #name { #(#fields),* }

                impl crate::proto::Message for #name {}
            }
        })
    });

    let queries = input.iter().filter_map(|def| {
        option_match!(def, Def::Query(msg) => {
            let attrs = &msg.attrs;
            let name = &msg.name;
            let req_name = &format_ident!("{}Request", name);
            let res_name = &format_ident!("{}Response", name);

            let request = msg.request.iter();
            let response = msg.response.iter();

            quote! {
                #(#attrs)*
                pub struct #name;
                impl crate::proto::Query for #name {
                    type Request = #req_name;
                    type Response = #res_name;
                }

                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #req_name { #(#request),* }
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub struct #res_name { #(#response),* }
            }
        })
    });

    let ret = quote! {
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
    defs: Vec<Def>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut defs = vec![];
        while !input.is_empty() {
            defs.push(input.parse()?);
        }
        Ok(Self { defs })
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
