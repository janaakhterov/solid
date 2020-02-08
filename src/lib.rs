#![allow(dead_code)]

#[macro_use]
extern crate anyhow;

// pub mod builder;
// // pub mod derive;
// pub mod error;
// pub mod from_bytes;
// pub mod result;
// pub mod selector;
// pub mod solidity;
// pub mod types;

// pub use crate::error::{
//     Error,
//     Result,
// };

// pub use types::{
//     i256,
//     u256,
// };

pub mod encode;
pub mod bytesfix;
pub mod into_type;
pub mod bytes;
pub mod int;
pub mod address;
pub mod function;
