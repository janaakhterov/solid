use std::convert::TryInto;

pub struct Function(pub Vec<u8>);

impl TryInto<Function> for &str {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Function, anyhow::Error> {
        match self.len() {
            48 => Ok(()),
            50 => Ok(()),
            length => Err(anyhow!(
                "Function string length expected to be 48 or 50, received: {}",
                length
            )),
        }?;

        Ok(Function(hex::decode(&self)?))
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
