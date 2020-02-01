use std::convert::TryInto;

pub struct Address(pub Vec<u8>);

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

        Ok(Address(hex::decode(&s)?))
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

        Ok(Address(self.to_vec()))
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

        Ok(Address(self.clone()))
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

        Ok(Address(self))
    }
}
