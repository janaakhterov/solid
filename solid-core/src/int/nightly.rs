use crate::{
    bytesfix::LengthAtLeast1,
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use std::{
    array::LengthAtMost32,
    borrow::Cow,
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
    [u8; M]: LengthAtMost32 + LengthAtLeast1;

impl<'a, const N: usize, const M: usize> Encode for Int<'a, N, M>
where
    [u8; M]: LengthAtMost32 + LengthAtLeast1,
{
    fn encode(&self) -> Vec<u8> {
        let bits = if self.0[0] & 0x80 == 0x80 { 0xff } else { 0x00 };
        let mut buf = vec![bits; 32];
        buf[32 - N / 8..].copy_from_slice(&self.0[..]);
        buf
    }
}

impl<'a, const N: usize, const M: usize> Decode<'a> for Int<'a, N, M>
where
    [u8; M]: LengthAtMost32 + LengthAtLeast1,
{
    fn decode(buf: &'a [u8]) -> Self {
        Int::<N, M>(TryFrom::try_from(&buf[32 - M..32]).unwrap())
    }
}

impl<'a, const N: usize, const M: usize> IntoType for Int<'a, N, M>
where
    [u8; M]: LengthAtMost32 + LengthAtLeast1,
{
    fn into_type() -> Cow<'static, str> {
        Cow::Owned(format!("int{}", N))
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
    [u8; M]: LengthAtMost32 + LengthAtLeast1;

impl<'a, const N: usize, const M: usize> Encode for Uint<'a, N, M>
where
    [u8; M]: LengthAtMost32 + LengthAtLeast1,
{
    fn encode(&self) -> Vec<u8> {
        let bits = if self.0[0] & 0x80 == 0x80 { 0xff } else { 0x00 };
        let mut buf = vec![bits; 32];
        buf[32 - N / 8..].copy_from_slice(&self.0[..]);
        buf
    }
}

impl<'a, const N: usize, const M: usize> Decode<'a> for Uint<'a, N, M>
where
    [u8; M]: LengthAtMost32 + LengthAtLeast1,
{
    fn decode(buf: &'a [u8]) -> Self {
        Uint::<N, M>(TryFrom::try_from(&buf[32 - M..32]).unwrap())
    }
}

impl<'a, const N: usize, const M: usize> IntoType for Uint<'a, N, M>
where
    [u8; M]: LengthAtMost32 + LengthAtLeast1,
{
    fn into_type() -> Cow<'static, str> {
        Cow::Owned(format!("uint{}", N))
    }
}

pub type Int24<'a> = Int<'a, 24, 3>;
pub type Int40<'a> = Int<'a, 40, 5>;
pub type Int48<'a> = Int<'a, 48, 6>;
pub type Int56<'a> = Int<'a, 56, 7>;
pub type Int72<'a> = Int<'a, 72, 9>;
pub type Int80<'a> = Int<'a, 80, 10>;
pub type Int88<'a> = Int<'a, 88, 11>;
pub type Int96<'a> = Int<'a, 96, 12>;
pub type Int104<'a> = Int<'a, 104, 13>;
pub type Int112<'a> = Int<'a, 112, 14>;
pub type Int120<'a> = Int<'a, 120, 15>;
pub type Int136<'a> = Int<'a, 136, 17>;
pub type Int144<'a> = Int<'a, 144, 18>;
pub type Int152<'a> = Int<'a, 152, 19>;
pub type Int160<'a> = Int<'a, 160, 20>;
pub type Int168<'a> = Int<'a, 168, 21>;
pub type Int176<'a> = Int<'a, 176, 22>;
pub type Int184<'a> = Int<'a, 184, 23>;
pub type Int192<'a> = Int<'a, 192, 24>;
pub type Int200<'a> = Int<'a, 200, 25>;
pub type Int208<'a> = Int<'a, 208, 26>;
pub type Int216<'a> = Int<'a, 216, 27>;
pub type Int224<'a> = Int<'a, 224, 28>;
pub type Int232<'a> = Int<'a, 232, 29>;
pub type Int240<'a> = Int<'a, 240, 30>;
pub type Int248<'a> = Int<'a, 248, 31>;
pub type Int256<'a> = Int<'a, 256, 32>;

pub type Uint24<'a> = Int<'a, 24, 3>;
pub type Uint40<'a> = Int<'a, 40, 5>;
pub type Uint48<'a> = Int<'a, 48, 6>;
pub type Uint56<'a> = Int<'a, 56, 7>;
pub type Uint72<'a> = Int<'a, 72, 9>;
pub type Uint80<'a> = Int<'a, 80, 10>;
pub type Uint88<'a> = Int<'a, 88, 11>;
pub type Uint96<'a> = Int<'a, 96, 12>;
pub type Uint104<'a> = Int<'a, 104, 13>;
pub type Uint112<'a> = Int<'a, 112, 14>;
pub type Uint120<'a> = Int<'a, 120, 15>;
pub type Uint136<'a> = Int<'a, 136, 17>;
pub type Uint144<'a> = Int<'a, 144, 18>;
pub type Uint152<'a> = Int<'a, 152, 19>;
pub type Uint160<'a> = Int<'a, 160, 20>;
pub type Uint168<'a> = Int<'a, 168, 21>;
pub type Uint176<'a> = Int<'a, 176, 22>;
pub type Uint184<'a> = Int<'a, 184, 23>;
pub type Uint192<'a> = Int<'a, 192, 24>;
pub type Uint200<'a> = Int<'a, 200, 25>;
pub type Uint208<'a> = Int<'a, 208, 26>;
pub type Uint216<'a> = Int<'a, 216, 27>;
pub type Uint224<'a> = Int<'a, 224, 28>;
pub type Uint232<'a> = Int<'a, 232, 29>;
pub type Uint240<'a> = Int<'a, 240, 30>;
pub type Uint248<'a> = Int<'a, 248, 31>;
pub type Uint256<'a> = Int<'a, 256, 32>;
