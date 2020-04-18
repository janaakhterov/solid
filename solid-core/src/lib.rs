#![cfg_attr(feature = "nightly", feature(const_generics))]
#![cfg_attr(feature = "nightly", feature(const_generic_impls_guard))]
#![allow(dead_code)]

pub mod address;
pub mod boolean;
pub mod builder;
pub mod bytes;
pub mod decode;
pub mod encode;
pub mod error;
pub mod function;
pub mod into_type;
pub mod selector;
pub mod string;
pub mod tuples;

/// Container for all `bytes<M>` Solidity types
pub mod bytesfix;

/// Container for all `int<M>` Solidity types
pub mod int;

mod test;

pub use crate::error::{
    Error,
    Result,
};

#[cfg(feature = "derive")]
pub mod derive;

#[cfg(feature = "eth_types")]
pub mod ethereum_types;
