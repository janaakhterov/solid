#[cfg(feature = "derive")]
pub use solidity_derive as derive;

pub use solidity_core::{
    address::Address,
    builder::Builder,
    bytes::Bytes,
    bytesfix::*,
    decode::Decode,
    encode::Encode,
    error::{
        Error,
        Result,
    },
    function::Function,
    int::*,
    selector::Selector,
};

#[cfg(feature = "serde")]
pub use solidity_core::derive::to_bytes;

#[cfg(feature = "fixed")]
pub use solidity_core::fixed::*;
