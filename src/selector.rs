use crate::into_type::IntoType;
use sha3::{
    Digest,
    Keccak256,
};

pub struct Selector {
    params: Vec<String>,
}

impl Selector {
    pub fn new() -> Self {
        Selector { params: Vec::new() }
    }

    pub fn add<T: IntoType>(mut self) -> Self {
        self.params.push(T::into_type());
        self
    }

    pub fn build(self, name: String) -> [u8; 4] {
        let signature = format!("{}({})", name, self.params.join(","));
        let mut sig = [0; 4];
        let mut hasher = Keccak256::new();
        hasher.input(&signature);
        sig.copy_from_slice(&hasher.result());
        sig
    }
}
