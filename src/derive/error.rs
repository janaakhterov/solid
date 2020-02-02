use serde::{de, ser};
use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    Message(String),
    Eof,
    TrailingCharacters,
    TryIntoSliceError(#[from] std::array::TryFromSliceError),
    Utf8Error(#[from] std::str::Utf8Error),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.to_string())
    }
}
