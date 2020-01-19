use byteorder::{BigEndian, ByteOrder};

enum SolidityType {
    Int32(i32),
    Uint32(u32),
}

impl Into<Vec<u8>> for SolidityType {
    fn into(self) -> Vec<u8> {
        use SolidityType::*;

        let mut buf = [0u8; 32].to_vec();
        match self {
            Int32(value) => BigEndian::write_i32(&mut buf[28..], value),
            Uint32(value) => BigEndian::write_u32(&mut buf[28..], value),
        }
        buf
    }
}

pub struct Builder<T: AsRef<str>> {
    name: Option<T>,
    params: Vec<SolidityType>,
}

impl<T: AsRef<str>> Builder<T> {
    fn new(name: Option<T>) -> Self {
        Builder {
            name,
            params: Vec::new(),
        }
    }

    fn addInt32(mut self, value: i32) -> Self {
        self.params.push(SolidityType::Int32(value));
        self
    }

    fn addUint32(mut self, value: u32) -> Self {
        self.params.push(SolidityType::Uint32(value));
        self
    }
}
