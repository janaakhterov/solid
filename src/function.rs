use std::convert::TryInto;
use crate::encode::Encode;

pub struct Function(pub [u8; 32]);

impl TryInto<Function> for &str {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        // Remove the `0x` prefix if the length suggests that.
        let s = match self.len() {
            48 => self,
            50 => self.split_at(2).1,
            _ => self
        };

        let slice = hex::decode(&s)?;
        let slice: [u8; 24] = slice.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl TryInto<Function> for &[u8] {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        let slice: [u8; 24] = self.try_into()?;
        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl TryInto<Function> for &Vec<u8> {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        let slice: [u8; 24] = self.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl TryInto<Function> for Vec<u8> {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        let slice: [u8; 24] = self.as_slice().try_into()?;
        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&slice);
        Ok(Function(buf))
    }
}

impl Encode for Function {
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
