use byteorder::{BigEndian, ByteOrder};

pub(crate) use into::IntoType;

mod address;
mod function;
mod into;
mod method;

#[cfg(test)]
mod test;

pub use address::Address;
pub use function::Function;

pub enum ConcreteSolidityType<'a> {
    I8(SolidityType, i8),
    U8(SolidityType, u8),
    I16(SolidityType, i16),
    U16(SolidityType, u16),
    I32(SolidityType, i32),
    U32(SolidityType, u32),
    I64(SolidityType, i64),
    U64(SolidityType, u64),
    I128(SolidityType, i128),
    U128(SolidityType, u128),
    I256(SolidityType, &'a [u8; 32]),

    #[cfg(feature = "U256")]
    U256(SolidityType, bigint::U256),

    #[cfg(not(feature = "U256"))]
    U256(SolidityType, &'a [u8; 32]),

    BytesN(SolidityType, [u8; 32]),
    Bytes(SolidityType, &'a [u8]),
    String(SolidityType, &'a str),
    Address(SolidityType, Address),
    Function(SolidityType, Function),

    Array(SolidityType, SolidityArray<'a>),
}

pub struct SolidityArray<'a> {
    pub dimensions: usize,
    pub array: Vec<ConcreteSolidityType<'a>>,
}

pub enum SolidityType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    I256,

    #[cfg(feature = "U256")]
    U256,

    #[cfg(not(feature = "U256"))]
    U256,

    BytesN(usize),
    Bytes,
    Address,
    Function,
    String,
}

impl SolidityType {
    fn to_string(&self) -> String {
        match self {
            SolidityType::I8 => "int8".to_owned(),
            SolidityType::U8 => "uint8".to_owned(),
            SolidityType::I16 => "int16".to_owned(),
            SolidityType::U16 => "uint16".to_owned(),
            SolidityType::I32 => "int32".to_owned(),
            SolidityType::U32 => "uint32".to_owned(),
            SolidityType::I64 => "int64".to_owned(),
            SolidityType::U64 => "uint64".to_owned(),
            SolidityType::I128 => "int128".to_owned(),
            SolidityType::U128 => "uint128".to_owned(),
            SolidityType::I256 => "int256".to_owned(),
            SolidityType::U256 => "uint256".to_owned(),
            SolidityType::BytesN(len) => format!("bytes{}", len),
            SolidityType::Bytes => "bytes".to_owned(),
            SolidityType::Address => "address".to_owned(),
            SolidityType::Function => "function".to_owned(),
            SolidityType::String => "string".to_owned(),
        }
    }

    fn is_dynamic(&self) -> bool {
        match self {
            SolidityType::I8 => false,
            SolidityType::U8 => false,
            SolidityType::I16 => false,
            SolidityType::U16 => false,
            SolidityType::I32 => false,
            SolidityType::U32 => false,
            SolidityType::I64 => false,
            SolidityType::U64 => false,
            SolidityType::I128 => false,
            SolidityType::U128 => false,
            SolidityType::I256 => false,
            SolidityType::U256 => false,
            SolidityType::BytesN(_) => false,
            SolidityType::Bytes => true,
            SolidityType::String => true,
            SolidityType::Address => false,
            SolidityType::Function => false,
        }
    }
}

impl<'a> ConcreteSolidityType<'a> {
    pub(crate) fn is_dynamic(&self) -> bool {
        match self {
            ConcreteSolidityType::I8(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::U8(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::I16(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::U16(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::I32(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::U32(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::I64(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::U64(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::I128(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::U128(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::I256(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::U256(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::BytesN(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::Bytes(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::String(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::Address(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::Function(ty, _) => ty.is_dynamic(),
            ConcreteSolidityType::Array(_, _) => true,
        }
    }

    pub(crate) fn required_byte_len(&self) -> usize {
        match self {
            ConcreteSolidityType::I8(_, _) => self.byte_len(),
            ConcreteSolidityType::U8(_, _) => self.byte_len(),
            ConcreteSolidityType::I16(_, _) => self.byte_len(),
            ConcreteSolidityType::U16(_, _) => self.byte_len(),
            ConcreteSolidityType::I32(_, _) => self.byte_len(),
            ConcreteSolidityType::U32(_, _) => self.byte_len(),
            ConcreteSolidityType::I64(_, _) => self.byte_len(),
            ConcreteSolidityType::U64(_, _) => self.byte_len(),
            ConcreteSolidityType::I128(_, _) => self.byte_len(),
            ConcreteSolidityType::U128(_, _) => self.byte_len(),
            ConcreteSolidityType::I256(_, _) => self.byte_len(),
            ConcreteSolidityType::U256(_, _) => self.byte_len(),
            ConcreteSolidityType::BytesN(_, _) => self.byte_len(),
            ConcreteSolidityType::Address(_, _) => self.byte_len(),
            ConcreteSolidityType::Function(_, _) => self.byte_len(),
            ConcreteSolidityType::Bytes(_, _) => 32 + self.byte_len(),
            ConcreteSolidityType::String(_, _) => 32 + self.byte_len(),
            ConcreteSolidityType::Array(ty, value) => {
                value
                    .array
                    .iter()
                    .map(ConcreteSolidityType::required_byte_len)
                    .fold(0, |sum, len| sum + len)
                    + if ty.is_dynamic() {
                        value.array.len() * 32
                    } else {
                        0
                    }
                    + 32
            }
        }
    }

    fn byte_len(&self) -> usize {
        match self {
            ConcreteSolidityType::I8(_, _) => 32,
            ConcreteSolidityType::U8(_, _) => 32,
            ConcreteSolidityType::I16(_, _) => 32,
            ConcreteSolidityType::U16(_, _) => 32,
            ConcreteSolidityType::I32(_, _) => 32,
            ConcreteSolidityType::U32(_, _) => 32,
            ConcreteSolidityType::I64(_, _) => 32,
            ConcreteSolidityType::U64(_, _) => 32,
            ConcreteSolidityType::I128(_, _) => 32,
            ConcreteSolidityType::U128(_, _) => 32,
            ConcreteSolidityType::I256(_, _) => 32,
            ConcreteSolidityType::U256(_, _) => 32,
            ConcreteSolidityType::BytesN(_, _) => 32,
            ConcreteSolidityType::Address(_, _) => 32,
            ConcreteSolidityType::Function(_, _) => 32,
            ConcreteSolidityType::Bytes(_, value) => (value.len() / 32 + 1) * 32,
            ConcreteSolidityType::String(_, value) => (value.as_bytes().len() / 32 + 1) * 32,
            ConcreteSolidityType::Array(_, value) => value
                .array
                .iter()
                .map(ConcreteSolidityType::byte_len)
                .fold(0, |sum, len| sum + len),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            ConcreteSolidityType::I8(ty, _) => ty.to_string(),
            ConcreteSolidityType::U8(ty, _) => ty.to_string(),
            ConcreteSolidityType::I16(ty, _) => ty.to_string(),
            ConcreteSolidityType::U16(ty, _) => ty.to_string(),
            ConcreteSolidityType::I32(ty, _) => ty.to_string(),
            ConcreteSolidityType::U32(ty, _) => ty.to_string(),
            ConcreteSolidityType::I64(ty, _) => ty.to_string(),
            ConcreteSolidityType::U64(ty, _) => ty.to_string(),
            ConcreteSolidityType::I128(ty, _) => ty.to_string(),
            ConcreteSolidityType::U128(ty, _) => ty.to_string(),
            ConcreteSolidityType::I256(ty, _) => ty.to_string(),
            ConcreteSolidityType::U256(ty, _) => ty.to_string(),
            ConcreteSolidityType::BytesN(ty, _) => ty.to_string(),
            ConcreteSolidityType::Address(ty, _) => ty.to_string(),
            ConcreteSolidityType::Function(ty, _) => ty.to_string(),
            ConcreteSolidityType::Bytes(ty, _) => ty.to_string(),
            ConcreteSolidityType::String(ty, _) => ty.to_string(),
            ConcreteSolidityType::Array(ty, value) => {
                format!("{}{}", ty.to_string(), "[]".repeat(value.dimensions))
            }
        }
    }

    pub(crate) fn to_bytes(self) -> (bool, Vec<u8>) {
        use ConcreteSolidityType::*;

        let mut buf = vec![0; self.required_byte_len()];

        let dynamic = self.is_dynamic();

        match self {
            I8(_, value) => buf[31] = value as u8,
            U8(_, value) => buf[31] = value,
            I16(_, value) => BigEndian::write_i16(&mut buf[30..32], value),
            U16(_, value) => BigEndian::write_u16(&mut buf[30..32], value),
            I32(_, value) => BigEndian::write_i32(&mut buf[28..32], value),
            U32(_, value) => BigEndian::write_u32(&mut buf[28..32], value),
            I64(_, value) => BigEndian::write_i64(&mut buf[24..32], value),
            U64(_, value) => BigEndian::write_u64(&mut buf[24..32], value),
            I128(_, value) => BigEndian::write_i128(&mut buf[16..32], value),
            U128(_, value) => BigEndian::write_u128(&mut buf[16..32], value),
            I256(_, value) => buf.copy_from_slice(value),

            #[cfg(feature = "U256")]
            U256(_, value) => value.to_big_endian(&mut buf[0..32]),

            #[cfg(not(feature = "U256"))]
            U256(_, value) => buf.copy_from_slice(value),

            BytesN(_, value) => buf.copy_from_slice(&value),
            Address(_, value) => buf[12..32].copy_from_slice(&value.0[0..20]),
            Function(_, value) => buf[8..32].copy_from_slice(&value.0[0..24]),

            Bytes(_, value) => {
                let len = ConcreteSolidityType::U64(SolidityType::U64, value.len() as u64);
                buf[0..32].copy_from_slice(&len.to_bytes().1[0..32]);
                buf[32..32 + value.len()].copy_from_slice(&value[0..]);
            }

            String(_, value) => {
                let len =
                    ConcreteSolidityType::U64(SolidityType::U64, value.as_bytes().len() as u64);
                buf[0..32].copy_from_slice(&len.to_bytes().1[0..32]);
                buf[32..32 + value.as_bytes().len()].copy_from_slice(&value.as_bytes()[0..]);
            }

            Array(ty, value) => {
                let mut offset = value.array.len() * 32 + if ty.is_dynamic() { 32 } else { 0 };

                let len = ConcreteSolidityType::U64(SolidityType::U64, value.array.len() as u64);

                let values = value
                    .array
                    .into_iter()
                    // We don't care about the values being dynamic or not
                    .map(|value| value.to_bytes().1)
                    .collect::<Vec<Vec<u8>>>();

                // Set length of array
                buf[0..32].copy_from_slice(&len.to_bytes().1[0..32]);

                for (index, bytes) in values.iter().enumerate() {
                    if ty.is_dynamic() {
                        BigEndian::write_u64(
                            &mut buf[(index + 1) * 32 + 24..(index + 2) * 32],
                            offset as u64,
                        );
                        buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
                        offset += bytes.len()
                    } else {
                        buf[(index + 1) * 32..(index + 2) * 32].copy_from_slice(&bytes);
                    }
                }
            }
        }

        (dynamic, buf)
    }
}
