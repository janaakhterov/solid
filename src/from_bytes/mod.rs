use crate::{
    solidity::IntoType,
    Error,
    Result,
};

// pub mod bytes;
// pub mod number;
// pub mod string;

pub trait FromBytes<'a, T> {
    fn from_bytes(value: &'a [u8], index: usize) -> Result<T>;
}

// Checks to see if a block (32 bytes) exists in the buffer.
pub(crate) fn valid_block(buf: &[u8], start: usize, end: usize) -> Result<()> {
    if let (Some(_), Some(_)) = (buf.get(start), buf.get(end - 1)) {
        Ok(())
    } else {
        Err(Error::Eof)
    }
}

impl<'a, T> FromBytes<'a, T> for T
where
    T: IntoType<'a>,
{
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<T> {
        let ty = <T as IntoType>::into_type();
        Err(Error::Eof)
    }
}
