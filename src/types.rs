use std::ops::Deref;
use std::ops::DerefMut;

/// Simple wrapper types for `i256` and `u256`.

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct u256(pub [u8; 32]);

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct i256(pub [u8; 32]);

impl Deref for u256 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for i256 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for u256 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for i256 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
