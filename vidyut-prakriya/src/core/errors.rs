use crate::core::RuleChoice;
use std::fmt;
use std::io;
use std::num;

pub type Result<T> = std::result::Result<T, Error>;

/// Models all of the errors this crate might produce.
#[derive(Debug)]
pub enum Error {
    /// An IO error.
    Io(io::Error),

    /// An input file is invalid in some way.
    InvalidFile,

    /// An integer couldn't be parsed.
    ParseInt(num::ParseIntError),

    /// The program received an it letter that it can't recognize.
    UnknownIt(char),

    /// An argument is missing a required field.
    MissingRequiredField(&'static str),

    /// An enum value could not parsed correctly.
    ParseError(String),

    /// A term has an empty upadesha.
    InvalidUpadesha(String),

    /// The caller's arguments are incompatible with the prakriya, so we aborted early.
    Abort(Vec<RuleChoice>),
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    #[inline]
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl Error {
    pub(crate) fn enum_parse_error(value: &str) -> Self {
        Error::ParseError(value.to_string())
    }

    pub(crate) fn missing_required_field(field: &'static str) -> Self {
        Error::MissingRequiredField(field)
    }

    pub(crate) fn invalid_upadesha(value: &str) -> Self {
        Error::InvalidUpadesha(value.to_string())
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(_) => write!(f, "Cannot read input file."),
            InvalidFile => write!(f, "The input file is invalid."),
            ParseInt(_) => write!(f, "Parse int error"),
            UnknownIt(c) => write!(f, "`{c}` could not be parsed as an it-samjna."),
            InvalidUpadesha(s) => write!(f, "The term `{s}` unexpectedly has an empty upadesha."),
            MissingRequiredField(s) => write!(f, "Please define the `{s}` field."),
            ParseError(v) => write!(f, "Could not parse `{v}` into an enum value."),
            Abort(_) => write!(f, "The given arguments cannot produce a valid prakriya."),
        }
    }
}
