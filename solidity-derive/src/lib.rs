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
    Field,
    Ident,
    Lifetime,
    LifetimeDef,
    Result,
    Token,
    Type,
};

// mod decode;
mod encode;

pub(crate) struct ItemField {
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) lifetimes: Option<Punctuated<Lifetime, Token![,]>>,
}

impl Parse for ItemField {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Option<Token![pub]> = input.parse().ok();
        let ident: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;

        let lifetimes: Option<Punctuated<Lifetime, Token![,]>> =
            if input.parse::<Token![<]>().is_ok() {
                let lifetimes: Punctuated<Lifetime, Token![,]> =
                    input.parse_terminated(Lifetime::parse)?;
                input.parse::<Option<Token![>]>>().ok();
                Some(lifetimes)
            } else {
                None
            };

        Ok(Self {
            ident,
            ty,
            lifetimes,
        })
    }
}

pub(crate) struct ItemStruct {
    pub(crate) ident: Ident,
    pub(crate) lifetimes: Option<LifetimeDef>,
    pub(crate) fields: Vec<ItemField>,
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut content;

        let _: Option<Token![pub]> = input.parse().ok();
        let _: Token![struct] = input.parse()?;
        let ident: Ident = input.parse()?;

        let _: Option<Token![<]> = input.parse().ok();
        let lifetimes: Option<LifetimeDef> = input.parse().ok();
        let _: Option<Token![>]> = input.parse().ok();

        braced!(content in input);

        // let mut fields = Vec::new();

        // loop {
        let fields: Punctuated<ItemField, Token![,]> =
            content.parse_terminated(ItemField::parse)?;

        let fields = fields.into_iter().collect::<Vec<ItemField>>();
        //     if content.parse::<Token![,]>().is_ok() {
        //         break;
        //     }
        // }

        Ok(ItemStruct {
            ident,
            lifetimes,
            fields,
        })
    }
}

#[proc_macro_derive(Encode)]
pub fn encode(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    TokenStream::from(encode::impl_encode(&ast))
}

// #[proc_macro_derive(Decode)]
// pub fn decode(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as ItemStruct);

//     TokenStream::from(decode::impl_decode(&ast))
// }

#[proc_macro_attribute]
pub fn solidity(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    (quote! {}).into()
}
