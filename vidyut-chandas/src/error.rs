use std::fmt;

#[allow(unused)]
pub(crate) type Result<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    ParseError,
    EnumParseError(String),
    IoError(std::io::Error),
}

impl Error {
    pub(crate) fn enum_parse_error(value: &str) -> Self {
        Error::EnumParseError(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            ParseError => write!(f, "Could not parse meter."),
            EnumParseError(e) => write!(f, "Could not parse enum value {e}."),
            IoError(_) => write!(f, "Could not open input file."),
        }
    }
}
