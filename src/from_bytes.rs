use crate::Error;
use crate::Result;
use crate::{i256, u256};
use std::convert::TryInto;
use std::mem;

pub trait FromBytes<'a, T> {
    fn from_bytes(value: &'a [u8], index: usize) -> Result<T>;
}

// Checks to see if a block (32 bytes) exists in the buffer.
fn valid_block(buf: &[u8], start: usize, end: usize) -> Result<()> {
    if let (Some(_), Some(_)) = (buf.get(start), buf.get(end - 1)) {
        Ok(())
    } else {
        Err(Error::Eof)
    }
}

impl<'a> FromBytes<'a, u256> for u256 {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<u256> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        Ok(u256(buf[index * 32..(index + 1) * 32].try_into()?))
    }
}

impl<'a> FromBytes<'a, Vec<u256>> for Vec<u256> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<u256>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let mut vec: Vec<u256> = Vec::new();

        let offset = offset + 32;

        for i in 0..len {
            valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

            vec.push(u256(
                buf[offset + i * 32..offset + (i + 1) * 32].try_into()?,
            ))
        }

        Ok(vec)
    }
}

impl<'a> FromBytes<'a, i256> for i256 {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<i256> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        Ok(i256(buf[index * 32..(index + 1) * 32].try_into()?))
    }
}

impl<'a> FromBytes<'a, Vec<i256>> for Vec<i256> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<i256>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let mut vec: Vec<i256> = Vec::new();

        let offset = offset + 32;

        for i in 0..len {
            valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

            vec.push(i256(
                buf[offset + i * 32..offset + (i + 1) * 32].try_into()?,
            ))
        }

        Ok(vec)
    }
}

impl<'a> FromBytes<'a, u8> for u8 {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<u8> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        Ok(<u8>::from_be_bytes(
            buf[(index + 1) * 32 - mem::size_of::<u8>()..(index + 1) * 32].try_into()?,
        ))
    }
}

// Vec<u8> is not supported because it fits `bytes` better.

macro_rules! impl_from_bytes {
    ($ty: ty) => {
        impl<'a> FromBytes<'a, $ty> for $ty {
            fn from_bytes(buf: &'a [u8], index: usize) -> Result<$ty> {
                valid_block(&buf, index * 32, (index + 1) * 32)?;
                Ok(<$ty>::from_be_bytes(
                    buf[(index + 1) * 32 - mem::size_of::<$ty>()..(index + 1) * 32].try_into()?,
                ))
            }
        }

        impl<'a> FromBytes<'a, Vec<$ty>> for Vec<$ty> {
            fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<$ty>> {
                valid_block(&buf, index * 32, (index + 1) * 32)?;
                let offset = usize::from_bytes(&buf, index)?;

                valid_block(&buf, offset, offset + 32)?;
                let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

                let mut vec: Vec<$ty> = Vec::new();

                let offset = offset + 32;

                for i in 0..len {
                    valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

                    let value = <$ty>::from_be_bytes(
                        buf[offset + ((i + 1) * 32) - mem::size_of::<$ty>()..offset + (i + 1) * 32]
                            .try_into()?,
                    );

                    vec.push(value);
                }

                Ok(vec)
            }
        }
    };
}

impl_from_bytes!(i8);
// impl_from_bytes!(u8);
impl_from_bytes!(i16);
impl_from_bytes!(u16);
impl_from_bytes!(i32);
impl_from_bytes!(u32);
impl_from_bytes!(i64);
impl_from_bytes!(u64);
impl_from_bytes!(i128);
impl_from_bytes!(u128);
impl_from_bytes!(usize);

impl<'a> FromBytes<'a, String> for String {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<String> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let value = std::str::from_utf8(&buf[offset + 32..offset + 32 + len])?.into();
        Ok(value)
    }
}

impl<'a> FromBytes<'a, Vec<String>> for Vec<String> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<String>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let mut vec: Vec<String> = Vec::new();

        let offset = offset + 32;

        for i in 0..len {
            valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

            let element_offset =
                usize::from_bytes(&buf[offset + (i * 32)..offset + ((i + 1) * 32)], 0)? as usize;

            let byte_len = u64::from_bytes(
                &buf[offset + element_offset..offset + element_offset + 32],
                0,
            )? as usize;

            let value = std::str::from_utf8(
                &buf[offset + element_offset + 32..offset + element_offset + 32 + byte_len],
            )?
            .into();

            vec.push(value);
        }

        Ok(vec)
    }
}

impl<'a> FromBytes<'a, Vec<u8>> for Vec<u8> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<u8>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let value = buf[offset + 32..offset + 32 + len].into();
        Ok(value)
    }
}

impl<'a> FromBytes<'a, Vec<Vec<u8>>> for Vec<Vec<u8>> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<Vec<u8>>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let mut vec: Vec<Vec<u8>> = Vec::new();

        let offset = offset + 32;

        for i in 0..len {
            valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

            let element_offset =
                usize::from_bytes(&buf[offset + (i * 32)..offset + ((i + 1) * 32)], 0)? as usize;

            let byte_len = u64::from_bytes(
                &buf[offset + element_offset..offset + element_offset + 32],
                0,
            )? as usize;

            let value =
                buf[offset + element_offset + 32..offset + element_offset + 32 + byte_len].into();

            vec.push(value);
        }

        Ok(vec)
    }
}
