use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use std::{
    convert::TryInto,
    mem,
};

macro_rules! impl_encode_signed {
    ($ty: ty) => {
        impl Encode for $ty {
            fn encode(&self) -> Vec<u8> {
                let bits = if self.to_be_bytes()[0] & 0x80 == 0x80 {
                    0xff
                } else {
                    0x00
                };
                let mut buf = vec![bits; 32];
                buf[32 - mem::size_of::<$ty>()..].copy_from_slice(&self.to_be_bytes());
                buf
            }

            fn required_len(&self) -> u64 {
                32
            }

            fn is_dynamic() -> bool {
                false
            }
        }
    };
}

macro_rules! impl_encode_unsigned {
    ($ty: ty) => {
        impl Encode for $ty {
            fn encode(&self) -> Vec<u8> {
                let mut buf = vec![0u8; 32];
                buf[32 - mem::size_of::<$ty>()..].copy_from_slice(&self.to_be_bytes());
                buf
            }

            fn required_len(&self) -> u64 {
                32
            }

            fn is_dynamic() -> bool {
                false
            }
        }
    };
}

impl_encode_signed!(i8);
impl_encode_unsigned!(u8);
impl_encode_signed!(i16);
impl_encode_unsigned!(u16);
impl_encode_signed!(i32);
impl_encode_unsigned!(u32);
impl_encode_signed!(i64);
impl_encode_unsigned!(u64);
impl_encode_signed!(i128);
impl_encode_unsigned!(u128);

impl<'a> Decode<'a> for i8 {
    fn decode(buf: &'a [u8]) -> Self {
        buf[31] as i8
    }
}

impl<'a> Decode<'a> for u8 {
    fn decode(buf: &'a [u8]) -> Self {
        buf[31]
    }
}

impl<'a> Decode<'a> for i16 {
    fn decode(buf: &'a [u8]) -> Self {
        i16::from_be_bytes(buf[30..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for u16 {
    fn decode(buf: &'a [u8]) -> Self {
        u16::from_be_bytes(buf[30..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for i32 {
    fn decode(buf: &'a [u8]) -> Self {
        i32::from_be_bytes(buf[28..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for u32 {
    fn decode(buf: &'a [u8]) -> Self {
        u32::from_be_bytes(buf[28..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for i64 {
    fn decode(buf: &'a [u8]) -> Self {
        i64::from_be_bytes(buf[24..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for u64 {
    fn decode(buf: &'a [u8]) -> Self {
        u64::from_be_bytes(buf[24..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for i128 {
    fn decode(buf: &'a [u8]) -> Self {
        i128::from_be_bytes(buf[16..32].try_into().unwrap())
    }
}

impl<'a> Decode<'a> for u128 {
    fn decode(buf: &'a [u8]) -> Self {
        u128::from_be_bytes(buf[16..32].try_into().unwrap())
    }
}

pub struct Int24(pub [u8; 3]);
pub struct Int40(pub [u8; 5]);
pub struct Int48(pub [u8; 6]);
pub struct Int56(pub [u8; 7]);
pub struct Int72(pub [u8; 9]);
pub struct Int80(pub [u8; 10]);
pub struct Int88(pub [u8; 11]);
pub struct Int96(pub [u8; 12]);
pub struct Int104(pub [u8; 13]);
pub struct Int112(pub [u8; 14]);
pub struct Int120(pub [u8; 15]);
pub struct Int136(pub [u8; 17]);
pub struct Int144(pub [u8; 18]);
pub struct Int152(pub [u8; 19]);
pub struct Int160(pub [u8; 20]);
pub struct Int168(pub [u8; 21]);
pub struct Int176(pub [u8; 22]);
pub struct Int184(pub [u8; 23]);
pub struct Int192(pub [u8; 24]);
pub struct Int200(pub [u8; 25]);
pub struct Int208(pub [u8; 26]);
pub struct Int216(pub [u8; 27]);
pub struct Int224(pub [u8; 28]);
pub struct Int232(pub [u8; 29]);
pub struct Int240(pub [u8; 30]);
pub struct Int248(pub [u8; 31]);
pub struct Int256(pub [u8; 32]);

pub struct Uint24(pub [u8; 3]);
pub struct Uint40(pub [u8; 5]);
pub struct Uint48(pub [u8; 6]);
pub struct Uint56(pub [u8; 7]);
pub struct Uint72(pub [u8; 9]);
pub struct Uint80(pub [u8; 10]);
pub struct Uint88(pub [u8; 11]);
pub struct Uint96(pub [u8; 12]);
pub struct Uint104(pub [u8; 13]);
pub struct Uint112(pub [u8; 14]);
pub struct Uint120(pub [u8; 15]);
pub struct Uint136(pub [u8; 17]);
pub struct Uint144(pub [u8; 18]);
pub struct Uint152(pub [u8; 19]);
pub struct Uint160(pub [u8; 20]);
pub struct Uint168(pub [u8; 21]);
pub struct Uint176(pub [u8; 22]);
pub struct Uint184(pub [u8; 23]);
pub struct Uint192(pub [u8; 24]);
pub struct Uint200(pub [u8; 25]);
pub struct Uint208(pub [u8; 26]);
pub struct Uint216(pub [u8; 27]);
pub struct Uint224(pub [u8; 28]);
pub struct Uint232(pub [u8; 29]);
pub struct Uint240(pub [u8; 30]);
pub struct Uint248(pub [u8; 31]);
pub struct Uint256(pub [u8; 32]);

macro_rules! impl_encode_int {
    ($ty: ident, $expr: expr) => {
        impl Encode for $ty {
            fn encode(&self) -> Vec<u8> {
                let mut value = vec![0u8; 32];
                value[32 - self.0.len()..32].copy_from_slice(&self.0);
                value
            }

            fn required_len(&self) -> u64 {
                32
            }

            fn is_dynamic() -> bool {
                false
            }
        }

        impl<'a> Decode<'a> for $ty {
            fn decode(buf: &'a [u8]) -> Self {
                $ty(buf[0..32].try_into().unwrap())
            }
        }

        impl IntoType for $ty {
            fn into_type() -> String {
                $expr.to_string()
            }
        }
    };
}

impl_encode_int!(Int24, "int24");
impl_encode_int!(Uint24, "uint24");
impl_encode_int!(Int40, "int40");
impl_encode_int!(Uint40, "uint40");
impl_encode_int!(Int48, "int48");
impl_encode_int!(Uint48, "uint48");
impl_encode_int!(Int56, "int56");
impl_encode_int!(Uint56, "uint56");
impl_encode_int!(Int72, "int72");
impl_encode_int!(Uint72, "uint72");
impl_encode_int!(Int80, "int80");
impl_encode_int!(Uint80, "uint80");
impl_encode_int!(Int88, "int88");
impl_encode_int!(Uint88, "uint88");
impl_encode_int!(Int96, "int96");
impl_encode_int!(Uint96, "uint96");
impl_encode_int!(Int104, "int104");
impl_encode_int!(Uint104, "uint104");
impl_encode_int!(Int112, "int112");
impl_encode_int!(Uint112, "uint112");
impl_encode_int!(Int120, "int120");
impl_encode_int!(Uint120, "uint120");
impl_encode_int!(Int136, "int136");
impl_encode_int!(Uint136, "uint136");
impl_encode_int!(Int144, "int144");
impl_encode_int!(Uint144, "uint144");
impl_encode_int!(Int152, "int152");
impl_encode_int!(Uint152, "uint152");
impl_encode_int!(Int160, "int160");
impl_encode_int!(Uint160, "uint160");
impl_encode_int!(Int168, "int168");
impl_encode_int!(Uint168, "uint168");
impl_encode_int!(Int176, "int176");
impl_encode_int!(Uint176, "uint176");
impl_encode_int!(Int184, "int184");
impl_encode_int!(Uint184, "uint184");
impl_encode_int!(Int192, "int192");
impl_encode_int!(Uint192, "uint192");
impl_encode_int!(Int200, "int200");
impl_encode_int!(Uint200, "uint200");
impl_encode_int!(Int208, "int208");
impl_encode_int!(Uint208, "uint208");
impl_encode_int!(Int216, "int216");
impl_encode_int!(Uint216, "uint216");
impl_encode_int!(Int224, "int224");
impl_encode_int!(Uint224, "uint224");
impl_encode_int!(Int232, "int232");
impl_encode_int!(Uint232, "uint232");
impl_encode_int!(Int240, "int240");
impl_encode_int!(Uint240, "uint240");
impl_encode_int!(Int248, "int248");
impl_encode_int!(Uint248, "uint248");
impl_encode_int!(Int256, "int256");
impl_encode_int!(Uint256, "uint256");
