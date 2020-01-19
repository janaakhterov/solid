use byteorder::{BigEndian, ByteOrder};

enum SolidityType<'a> {
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Int128(i128),
    Uint128(u128),
    Int256(&'a [u8; 32]),

    #[cfg(feature = "U256")]
    Uint256(bigint::U256),

    #[cfg(not(feature = "U256"))]
    Uint256(&'a [u8; 32]),
}

impl<'a> SolidityType<'a> {
    fn required_byte_len(&self) -> usize {
        use SolidityType::*;

        match self {
            Int8(_) => 32,
            Uint8(_) => 32,
            Int16(_) => 32,
            Uint16(_) => 32,
            Int32(_) => 32,
            Uint32(_) => 32,
            Int64(_) => 32,
            Uint64(_) => 32,
            Int128(_) => 32,
            Uint128(_) => 32,
            Int256(_) => 32,
            Uint256(_) => 32,
        }
    }

    pub(crate) fn to_bytes(self) -> Vec<u8> {
        use SolidityType::*;

        let mut buf = Vec::with_capacity(self.required_byte_len());

        match self {
            Int8(value) => buf[31] = value as u8,
            Uint8(value) => buf[31] = value,
            Int16(value) => BigEndian::write_i16(&mut buf[30..], value),
            Uint16(value) => BigEndian::write_u16(&mut buf[30..], value),
            Int32(value) => BigEndian::write_i32(&mut buf[28..], value),
            Uint32(value) => BigEndian::write_u32(&mut buf[28..], value),
            Int64(value) => BigEndian::write_i64(&mut buf[24..], value),
            Uint64(value) => BigEndian::write_u64(&mut buf[24..], value),
            Int128(value) => BigEndian::write_i128(&mut buf[16..], value),
            Uint128(value) => BigEndian::write_u128(&mut buf[16..], value),
            Int256(value) => buf.copy_from_slice(value),

            #[cfg(feature = "U256")]
            Uint256(value) => value.to_big_endian(&mut buf[0..]),

            #[cfg(not(feature = "U256"))]
            Uint256(value) => buf.copy_from_slice(value),
        }
        buf
    }
}

pub struct Builder<'a, T: AsRef<str>> {
    name: Option<T>,
    params: Vec<SolidityType<'a>>,
}

impl<'a, T: AsRef<str>> Builder<'a, T> {
    fn new(name: Option<T>) -> Self {
        Builder {
            name,
            params: Vec::new(),
        }
    }

    fn addInt8(mut self, value: i8) -> Self {
        self.params.push(SolidityType::Int8(value));
        self
    }

    fn addUint8(mut self, value: u8) -> Self {
        self.params.push(SolidityType::Uint8(value));
        self
    }

    fn addInt16(mut self, value: i16) -> Self {
        self.params.push(SolidityType::Int16(value));
        self
    }

    fn addUint16(mut self, value: u16) -> Self {
        self.params.push(SolidityType::Uint16(value));
        self
    }

    fn addInt32(mut self, value: i32) -> Self {
        self.params.push(SolidityType::Int32(value));
        self
    }

    fn addUint32(mut self, value: u32) -> Self {
        self.params.push(SolidityType::Uint32(value));
        self
    }

    fn addInt64(mut self, value: i64) -> Self {
        self.params.push(SolidityType::Int64(value));
        self
    }

    fn addUint64(mut self, value: u64) -> Self {
        self.params.push(SolidityType::Uint64(value));
        self
    }

    fn addInt128(mut self, value: i128) -> Self {
        self.params.push(SolidityType::Int128(value));
        self
    }

    fn addUint128(mut self, value: u128) -> Self {
        self.params.push(SolidityType::Uint128(value));
        self
    }

    fn addInt256(mut self, value: &'a [u8; 32]) -> Self {
        self.params.push(SolidityType::Int256(value));
        self
    }

    #[cfg(feature = "U256")]
    fn addUint256(mut self, value: bigint::U256) -> Self {
        self.params.push(SolidityType::Uint256(value));
        self
    }

    #[cfg(not(feature = "U256"))]
    fn addUint256(mut self, value: &'a [u8; 32]) -> Self {
        self.params.push(SolidityType::Uint256(value));
        self
    }
}
