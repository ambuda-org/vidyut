use std::fmt;
use std::io;
use std::num;

pub type Result<T> = std::result::Result<T, Error>;

/// Models all of the errors this crate might produce.
#[derive(Debug)]
pub enum Error {
    /// An IO error.
    Io(io::Error),
    /// An FST error.
    Fst(fst::raw::Error),
    /// The given int could not be mapped to a dhatu.
    UnknownDhatuId(u32),
    /// The given int could not be mapped to a pratipadika.
    UnknownPratipadikaId(u32),
    /// Value could not be parsed into the given enum.
    EnumParse(&'static str, String),
    /// An integer couldn't be parsed.
    TryFromInt(num::TryFromIntError),
    /// A eneric error.
    Generic(String),
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

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(_) => write!(f, "I/O error"),
            UnknownDhatuId(id) => write!(f, "Unknown dhatu ID {}", id),
            UnknownPratipadikaId(id) => write!(f, "Unknown pratipadika id {}", id),
            _ => write!(f, "foo"),
        }
    }
}
