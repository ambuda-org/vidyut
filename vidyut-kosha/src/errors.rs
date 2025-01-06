use rmp_serde::decode::Error as DecodeError;
use rmp_serde::encode::Error as EncodeError;
use std::fmt;
use std::io;
use std::num;

pub type Result<T> = std::result::Result<T, Error>;

/// Models all of the errors this crate might produce.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An IO error.
    Io(io::Error),
    /// A decoding IO error.
    DecodeError(DecodeError),
    /// An encoding IO error.
    EncodeError(EncodeError),
    /// An FST error.
    Fst(fst::raw::Error),
    /// An integer couldn't be parsed.
    TryFromInt(num::TryFromIntError),
    /// Tried to insert too many duplicates into the kosha.
    TooManyDuplicates(String),
    /// The given int could not be mapped to a registry item.
    UnknownId(&'static str, usize),
    /// The given data type was not found in the registry.
    NotRegistered(&'static str),
    /// Value could not be parsed into the given enum.
    ParseEnum(&'static str, String),
    /// Data type is not yet supported in the kosha.
    UnsupportedType,
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<fst::Error> for Error {
    #[inline]
    fn from(err: fst::Error) -> Error {
        match err {
            fst::Error::Io(e) => Error::Io(e),
            fst::Error::Fst(e) => Error::Fst(e),
        }
    }
}

impl From<num::TryFromIntError> for Error {
    #[inline]
    fn from(err: num::TryFromIntError) -> Error {
        Error::TryFromInt(err)
    }
}

impl From<DecodeError> for Error {
    #[inline]
    fn from(err: DecodeError) -> Error {
        Error::DecodeError(err)
    }
}

impl From<EncodeError> for Error {
    #[inline]
    fn from(err: EncodeError) -> Error {
        Error::EncodeError(err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(e) => e.fmt(f),
            DecodeError(e) => e.fmt(f),
            EncodeError(e) => e.fmt(f),
            Fst(e) => e.fmt(f),
            TooManyDuplicates(s) => write!(f, "Key `{}` has been inserted too many times.", s),
            UnknownId(name, id) => write!(f, "Unknown {name} ID: {}", id),
            NotRegistered(name) => write!(f, "Record of type {name} was not in the registry."),
            ParseEnum(name, value) => write!(f, "Enum `{name}` has no value `{value}`."),
            TryFromInt(e) => e.fmt(f),
            UnsupportedType => write!(f, "Data type not yet supported."),
        }
    }
}
