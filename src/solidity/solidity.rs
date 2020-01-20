use byteorder::{BigEndian, ByteOrder};

pub(crate) enum Type<'a> {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    I256(&'a [u8; 32]),

    #[cfg(feature = "U256")]
    U256(bigint::U256),

    #[cfg(not(feature = "U256"))]
    U256(&'a [u8; 32]),

    BytesN(usize, [u8; 32]),

    Bytes(&'a [u8]),

    String(&'a str),
    // Array(Vec<Type<'a>>),
}

impl<'a> Type<'a> {
    pub(crate) fn is_dynamic(&self) -> bool {
        use Type::*;

        match self {
            I8(_) => false,
            U8(_) => false,
            I16(_) => false,
            U16(_) => false,
            I32(_) => false,
            U32(_) => false,
            I64(_) => false,
            U64(_) => false,
            I128(_) => false,
            U128(_) => false,
            I256(_) => false,
            U256(_) => false,
            BytesN(_, _) => false,
            Bytes(_) => true,
            String(_) => true,
            // Array(_) => true,
        }
    }

    pub(crate) fn required_byte_len(&self) -> usize {
        use Type::*;

        match self {
            I8(_) => self.byte_len(),
            U8(_) => self.byte_len(),
            I16(_) => self.byte_len(),
            U16(_) => self.byte_len(),
            I32(_) => self.byte_len(),
            U32(_) => self.byte_len(),
            I64(_) => self.byte_len(),
            U64(_) => self.byte_len(),
            I128(_) => self.byte_len(),
            U128(_) => self.byte_len(),
            I256(_) => self.byte_len(),
            U256(_) => self.byte_len(),
            BytesN(_, _) => self.byte_len(),
            Bytes(_) => 32 + self.byte_len(),
            String(_) => 32 + self.byte_len(),
            // Array(value) => value
            //     .iter()
            //     .map(Type::required_byte_len)
            //     .fold(0, |sum, len| sum + len),
        }
    }

    fn byte_len(&self) -> usize {
        use Type::*;

        match self {
            I8(_) => 32,
            U8(_) => 32,
            I16(_) => 32,
            U16(_) => 32,
            I32(_) => 32,
            U32(_) => 32,
            I64(_) => 32,
            U64(_) => 32,
            I128(_) => 32,
            U128(_) => 32,
            I256(_) => 32,
            U256(_) => 32,
            BytesN(_, _) => 32,
            Bytes(value) => (value.len() / 32 + 1) * 32,
            String(value) => (value.as_bytes().len() / 32 + 1) * 32,
            // Array(value) => value
            //     .iter()
            //     .map(Type::byte_len)
            //     .fold(0, |sum, len| sum + len),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        use Type::*;

        match self {
            I8(_) => "int8".to_owned(),
            U8(_) => "uint8".to_owned(),
            I16(_) => "int16".to_owned(),
            U16(_) => "uint16".to_owned(),
            I32(_) => "int32".to_owned(),
            U32(_) => "uint32".to_owned(),
            I64(_) => "int64".to_owned(),
            U64(_) => "uint64".to_owned(),
            I128(_) => "int128".to_owned(),
            U128(_) => "uint128".to_owned(),
            I256(_) => "int256".to_owned(),
            U256(_) => "uint256".to_owned(),
            BytesN(len, _) => format!("bytes{}", len),
            Bytes(_) => "bytes".to_owned(),
            String(_) => "string".to_owned(),
            // Array(_) =>
        }
    }

    pub(crate) fn to_bytes(self) -> (bool, Vec<u8>) {
        use Type::*;

        let mut buf = vec![0; self.required_byte_len()];

        match self {
            I8(value) => buf[31] = value as u8,
            U8(value) => buf[31] = value,
            I16(value) => BigEndian::write_i16(&mut buf[30..32], value),
            U16(value) => BigEndian::write_u16(&mut buf[30..32], value),
            I32(value) => BigEndian::write_i32(&mut buf[28..32], value),
            U32(value) => BigEndian::write_u32(&mut buf[28..32], value),
            I64(value) => BigEndian::write_i64(&mut buf[24..32], value),
            U64(value) => BigEndian::write_u64(&mut buf[24..32], value),
            I128(value) => BigEndian::write_i128(&mut buf[16..32], value),
            U128(value) => BigEndian::write_u128(&mut buf[16..32], value),
            I256(value) => buf.copy_from_slice(value),

            #[cfg(feature = "U256")]
            U256(value) => value.to_big_endian(&mut buf[0..32]),

            #[cfg(not(feature = "U256"))]
            U256(value) => buf.copy_from_slice(value),

            BytesN(_, value) => buf.copy_from_slice(&value),

            Bytes(value) => {
                let len = Type::U64(value.len() as u64);
                buf[0..32].copy_from_slice(&len.to_bytes().1[0..32]);
                buf[32..32 + value.len()].copy_from_slice(&value[0..]);
            }

            String(value) => {
                let len = Type::U64(value.as_bytes().len() as u64);
                buf[0..32].copy_from_slice(&len.to_bytes().1[0..32]);
                buf[32..32 + value.as_bytes().len()].copy_from_slice(&value.as_bytes()[0..]);
            }
        }

        (self.is_dynamic(), buf)
    }
}
