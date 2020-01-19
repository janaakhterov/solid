use byteorder::{BigEndian, ByteOrder};

pub trait SolidityResult {
    fn int8(&self, index: usize) -> i8;
    fn uint8(&self, index: usize) -> u8;
    fn int16(&self, index: usize) -> i16;
    fn uint16(&self, index: usize) -> u16;
    fn int32(&self, index: usize) -> i32;
    fn uint32(&self, index: usize) -> u32;
    fn int64(&self, index: usize) -> i64;
    fn uint64(&self, index: usize) -> u64;
    fn int128(&self, index: usize) -> i128;
    fn uint128(&self, index: usize) -> u128;
    fn int256(&self, index: usize) -> &[u8];

    #[cfg(feature = "U256")]
    fn uint256(&self, index: usize) -> bigint::U256;
    #[cfg(not(feature = "U256"))]
    fn uint256(&self, index: usize) -> &[u8];
}

impl<T> SolidityResult for T
where
    T: AsRef<[u8]>,
{
    #[inline(always)]
    fn int8(&self, index: usize) -> i8 {
        self.as_ref()[index * 32 + 31] as i8
    }

    #[inline(always)]
    fn uint8(&self, index: usize) -> u8 {
        self.as_ref()[index * 32 + 31]
    }

    #[inline(always)]
    fn int16(&self, index: usize) -> i16 {
        BigEndian::read_i16(&self.as_ref()[index * 32 + 30..(index + 1) * 32])
    }

    #[inline(always)]
    fn uint16(&self, index: usize) -> u16 {
        BigEndian::read_u16(&self.as_ref()[index * 32 + 30..(index + 1) * 32])
    }

    #[inline(always)]
    fn int32(&self, index: usize) -> i32 {
        BigEndian::read_i32(&self.as_ref()[index * 32 + 28..(index + 1) * 32])
    }

    #[inline(always)]
    fn uint32(&self, index: usize) -> u32 {
        BigEndian::read_u32(&self.as_ref()[index * 32 + 28..(index + 1) * 32])
    }

    #[inline(always)]
    fn int64(&self, index: usize) -> i64 {
        BigEndian::read_i64(&self.as_ref()[index * 32 + 24..(index + 1) * 32])
    }

    #[inline(always)]
    fn uint64(&self, index: usize) -> u64 {
        BigEndian::read_u64(&self.as_ref()[index * 32 + 24..(index + 1) * 32])
    }

    #[inline(always)]
    fn int128(&self, index: usize) -> i128 {
        BigEndian::read_i128(&self.as_ref()[index * 32 + 24..(index + 1) * 32])
    }

    #[inline(always)]
    fn uint128(&self, index: usize) -> u128 {
        BigEndian::read_u128(&self.as_ref()[index * 32 + 24..(index + 1) * 32])
    }

    #[inline(always)]
    fn int256(&self, index: usize) -> &[u8] {
        &self.as_ref()[index * 32 + 24..(index + 1) * 32]
    }

    #[cfg(feature = "U256")]
    #[inline(always)]
    fn uint256(&self, index: usize) -> bigint::U256 {
        bigint::U256::from(&self.as_ref()[index * 32 + 24..(index + 1) * 32])
    }

    #[cfg(not(feature = "U256"))]
    #[inline(always)]
    fn uint256(&self, index: usize) -> &[u8] {
        &self.as_ref()[index * 32 + 24..(index + 1) * 32]
    }
}
