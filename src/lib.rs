//! ### Basic usage of the crate without the "serde" nor "derive" features would be using `Builder` and `Selector`.
//!
//! ```rust
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
//! ### Usage with the "serde" feature would look like.
//!
//! ```rust
//! #[derive(Serialize)]
//! #[solid(rename = "random_function")]
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
//!     bytes: Bytes<'a>,
//!     memo: &'a str,
//!     // Address is not supported.
//!     // address: Address,
//! }
//! ```
//!
#[cfg(feature = "derive")]
pub use solid_derive as derive;

pub use solid_core::{
    address::Address,
    builder::Builder,
    bytes::Bytes,
    bytesfix::stable::*,
    decode::Decode,
    encode::Encode,
    error::{
        Error,
        Result,
    },
    function::Function,
    int::stable::*,
    selector::Selector,
};

#[cfg(feature = "nightly")]
pub use solid_core::bytesfix::nightly::*;

#[cfg(feature = "nightly")]
pub use solid_core::int::nightly::*;

#[cfg(feature = "serde")]
pub use solid_core::derive::to_bytes;

#[cfg(feature = "nightly")]
pub use solid_core::bytesfix::nightly::*;

#[cfg(feature = "nightly")]
pub use solid_core::int::nightly::*;

// #[cfg(feature = "fixed")]
// pub use solid_core::fixed::*;
