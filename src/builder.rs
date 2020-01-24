use crate::solidity::Address;
use crate::solidity::ConcreteSolidityType;
use crate::solidity::Function;
use crate::solidity::IntoType;
use crate::solidity::SolidityType;
use byteorder::{BigEndian, ByteOrder};
use sha3::{Digest, Keccak256};
use std::convert::TryInto;

pub struct Builder<'a> {
    name: Option<String>,
    pub(super) params: Vec<ConcreteSolidityType<'a>>,
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

    pub fn add<F: IntoType<'a>>(mut self, value: F) -> Self {
        self.params.push(value.into_type());
        self
    }

    pub fn add_address<F: TryInto<Address>>(mut self, value: F) -> Result<Self, F::Error> {
        let address: Address = value.try_into()?;
        self.params.push(ConcreteSolidityType::Address(
            SolidityType::Address,
            address,
        ));

        Ok(self)
    }

    pub fn add_function<F: TryInto<Function>>(mut self, value: F) -> Result<Self, F::Error> {
        let function: Function = value.try_into()?;
        self.params.push(ConcreteSolidityType::Function(
            SolidityType::Function,
            function,
        ));

        Ok(self)
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
                    .map(ConcreteSolidityType::to_string)
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
            .map(ConcreteSolidityType::required_byte_len)
            .zip(self.params.iter().map(ConcreteSolidityType::is_dynamic))
            .fold(
                0,
                |sum, (len, dynamic)| if dynamic { 32 + sum + len } else { sum + len },
            );

        println!("total_len: {}", total_len);

        let mut buf: Vec<u8> = vec![0; total_len + name_offset];

        let mut offset: usize = self.params.len() * 32 + name_offset;

        for (index, (dynamic, bytes)) in self
            .params
            .into_iter()
            .map(ConcreteSolidityType::to_bytes)
            .into_iter()
            .enumerate()
        {
            if dynamic {
                BigEndian::write_u64(
                    &mut buf[index * 32 + 24 + name_offset..(index + 1) * 32 + name_offset],
                    offset as u64,
                );
                buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
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
