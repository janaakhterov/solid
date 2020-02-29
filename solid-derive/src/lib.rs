//! ### Example usage.
//!
//! ```rust
//! #[derive(Encode)]
//! #[solid(rename = "random_function")]
//! struct ContractCallComposite<'a> {
//!     to: (&'a str, u128),
//!     memos: &'a [&'a str],
//!     matrix: &'a [&'a [&'a [u8]]],
//! }
//!
//! #[derive(Decode)]
//! struct ContractCallResponse<'a> {
//!     int: Uint256,
//!     bytes: Bytes<'a>,
//!     memo: &'a str,
//!     address: Address,
//! }
//! ```
//!
//! ### Supported attribute key/value pairs for `Encode`:
//!
//! "rename": must have a value associated with it indicating the function name.
//! If this is key is not used, then the struct identifier is used as the function name.
//!
//! #### Exammple
//! ```rust
//! #[derive(Encode)]
//! #[solid(rename = "random_function")]
//! struct RandomFunction<'a> {
//!     memo: String,
//!     data: Bytes<'a>
//! }
//! ```
//!
//! "constructor": This indicates the function that is being called is a constructor and hence
//! should not have the function signature encoded in the buffer.
//! Note: The function signature in solidity is 4 bytes hash in the beginning of the buffer.
//!
//! #### Exammple
//! ```rust
//! #[derive(Encode)]
//! #[solid(constructor)]
//! struct Contract {
//!     creator: String,
//! }
//! ```
//!
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
