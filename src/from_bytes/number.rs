use super::{
    valid_block,
    FromBytes,
};
use crate::{
    i256,
    u256,
    Result,
};
use std::{
    convert::TryInto,
    mem,
};

macro_rules! impl_from_bytes {
    ($ty: ty) => {
        impl<'a> FromBytes<'a, $ty> for $ty {
            fn from_bytes(buf: &'a [u8], index: usize) -> Result<$ty> {
                Ok(<$ty>::from_be_bytes(
                    buf[(index + 1) * 32 - mem::size_of::<$ty>()..(index + 1) * 32].try_into()?,
                ))
            }
        }

        // impl<'a> FromBytes<'a, Vec<$ty>> for Vec<$ty> {
        //     fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<$ty>> {
        //         valid_block(&buf, index * 32, (index + 1) * 32)?;
        //         let offset = usize::from_bytes(&buf, index)?;

        //         valid_block(&buf, offset, offset + 32)?;
        //         let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        //         let mut vec: Vec<$ty> = Vec::new();

        //         let offset = offset + 32;

        //         for i in 0..len {
        //             valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

        //             let value = <$ty>::from_be_bytes(
        //                 buf[offset + ((i + 1) * 32) - mem::size_of::<$ty>()..offset + (i + 1) * 32]
        //                     .try_into()?,
        //             );

        //             vec.push(value);
        //         }

        //         Ok(vec)
        //     }
        // }
    };
}

impl_from_bytes!(i8);
impl_from_bytes!(u8);
impl_from_bytes!(i16);
impl_from_bytes!(u16);
impl_from_bytes!(i32);
impl_from_bytes!(u32);
impl_from_bytes!(i64);
impl_from_bytes!(u64);
impl_from_bytes!(i128);
impl_from_bytes!(u128);
impl_from_bytes!(usize);

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

#[cfg(test)]
mod test {
    use crate::{
        i256,
        result::SolidityResult,
        u256,
    };

    #[test]
    #[rustfmt::skip]
    fn decode_numbers() -> anyhow::Result<()> {
        let bytes   = hex::decode("\
00000000000000000000000000000000000000000000000000000000000000ff\
00000000000000000000000000000000000000000000000000000000000000ff\
000000000000000000000000000000000000000000000000000000000000ffff\
000000000000000000000000000000000000000000000000000000000000ffff\
00000000000000000000000000000000000000000000000000000000ffffffff\
00000000000000000000000000000000000000000000000000000000ffffffff\
000000000000000000000000000000000000000000000000ffffffffffffffff\
000000000000000000000000000000000000000000000000ffffffffffffffff\
00000000000000000000000000000000ffffffffffffffffffffffffffffffff\
00000000000000000000000000000000ffffffffffffffffffffffffffffffff\
").unwrap();

        assert_eq!((&*bytes).get_param::<i8>(0)?,   0xffu8                                 as i8);
        assert_eq!((&*bytes).get_param::<u8>(1)?,   0xffu8);
        assert_eq!((&*bytes).get_param::<i16>(2)?,  0xffffu16                              as i16);
        assert_eq!((&*bytes).get_param::<u16>(3)?,  0xffffu16);
        assert_eq!((&*bytes).get_param::<i32>(4)?,  0xffffffffu32                          as i32);
        assert_eq!((&*bytes).get_param::<u32>(5)?,  0xffffffffu32);
        assert_eq!((&*bytes).get_param::<i64>(6)?,  0xffffffffffffffffu64                  as i64);
        assert_eq!((&*bytes).get_param::<u64>(7)?,  0xffffffffffffffffu64);
        assert_eq!((&*bytes).get_param::<i128>(8)?, 0xffffffffffffffffffffffffffffffffu128 as i128);
        assert_eq!((&*bytes).get_param::<u128>(9)?, 0xffffffffffffffffffffffffffffffffu128);
        
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn decode_numbers_array() -> anyhow::Result<()> {
        let bytes   = hex::decode("\
0000000000000000000000000000000000000000000000000000000000000180\
00000000000000000000000000000000000000000000000000000000000001e0\
0000000000000000000000000000000000000000000000000000000000000240\
00000000000000000000000000000000000000000000000000000000000002a0\
0000000000000000000000000000000000000000000000000000000000000300\
0000000000000000000000000000000000000000000000000000000000000360\
00000000000000000000000000000000000000000000000000000000000003c0\
0000000000000000000000000000000000000000000000000000000000000420\
0000000000000000000000000000000000000000000000000000000000000480\
00000000000000000000000000000000000000000000000000000000000004e0\
0000000000000000000000000000000000000000000000000000000000000540\
00000000000000000000000000000000000000000000000000000000000005a0\
0000000000000000000000000000000000000000000000000000000000000002\
00000000000000000000000000000000000000000000000000000000000000ff\
00000000000000000000000000000000000000000000000000000000000000ff\
0000000000000000000000000000000000000000000000000000000000000002\
00000000000000000000000000000000000000000000000000000000000000ff\
00000000000000000000000000000000000000000000000000000000000000ff\
0000000000000000000000000000000000000000000000000000000000000002\
000000000000000000000000000000000000000000000000000000000000ffff\
000000000000000000000000000000000000000000000000000000000000ffff\
0000000000000000000000000000000000000000000000000000000000000002\
000000000000000000000000000000000000000000000000000000000000ffff\
000000000000000000000000000000000000000000000000000000000000ffff\
0000000000000000000000000000000000000000000000000000000000000002\
00000000000000000000000000000000000000000000000000000000ffffffff\
00000000000000000000000000000000000000000000000000000000ffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
00000000000000000000000000000000000000000000000000000000ffffffff\
00000000000000000000000000000000000000000000000000000000ffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
000000000000000000000000000000000000000000000000ffffffffffffffff\
000000000000000000000000000000000000000000000000ffffffffffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
000000000000000000000000000000000000000000000000ffffffffffffffff\
000000000000000000000000000000000000000000000000ffffffffffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
00000000000000000000000000000000ffffffffffffffffffffffffffffffff\
00000000000000000000000000000000ffffffffffffffffffffffffffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
00000000000000000000000000000000ffffffffffffffffffffffffffffffff\
00000000000000000000000000000000ffffffffffffffffffffffffffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
0000000000000000000000000000000000000000000000000000000000000002\
ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
").unwrap();

        assert_eq!((&*bytes).get_param::<Vec<i8>>(0)?,   &[0xffu8                                  as i8;   2]);
        // Vec<u8> is unsupported because it fits solidity type `Bytes` better.
        // assert_eq!((&*bytes).get_param::<Vec<u8>>(1)?,   &[0xffu8;                                 2]);
        assert_eq!((&*bytes).get_param::<Vec<i16>>(2)?,  &[0xffffu16                               as i16;  2]);
        assert_eq!((&*bytes).get_param::<Vec<u16>>(3)?,  &[0xffffu16;                              2]);
        assert_eq!((&*bytes).get_param::<Vec<i32>>(4)?,  &[0xffffffffu32                           as i32;  2]);
        assert_eq!((&*bytes).get_param::<Vec<u32>>(5)?,  &[0xffffffffu32;                          2]);
        assert_eq!((&*bytes).get_param::<Vec<i64>>(6)?,  &[0xffffffffffffffffu64                   as i64;  2]);
        assert_eq!((&*bytes).get_param::<Vec<u64>>(7)?,  &[0xffffffffffffffffu64;                  2]);
        assert_eq!((&*bytes).get_param::<Vec<i128>>(8)?, &[0xffffffffffffffffffffffffffffffffu128  as i128; 2]);
        assert_eq!((&*bytes).get_param::<Vec<u128>>(9)?, &[0xffffffffffffffffffffffffffffffffu128; 2]);
        assert_eq!((&*bytes).get_param::<Vec<i256>>(10)?, &[i256([0xffu8; 32]); 2]);
        assert_eq!((&*bytes).get_param::<Vec<u256>>(11)?, &[u256([0xffu8; 32]); 2]);
        
        Ok(())
    }
}
