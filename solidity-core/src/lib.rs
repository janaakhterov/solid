#![allow(dead_code)]

pub mod address;
pub mod boolean;
pub mod builder;
pub mod bytes;
pub mod bytesfix;
pub mod decode;
pub mod encode;
pub mod error;
pub mod function;
pub mod int;
pub mod into_type;
pub mod selector;
pub mod string;
pub mod test;
pub mod tuples;

pub use crate::error::{
    Error,
    Result,
};

#[cfg(feature = "derive")]
pub mod derive;
