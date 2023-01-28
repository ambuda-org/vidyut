use std::fmt;
use std::io;

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Models all of the errors this crate might produce.
#[derive(Debug)]
pub enum Error {
    /// An IO error
    Io(io::Error),
    /// A CSV format error.
    Csv(csv::Error),
    /// The CSV file doesn't contain any rules.
    EmptyFile,
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<csv::Error> for Error {
    #[inline]
    fn from(err: csv::Error) -> Error {
        Error::Csv(err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(e) => e.fmt(f),
            Csv(e) => e.fmt(f),
            EmptyFile => write!(f, "Sandhi file is empty."),
        }
    }
}
