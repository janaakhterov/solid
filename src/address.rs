use std::convert::TryInto;
use crate::encode::Encode;

pub struct Address(pub [u8; 32]);

impl TryInto<Address> for &str {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        // Remove the `0x` prefix if the length suggests that.
        let s = match self.len() {
            40 => Ok(self),
            42 => Ok(self.split_at(2).1),
            length => Err(anyhow!(
                "Address string length expected to be 40 or 42, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&hex::decode(&s)?);
        Ok(Address(buf))
    }
}

impl TryInto<Address> for &[u8] {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        match self.len() {
            20 => Ok(()),
            length => Err(anyhow!(
                "Address string length expected to be 20, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&self);
        Ok(Address(buf))
    }
}

impl TryInto<Address> for &Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        match self.len() {
            20 => Ok(()),
            length => Err(anyhow!(
                "Address string length expected to be 20, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&self);
        Ok(Address(buf))
    }
}

impl TryInto<Address> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        match self.len() {
            20 => Ok(()),
            length => Err(anyhow!(
                "Address string length expected to be 20, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&self);
        Ok(Address(buf))
    }
}

impl Encode for Address {
    fn encode(self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}
