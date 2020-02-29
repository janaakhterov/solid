extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

#[cfg(not(feature = "nightly"))]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Literal;

use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    parse_macro_input,
    DeriveInput,
    Ident,
    Result,
    Token,
};

mod decode;
mod encode;

#[derive(Debug)]
pub(crate) struct Solidity {
    ident: Ident,
    name: Option<Literal>,
}

impl Parse for Solidity {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let name = if let Ok(_) = input.parse::<Token![=]>() {
            Some(input.parse::<Literal>()?)
        } else {
            None
        };

        Ok(Self { ident, name })
    }
}

#[proc_macro_derive(Encode, attributes(solid))]
pub fn encode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    TokenStream::from(encode::impl_encode(&ast))
}

#[proc_macro_derive(Decode, attributes(solid))]
pub fn decode(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    TokenStream::from(decode::impl_decode(&mut ast))
}
