use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
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

macro_rules! impl_into_type_and_encode_bytesfix {
    ($ty: ident, $expr: expr) => {
        impl Encode for $ty {
            fn encode(&self) -> Vec<u8> {
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
        impl IntoType for $ty {
            fn into_type() -> String {
                ($expr).to_string()
            }
        }
    };
}

impl_into_type_and_encode_bytesfix!(Bytes1, "bytes1");
impl_into_type_and_encode_bytesfix!(Bytes2, "bytes2");
impl_into_type_and_encode_bytesfix!(Bytes3, "bytes3");
impl_into_type_and_encode_bytesfix!(Bytes4, "bytes4");
impl_into_type_and_encode_bytesfix!(Bytes5, "bytes5");
impl_into_type_and_encode_bytesfix!(Bytes6, "bytes6");
impl_into_type_and_encode_bytesfix!(Bytes7, "bytes7");
impl_into_type_and_encode_bytesfix!(Bytes8, "bytes8");
impl_into_type_and_encode_bytesfix!(Bytes9, "bytes9");
impl_into_type_and_encode_bytesfix!(Bytes10, "bytes10");
impl_into_type_and_encode_bytesfix!(Bytes11, "bytes11");
impl_into_type_and_encode_bytesfix!(Bytes12, "bytes12");
impl_into_type_and_encode_bytesfix!(Bytes13, "bytes13");
impl_into_type_and_encode_bytesfix!(Bytes14, "bytes14");
impl_into_type_and_encode_bytesfix!(Bytes15, "bytes15");
impl_into_type_and_encode_bytesfix!(Bytes16, "bytes16");
impl_into_type_and_encode_bytesfix!(Bytes17, "bytes17");
impl_into_type_and_encode_bytesfix!(Bytes18, "bytes18");
impl_into_type_and_encode_bytesfix!(Bytes19, "bytes19");
impl_into_type_and_encode_bytesfix!(Bytes20, "bytes20");
impl_into_type_and_encode_bytesfix!(Bytes21, "bytes21");
impl_into_type_and_encode_bytesfix!(Bytes22, "bytes22");
impl_into_type_and_encode_bytesfix!(Bytes23, "bytes23");
impl_into_type_and_encode_bytesfix!(Bytes24, "bytes24");
impl_into_type_and_encode_bytesfix!(Bytes25, "bytes25");
impl_into_type_and_encode_bytesfix!(Bytes26, "bytes26");
impl_into_type_and_encode_bytesfix!(Bytes27, "bytes27");
impl_into_type_and_encode_bytesfix!(Bytes28, "bytes28");
impl_into_type_and_encode_bytesfix!(Bytes29, "bytes29");
impl_into_type_and_encode_bytesfix!(Bytes30, "bytes30");
impl_into_type_and_encode_bytesfix!(Bytes31, "bytes31");
impl_into_type_and_encode_bytesfix!(Bytes32, "bytes32");
