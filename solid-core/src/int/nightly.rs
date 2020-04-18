use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use std::{
    array::LengthAtMost32,
    convert::TryFrom,
};

/// A simpler structure to represent all Solidity `int<N>` types using const generics
///
/// Due to const generics not being stable, the user would be required to specify
/// both the size in bites and bytes (N and M respectively) for each type. N **MUST**
/// be equal to M * 8.
/// ```rust
/// let int256 = Int<256, 32>(&[0u8; 32]);
/// let int200 = Int<200, 25>(&[0u8; 25]);
/// ```
/// Note: It's very easy to missuse this structure because the even though the struct
/// requries both the number of bits and bytes to be supplied, it cannot validate that
/// N = M * 8 which is why it's not recommend to use this until const generics are stable.
pub struct Int<'a, const N: usize, const M: usize>(pub &'a [u8; M])
where
    [u8; M]: LengthAtMost32;

impl<'a, const N: usize, const M: usize> Encode for Int<'a, N, M>
where
    [u8; M]: LengthAtMost32,
{
    fn encode(&self) -> Vec<u8> {
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
    fn into_type() -> Cow<'static, str> {
        format!("int{}", N)
    }
}

/// A simpler structure to represent all Solidity `uint<N>` types using const generics
///
/// Due to const generics not being stable, the user would be required to specify
/// both the size in bites and bytes (N and M respectively) for each type. N **MUST**
/// be equal to M * 8.
/// ```rust
/// let int256 = Uint<256, 32>(&[0u8; 32]);
/// let int200 = Uint<200, 25>(&[0u8; 25]);
/// ```
/// Note: It's very easy to missuse this structure because the even though the struct
/// requries both the number of bits and bytes to be supplied, it cannot validate that
/// N = M * 8 which is why it's not recommend to use this until const generics are stable.
pub struct Uint<'a, const N: usize, const M: usize>(pub &'a [u8; M])
where
    [u8; M]: LengthAtMost32;

impl<'a, const N: usize, const M: usize> Encode for Uint<'a, N, M>
where
    [u8; M]: LengthAtMost32,
{
    fn encode(&self) -> Vec<u8> {
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
    fn into_type() -> Cow<'static, str> {
        format!("uint{}", N)
    }
}
