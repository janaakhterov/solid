use super::Error;
use super::Result;
use std::convert::TryInto;

pub trait FromBytes<'a, T> {
    fn from_bytes(value: &'a [u8]) -> Result<T>;
}

impl<'a> FromBytes<'a, i8> for i8 {
    fn from_bytes(value: &'a [u8]) -> Result<i8> {
        if value.len() < 1 {
            Err(Error::Message(
                "Invalid buffer length. Expected at least 1 byte".to_string(),
            ))
        } else {
            Ok(value[0] as i8)
        }
    }
}

impl<'a> FromBytes<'a, u8> for u8 {
    fn from_bytes(value: &'a [u8]) -> Result<u8> {
        if value.len() < 1 {
            Err(Error::Message(
                "Invalid buffer length. Expected at least 1 byte".to_string(),
            ))
        } else {
            Ok(value[0])
        }
    }
}

macro_rules! impl_from_bytes {
    ($ty: ty) => {
        impl<'a> FromBytes<'a, $ty> for $ty {
            fn from_bytes(value: &'a [u8]) -> Result<$ty> {
                Ok(<$ty>::from_be_bytes(value.try_into()?))
            }
        }
    };
}

impl_from_bytes!(i16);
impl_from_bytes!(u16);
impl_from_bytes!(i32);
impl_from_bytes!(u32);
impl_from_bytes!(i64);
impl_from_bytes!(u64);
impl_from_bytes!(i128);
impl_from_bytes!(u128);
impl_from_bytes!(usize);
