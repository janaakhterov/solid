extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

#[cfg(not(feature = "nightly"))]
extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{
    braced,
    parse::{
        Parse,
        ParseStream,
    },
    parse_macro_input,
    punctuated::Punctuated,
    token,
    Field,
    Ident,
    Result,
    Token,
};

mod encode;
mod decode;

pub(crate) struct ItemStruct {
    _struct_token: Token![struct],
    ident: Ident,
    _brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            _struct_token: input.parse()?,
            ident: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    TokenStream::from(encode::impl_encode(&ast))
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    TokenStream::from(decode::impl_decode(&ast))
}

#[proc_macro_attribute]
pub fn function_name(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    // let ast = parse_macro_input!(item as ItemStruct);
    (quote! {}).into()
}
