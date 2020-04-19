use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use std::{
    array::LengthAtMost32,
    borrow::Cow,
    convert::TryFrom,
};

/// A simpler structure to represent all Solidity `bytes<N>` types using const generics
/// ```rust
/// let bytes10 = Bytes<10>(&[0u8; 10]);
/// ```
pub struct BytesFix<'a, const N: usize>(pub &'a [u8; N]);

impl<'a, const N: usize> Encode for BytesFix<'a, N> {
    fn encode(&self) -> Vec<u8> {
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
    fn into_type() -> Cow<'static, str> {
        Cow::Owned(format!("bytes{}", N))
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

pub type Bytes1<'a> = BytesFix<'a, 1>;
pub type Bytes2<'a> = BytesFix<'a, 2>;
pub type Bytes3<'a> = BytesFix<'a, 3>;
pub type Bytes4<'a> = BytesFix<'a, 4>;
pub type Bytes5<'a> = BytesFix<'a, 5>;
pub type Bytes6<'a> = BytesFix<'a, 6>;
pub type Bytes7<'a> = BytesFix<'a, 7>;
pub type Bytes8<'a> = BytesFix<'a, 8>;
pub type Bytes9<'a> = BytesFix<'a, 9>;
pub type Bytes10<'a> = BytesFix<'a, 10>;
pub type Bytes11<'a> = BytesFix<'a, 11>;
pub type Bytes12<'a> = BytesFix<'a, 12>;
pub type Bytes13<'a> = BytesFix<'a, 13>;
pub type Bytes14<'a> = BytesFix<'a, 14>;
pub type Bytes15<'a> = BytesFix<'a, 15>;
pub type Bytes16<'a> = BytesFix<'a, 16>;
pub type Bytes17<'a> = BytesFix<'a, 17>;
pub type Bytes18<'a> = BytesFix<'a, 18>;
pub type Bytes19<'a> = BytesFix<'a, 19>;
pub type Bytes20<'a> = BytesFix<'a, 20>;
pub type Bytes21<'a> = BytesFix<'a, 21>;
pub type Bytes22<'a> = BytesFix<'a, 22>;
pub type Bytes23<'a> = BytesFix<'a, 23>;
pub type Bytes24<'a> = BytesFix<'a, 24>;
pub type Bytes25<'a> = BytesFix<'a, 25>;
pub type Bytes26<'a> = BytesFix<'a, 26>;
pub type Bytes27<'a> = BytesFix<'a, 27>;
pub type Bytes28<'a> = BytesFix<'a, 28>;
pub type Bytes29<'a> = BytesFix<'a, 29>;
pub type Bytes30<'a> = BytesFix<'a, 30>;
pub type Bytes31<'a> = BytesFix<'a, 31>;
pub type Bytes32<'a> = BytesFix<'a, 32>;
