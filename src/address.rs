use std::convert::TryInto;
use crate::encode::Encode;

pub struct Address(pub [u8; 32]);

impl TryInto<Address> for &str {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        // Remove the `0x` prefix if the length suggests that.
        let s = match self.len() {
            40 => self,
            42 => self.split_at(2).1,
            _ => self,
        };

        let slice = hex::decode(&s)?;
        let slice: [u8; 24] = slice.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl TryInto<Address> for &[u8] {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        let slice: [u8; 24] = self.try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl TryInto<Address> for &Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        let slice: [u8; 24] = self.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
        Ok(Address(buf))
    }
}

impl TryInto<Address> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Address, anyhow::Error> {
        let slice: [u8; 24] = self.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(&slice);
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
