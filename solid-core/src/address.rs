use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
    Error,
};
use std::{
    borrow::Cow,
    convert::{
        TryFrom,
        TryInto,
    },
};

/// Simple wrapper for the Solidity type `address`
pub struct Address(pub [u8; 32]);

impl TryFrom<&str> for Address {
    type Error = Error;

    fn try_from(value: &str) -> Result<Address, crate::Error> {
        // Remove the `0x` prefix if the length suggests that.
        let s = match value.len() {
            40 => value,
            42 => value.split_at(2).1,
            _ => value,
        };

        let slice = hex::decode(&s)?;
        let slice: [u8; 24] = slice.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Address, crate::Error> {
        let slice: [u8; 20] = value.try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl TryFrom<&Vec<u8>> for Address {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Address, crate::Error> {
        let slice: [u8; 20] = value.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl TryFrom<Vec<u8>> for Address {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Address, crate::Error> {
        let slice: [u8; 20] = value.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl Encode for Address {
    fn encode(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a> Decode<'a> for Address {
    fn decode(buf: &'a [u8]) -> Self {
        Address(buf[0..32].try_into().unwrap())
    }
}

impl IntoType for Address {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("address")
    }
}
