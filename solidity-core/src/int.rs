use crate::{
    decode::Decode,
    encode::Encode,
};
use std::{
    convert::TryInto,
    mem,
};

macro_rules! impl_encode_signed {
    ($ty: ty) => {
        impl Encode for $ty {
            fn encode(self) -> Vec<u8> {
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
            fn encode(self) -> Vec<u8> {
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

pub struct Int8(pub [u8; 32]);
pub struct Int16(pub [u8; 32]);
pub struct Int24(pub [u8; 32]);
pub struct Int32(pub [u8; 32]);
pub struct Int40(pub [u8; 32]);
pub struct Int48(pub [u8; 32]);
pub struct Int56(pub [u8; 32]);
pub struct Int64(pub [u8; 32]);
pub struct Int72(pub [u8; 32]);
pub struct Int80(pub [u8; 32]);
pub struct Int88(pub [u8; 32]);
pub struct Int96(pub [u8; 32]);
pub struct Int104(pub [u8; 32]);
pub struct Int112(pub [u8; 32]);
pub struct Int120(pub [u8; 32]);
pub struct Int128(pub [u8; 32]);
pub struct Int136(pub [u8; 32]);
pub struct Int144(pub [u8; 32]);
pub struct Int152(pub [u8; 32]);
pub struct Int160(pub [u8; 32]);
pub struct Int168(pub [u8; 32]);
pub struct Int176(pub [u8; 32]);
pub struct Int184(pub [u8; 32]);
pub struct Int192(pub [u8; 32]);
pub struct Int200(pub [u8; 32]);
pub struct Int208(pub [u8; 32]);
pub struct Int216(pub [u8; 32]);
pub struct Int224(pub [u8; 32]);
pub struct Int232(pub [u8; 32]);
pub struct Int240(pub [u8; 32]);
pub struct Int248(pub [u8; 32]);
pub struct Int256(pub [u8; 32]);

pub struct Uint8(pub [u8; 32]);
pub struct Uint16(pub [u8; 32]);
pub struct Uint24(pub [u8; 32]);
pub struct Uint32(pub [u8; 32]);
pub struct Uint40(pub [u8; 32]);
pub struct Uint48(pub [u8; 32]);
pub struct Uint56(pub [u8; 32]);
pub struct Uint64(pub [u8; 32]);
pub struct Uint72(pub [u8; 32]);
pub struct Uint80(pub [u8; 32]);
pub struct Uint88(pub [u8; 32]);
pub struct Uint96(pub [u8; 32]);
pub struct Uint104(pub [u8; 32]);
pub struct Uint112(pub [u8; 32]);
pub struct Uint120(pub [u8; 32]);
pub struct Uint128(pub [u8; 32]);
pub struct Uint136(pub [u8; 32]);
pub struct Uint144(pub [u8; 32]);
pub struct Uint152(pub [u8; 32]);
pub struct Uint160(pub [u8; 32]);
pub struct Uint168(pub [u8; 32]);
pub struct Uint176(pub [u8; 32]);
pub struct Uint184(pub [u8; 32]);
pub struct Uint192(pub [u8; 32]);
pub struct Uint200(pub [u8; 32]);
pub struct Uint208(pub [u8; 32]);
pub struct Uint216(pub [u8; 32]);
pub struct Uint224(pub [u8; 32]);
pub struct Uint232(pub [u8; 32]);
pub struct Uint240(pub [u8; 32]);
pub struct Uint248(pub [u8; 32]);
pub struct Uint256(pub [u8; 32]);

macro_rules! impl_encode_int {
    ($ty: ty) => {
        impl Encode for $ty {
            fn encode(self) -> Vec<u8> {
                self.0.to_vec()
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

impl_encode_int!(Int8);
impl_encode_int!(Uint8);
impl_encode_int!(Int16);
impl_encode_int!(Uint16);
impl_encode_int!(Int24);
impl_encode_int!(Uint24);
impl_encode_int!(Int32);
impl_encode_int!(Uint32);
impl_encode_int!(Int40);
impl_encode_int!(Uint40);
impl_encode_int!(Int48);
impl_encode_int!(Uint48);
impl_encode_int!(Int56);
impl_encode_int!(Uint56);
impl_encode_int!(Int64);
impl_encode_int!(Uint64);
impl_encode_int!(Int72);
impl_encode_int!(Uint72);
impl_encode_int!(Int80);
impl_encode_int!(Uint80);
impl_encode_int!(Int88);
impl_encode_int!(Uint88);
impl_encode_int!(Int96);
impl_encode_int!(Uint96);
impl_encode_int!(Int104);
impl_encode_int!(Uint104);
impl_encode_int!(Int112);
impl_encode_int!(Uint112);
impl_encode_int!(Int120);
impl_encode_int!(Uint120);
impl_encode_int!(Int128);
impl_encode_int!(Uint128);
impl_encode_int!(Int136);
impl_encode_int!(Uint136);
impl_encode_int!(Int144);
impl_encode_int!(Uint144);
impl_encode_int!(Int152);
impl_encode_int!(Uint152);
impl_encode_int!(Int160);
impl_encode_int!(Uint160);
impl_encode_int!(Int168);
impl_encode_int!(Uint168);
impl_encode_int!(Int176);
impl_encode_int!(Uint176);
impl_encode_int!(Int184);
impl_encode_int!(Uint184);
impl_encode_int!(Int192);
impl_encode_int!(Uint192);
impl_encode_int!(Int200);
impl_encode_int!(Uint200);
impl_encode_int!(Int208);
impl_encode_int!(Uint208);
impl_encode_int!(Int216);
impl_encode_int!(Uint216);
impl_encode_int!(Int224);
impl_encode_int!(Uint224);
impl_encode_int!(Int232);
impl_encode_int!(Uint232);
impl_encode_int!(Int240);
impl_encode_int!(Uint240);
impl_encode_int!(Int248);
impl_encode_int!(Uint248);
impl_encode_int!(Int256);
impl_encode_int!(Uint256);
