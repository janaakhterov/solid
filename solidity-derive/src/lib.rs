extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

#[cfg(not(feature = "nightly"))]
extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{
    parse_macro_input,
    DeriveInput,
};

mod decode;
mod encode;

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    TokenStream::from(encode::impl_encode(&ast))
}

#[proc_macro_derive(Decode)]
pub fn decode(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    TokenStream::from(decode::impl_decode(&mut ast))
}

#[proc_macro_attribute]
pub fn solidity(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    (quote! {}).into()
}
