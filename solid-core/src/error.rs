#[cfg(feature = "derive")]
use serde::{
    de,
    ser,
};
use std::{
    fmt,
    string::FromUtf8Error,
};

/// Simple wrapper around `std::result::Result`
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Crate level error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    Message(String),
    Eof,
    TrailingCharacters,
    TryIntoSliceError(#[from] std::array::TryFromSliceError),
    Utf8Error(#[from] std::str::Utf8Error),
    FromUtf8Error(#[from] FromUtf8Error),
    FromHexError(#[from] hex::FromHexError),
}

#[cfg(feature = "derive")]
impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

#[cfg(feature = "derive")]
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
