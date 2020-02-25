#[cfg(feature = "derive")]
pub use solidity_derive as derive;

pub use solidity_core::{
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
pub use solidity_core::bytesfix::nightly::*;

#[cfg(feature = "nightly")]
pub use solidity_core::int::nightly::*;

#[cfg(feature = "serde")]
pub use solidity_core::derive::to_bytes;

#[cfg(feature = "nightly")]
pub use solidity_core::bytesfix::nightly::*;

#[cfg(feature = "nightly")]
pub use solidity_core::int::nightly::*;

// #[cfg(feature = "fixed")]
// pub use solidity_core::fixed::*;
