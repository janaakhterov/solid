use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use std::{
    array::LengthAtMost32,
    convert::TryFrom,
};

pub struct BytesFix<'a, const N: usize>(pub &'a [u8; N]);

impl<'a, const N: usize> Encode for BytesFix<'a, N> {
    fn encode(self) -> Vec<u8> {
        let mut buf = vec![0u8; 32];
        buf[0..N].copy_from_slice(&self.0[..]);
        buf
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a, const N: usize> IntoType for BytesFix<'a, N> {
    fn into_type() -> String {
        format!("bytes{}", N)
    }
}

pub trait LengthAtLeast1 {}

macro_rules! impl_length_at_least_1 {
    ($($N:literal)+) => {
        $(
            impl<T> LengthAtLeast1 for [T; $N] {}
        )+
    }
}

impl_length_at_least_1! {
    1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

impl<'a, const N: usize> Decode<'a> for BytesFix<'a, N>
where
    [u8; N]: LengthAtMost32 + LengthAtLeast1,
{
    fn decode(buf: &'a [u8]) -> Self {
        BytesFix::<N>(TryFrom::try_from(&buf[0..N]).unwrap())
    }
}
