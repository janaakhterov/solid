use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use std::{
    array::LengthAtMost32,
    convert::TryFrom,
};

pub struct Int<'a, const N: usize, const M: usize>(pub &'a [u8; M]);

impl<'a, const N: usize, const M: usize> Encode for Int<'a, N, M> {
    fn encode(self) -> Vec<u8> {
        let bits = if self.0[0] & 0x80 == 0x80 { 0xff } else { 0x00 };
        let mut buf = vec![bits; 32];
        buf[32 - N / 8..].copy_from_slice(&self.0[..]);
        buf
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a, const N: usize, const M: usize> Decode<'a> for Int<'a, N, M>
where
    [u8; M]: LengthAtMost32,
{
    fn decode(buf: &'a [u8]) -> Self {
        Int::<N, M>(TryFrom::try_from(&buf[32 - M..32]).unwrap())
    }
}

impl<'a, const N: usize, const M: usize> IntoType for Int<'a, N, M>
where
    [u8; M]: LengthAtMost32,
{
    fn into_type() -> String {
        format!("int{}", M)
    }
}

pub struct Uint<'a, const N: usize, const M: usize>(pub &'a [u8; M]);

impl<'a, const N: usize, const M: usize> Encode for Uint<'a, N, M> {
    fn encode(self) -> Vec<u8> {
        let bits = if self.0[0] & 0x80 == 0x80 { 0xff } else { 0x00 };
        let mut buf = vec![bits; 32];
        buf[32 - N / 8..].copy_from_slice(&self.0[..]);
        buf
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a, const N: usize, const M: usize> Decode<'a> for Uint<'a, N, M>
where
    [u8; M]: LengthAtMost32,
{
    fn decode(buf: &'a [u8]) -> Self {
        Uint::<N, M>(TryFrom::try_from(&buf[32 - M..32]).unwrap())
    }
}

impl<'a, const N: usize, const M: usize> IntoType for Uint<'a, N, M>
where
    [u8; M]: LengthAtMost32,
{
    fn into_type() -> String {
        format!("uint{}", M)
    }
}
