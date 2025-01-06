use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    DecodeError(rmp_serde::decode::Error),
    EncodeError(rmp_serde::encode::Error),
    Sandhi(vidyut_sandhi::Error),
    Kosha(vidyut_kosha::Error),
    NonAsciiText,
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::Io(val)
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(val: rmp_serde::decode::Error) -> Self {
        Error::DecodeError(val)
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(val: rmp_serde::encode::Error) -> Self {
        Error::EncodeError(val)
    }
}

impl From<vidyut_kosha::Error> for Error {
    fn from(val: vidyut_kosha::Error) -> Self {
        Error::Kosha(val)
    }
}

impl From<vidyut_sandhi::Error> for Error {
    fn from(val: vidyut_sandhi::Error) -> Self {
        Error::Sandhi(val)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "chedaka error, {:?}", self)
    }
}
