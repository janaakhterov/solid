use crate::{
    decode::Decode,
    encode::Encode,
};
use std::convert::TryInto;

pub struct Bytes1(pub [u8; 1]);
pub struct Bytes2(pub [u8; 2]);
pub struct Bytes3(pub [u8; 3]);
pub struct Bytes4(pub [u8; 4]);
pub struct Bytes5(pub [u8; 5]);
pub struct Bytes6(pub [u8; 6]);
pub struct Bytes7(pub [u8; 7]);
pub struct Bytes8(pub [u8; 8]);
pub struct Bytes9(pub [u8; 9]);
pub struct Bytes10(pub [u8; 10]);
pub struct Bytes11(pub [u8; 11]);
pub struct Bytes12(pub [u8; 12]);
pub struct Bytes13(pub [u8; 13]);
pub struct Bytes14(pub [u8; 14]);
pub struct Bytes15(pub [u8; 15]);
pub struct Bytes16(pub [u8; 16]);
pub struct Bytes17(pub [u8; 17]);
pub struct Bytes18(pub [u8; 18]);
pub struct Bytes19(pub [u8; 19]);
pub struct Bytes20(pub [u8; 20]);
pub struct Bytes21(pub [u8; 21]);
pub struct Bytes22(pub [u8; 22]);
pub struct Bytes23(pub [u8; 23]);
pub struct Bytes24(pub [u8; 24]);
pub struct Bytes25(pub [u8; 25]);
pub struct Bytes26(pub [u8; 26]);
pub struct Bytes27(pub [u8; 27]);
pub struct Bytes28(pub [u8; 28]);
pub struct Bytes29(pub [u8; 29]);
pub struct Bytes30(pub [u8; 30]);
pub struct Bytes31(pub [u8; 31]);
pub struct Bytes32(pub [u8; 32]);

macro_rules! impl_encode_bytesfix {
    ($ty: ident) => {
        impl Encode for $ty {
            fn encode(self) -> Vec<u8> {
                let mut buf = vec![0u8; 32];
                buf[0..self.0.len()].copy_from_slice(&self.0);
                buf
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
    };
}

impl_encode_bytesfix!(Bytes1);
impl_encode_bytesfix!(Bytes2);
impl_encode_bytesfix!(Bytes3);
impl_encode_bytesfix!(Bytes4);
impl_encode_bytesfix!(Bytes5);
impl_encode_bytesfix!(Bytes6);
impl_encode_bytesfix!(Bytes7);
impl_encode_bytesfix!(Bytes8);
impl_encode_bytesfix!(Bytes9);
impl_encode_bytesfix!(Bytes10);
impl_encode_bytesfix!(Bytes11);
impl_encode_bytesfix!(Bytes12);
impl_encode_bytesfix!(Bytes13);
impl_encode_bytesfix!(Bytes14);
impl_encode_bytesfix!(Bytes15);
impl_encode_bytesfix!(Bytes16);
impl_encode_bytesfix!(Bytes17);
impl_encode_bytesfix!(Bytes18);
impl_encode_bytesfix!(Bytes19);
impl_encode_bytesfix!(Bytes20);
impl_encode_bytesfix!(Bytes21);
impl_encode_bytesfix!(Bytes22);
impl_encode_bytesfix!(Bytes23);
impl_encode_bytesfix!(Bytes24);
impl_encode_bytesfix!(Bytes25);
impl_encode_bytesfix!(Bytes26);
impl_encode_bytesfix!(Bytes27);
impl_encode_bytesfix!(Bytes28);
impl_encode_bytesfix!(Bytes29);
impl_encode_bytesfix!(Bytes30);
impl_encode_bytesfix!(Bytes31);
impl_encode_bytesfix!(Bytes32);
