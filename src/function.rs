use std::convert::TryInto;
use crate::encode::Encode;

pub struct Function(pub [u8; 32]);

impl TryInto<Function> for &str {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Function, anyhow::Error> {
        // Remove the `0x` prefix if the length suggests that.
        let s = match self.len() {
            48 => Ok(self),
            50 => Ok(self.split_at(2).1),
            length => Err(anyhow!(
                "Function string length expected to be 48 or 50, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&hex::decode(&s)?);
        Ok(Function(buf))
    }
}

impl TryInto<Function> for &[u8] {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Function, anyhow::Error> {
        match self.len() {
            24 => Ok(()),
            length => Err(anyhow!(
                "Function slice length expected to be 24, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&self);
        Ok(Function(buf))
    }
}

impl TryInto<Function> for &Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Function, anyhow::Error> {
        match self.len() {
            24 => Ok(()),
            length => Err(anyhow!(
                "Function vec length expected to be 24, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&self);
        Ok(Function(buf))
    }
}

impl TryInto<Function> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Function, anyhow::Error> {
        match self.len() {
            24 => Ok(()),
            length => Err(anyhow!(
                "Function vec length expected to be 24, received: {}",
                length
            )),
        }?;

        let mut buf = [0u8; 32];
        buf[8..].copy_from_slice(&self);
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
