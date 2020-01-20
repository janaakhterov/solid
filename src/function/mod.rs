use crate::solidity::SolidityBytesN;
use crate::solidity::Type;
use byteorder::{BigEndian, ByteOrder};
use sha3::{Digest, Keccak256};

pub struct Builder<'a> {
    name: Option<String>,
    params: Vec<Type<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            name: None,
            params: Vec::new(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add<F: Into<Type<'a>>>(mut self, value: F) -> Self {
        self.params.push(value.into());
        self
    }

    pub fn add_bytes_n<F: SolidityBytesN + AsRef<[u8]>>(mut self, value: F) -> Self {
        let mut bytes = [0; 32];
        bytes[0..value.as_ref().len()].copy_from_slice(&value.as_ref());
        self.params
            .push(Type::BytesN(value.solidity_bytes_len(), bytes));
        self
    }

    pub fn add_i256(mut self, value: &'a [u8; 32]) -> Self {
        self.params.push(Type::I256(value));
        self
    }

    #[cfg(feature = "U256")]
    pub fn add_u256(mut self, value: bigint::U256) -> Self {
        self.params.push(Type::U256(value));
        self
    }

    #[cfg(not(feature = "U256"))]
    pub fn add_u256(mut self, value: &'a [u8; 32]) -> Self {
        self.params.push(Type::U256(value));
        self
    }

    pub fn add_bytes(mut self, value: &'a [u8]) -> Self {
        self.params.push(Type::Bytes(value));
        self
    }

    pub fn add_string(mut self, value: &'a str) -> Self {
        self.params.push(Type::String(value));
        self
    }

    pub fn build(self) -> Vec<u8> {
        let name_offset = match self.name {
            None => 0,
            Some(_) => 4,
        };

        let sig: Option<[u8; 4]> = if let Some(name) = self.name {
            let mut sig = [0; 4];
            let mut hasher = Keccak256::new();
            let function = format!(
                "{}({})",
                name,
                self.params
                    .iter()
                    .map(Type::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            );
            hasher.input(&function);
            sig.copy_from_slice(&hasher.result());
            Some(sig)
        } else {
            None
        };

        let total_len = self
            .params
            .iter()
            .map(Type::required_byte_len)
            .zip(self.params.iter().map(Type::is_dynamic))
            .fold(
                0,
                |sum, (len, dynamic)| if dynamic { 32 + sum + len } else { sum + len },
            );

        let mut buf: Vec<u8> = vec![0; total_len + name_offset];

        let mut offset: usize = self.params.len() * 32 + name_offset;

        for (index, (dynamic, bytes)) in self
            .params
            .into_iter()
            .map(Type::to_bytes)
            .into_iter()
            .enumerate()
        {
            if dynamic {
                BigEndian::write_u64(
                    &mut buf[index * 32 + 24 + name_offset..(index + 1) * 32 + name_offset],
                    offset as u64,
                );
                buf[offset..].copy_from_slice(&bytes);
                offset += bytes.len()
            } else {
                buf[index * 32 + name_offset..(index + 1) * 32 + name_offset]
                    .copy_from_slice(&bytes);
            }
        }

        if let Some(sig) = sig {
            buf.copy_from_slice(&sig)
        }

        buf
    }
}
