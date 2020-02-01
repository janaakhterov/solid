use std::convert::TryInto;

pub struct Function(pub Vec<u8>);

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

        Ok(Function(hex::decode(&s)?))
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

        Ok(Function(self.to_vec()))
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

        Ok(Function(self.clone()))
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

        Ok(Function(self))
    }
}
