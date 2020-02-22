use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use fixed::types::{
    I0F16,
    I0F8,
};
use std::convert::TryInto;

macro_rules! impl_fixed {
    ($ty: ident, $solidity_type: expr) => {
        impl<Frac> Encode for $ty<Frac> {
            fn encode(self) -> Vec<u8> {
                let mut vec = vec![0u8; 32];
                let bytes = self.to_be_bytes();
                vec[32 - bytes.len()..].copy_from_slice(&bytes);
                vec
            }

            fn required_len(&self) -> u64 {
                32
            }

            fn is_dynamic() -> bool {
                false
            }
        }

        impl<'a, Frac> Decode<'a> for $ty<Frac> {
            fn decode(buf: &[u8]) -> Self {
                $ty::from_be_bytes(buf.try_into().unwrap())
            }
        }

        impl<Frac> IntoType for $ty<Frac> {
            fn into_type() -> String {
                $solidity_type.to_string()
            }
        }
    };
}

impl_fixed!(I0F8, "fixed");
