//! ### Basic usage of the crate without the "deser" nor "derive" features would be using `Builder` and `Selector`.
//!
//! ```rust
//! # use solid::{
//! #     Bytes10,
//! #     Builder,
//! # };
//! #
//! let function = Builder::new()
//!     .name("transfer")
//!     .push("daniel")
//!     .push(10u128)
//!     .push(Bytes10([1u8; 10]))
//!     .build();
//! ```
//!
//! ### Usage with the "derive" feature would look like.
//!
//! Use `#[solid(contstructor)]` if you're constructing a contract.
//! Otherwise the name of the struct and the field types will be used
//! to derive the function signature.
//!
//! ```rust
//! # use solid::{
//! #     Address,
//! #     Bytes,
//! #     derive::{
//! #         Encode,
//! #         Decode,
//! #     },
//! #     Encode,
//! #     Decode,
//! #     Uint256,
//! # };
//! #
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
//! ### Usage with the "deser" feature would look like.
//!
//! ```rust
//! # use solid::Bytes;
//! # use serde::{Serialize, Deserialize};
//! #
//! #[derive(Serialize)]
//! struct ContractCallComposite<'a> {
//!     to: (&'a str, u128),
//!     memos: &'a [&'a str],
//!     matrix: &'a [&'a [&'a [u8]]],
//! }
//!
//! #[derive(Deserialize)]
//! struct ContractCallResponse<'a> {
//!     // Uint256 is not supported.
//!     // int: Uint256,
//!     int: u128,
//!
//!     #[serde(borrow)]
//!     bytes: Bytes<'a>,
//!     // You can also do the following because
//!     // serde treats `&'a [u8]` as bytes
//!     // bytes: &'a [u8],
//!     //
//!     // However, note that if you want to get `uint8[]` using serde you'll need to use a vec
//!     // uint8array: Vec<u8>,
//!
//!     memo: &'a str,
//!     // Address is not supported.
//!     // address: Address,
//! }
//! ```
//!
//! ### Supported attribute key/value pairs for `Encode`:
//!
//! "rename": must have a value associated with it indicating the function name.
//! If this is key is not used, then the struct identifier is used as the function name.
//!
//! ```rust
//! # use solid::Bytes;
//! # use solid::derive::Encode;
//! # use solid::Encode;
//! #
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
//! ```rust
//! # use solid::derive::Encode;
//! # use solid::Encode;
//! #
//! #[derive(Encode)]
//! #[solid(constructor)]
//! struct Contract {
//!     creator: String,
//! }
//! ```
#[cfg(feature = "derive")]
pub use solid_derive as derive;

pub use solid_core::{
    address::Address,
    builder::Builder,
    bytes::Bytes,
    bytesfix,
    decode::Decode,
    encode::Encode,
    error::{
        Error,
        Result,
    },
    function::Function,
    int,
    selector::Selector,
};

#[cfg(feature = "deser")]
pub use solid_core::derive::{
    from_bytes,
    to_bytes,
};
