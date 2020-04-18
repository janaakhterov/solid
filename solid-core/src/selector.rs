use crate::into_type::IntoType;
use sha3::{
    Digest,
    Keccak256,
};
use std::borrow::Cow;

/// Function signature builder
pub struct Selector {
    params: Vec<Cow<'static, str>>,
}

impl Selector {
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    pub fn push<T: IntoType>(mut self) -> Self {
        self.params.push(T::into_type());
        self
    }

    pub fn build(self, name: &str) -> [u8; 4] {
        let signature = format!("{}({})", name, self.params.join(","));
        let mut sig = [0; 4];
        let mut hasher = Keccak256::new();
        hasher.input(&signature);
        sig.copy_from_slice(&hasher.result()[0..4]);
        sig
    }
}
