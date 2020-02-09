#![allow(dead_code)]

// pub mod builder;
// // pub mod derive;
// pub mod from_bytes;
// pub mod result;
// pub mod selector;
// pub mod solidity;

pub use crate::error::{
    Error,
    Result,
};

pub mod encode;
pub mod bytesfix;
pub mod into_type;
pub mod bytes;
pub mod int;
pub mod address;
pub mod function;
pub mod string;
pub mod selector;
pub mod error;
pub mod builder;
pub mod test;
