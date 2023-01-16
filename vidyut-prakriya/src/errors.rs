use crate::prakriya::Prakriya;
use std::fmt;
use std::io;
use std::num;

pub type Result<T> = std::result::Result<T, Error>;

/// Models all of the errors this crate might produce.
#[derive(Debug)]
pub enum Error {
    /// An IO error
    Io(io::Error),
    /// A CSV format error.
    Csv(csv::Error),
    /// An input vile is invalid in some way.
    InvalidFile,

    /// An integer couldn't be parsed.
    ParseInt(num::ParseIntError),

    /// The program received an it letter that it can't recognize.
    UnknownIt(char),

    /// An argument is missing a required field.
    MissingRequiredField(&'static str),

    /// An enum value could not parsed correctly.
    ParseError(String),

    /// An enum value could not parsed correctly.
    GanaParseError(u8),

    /// A term has an empty upadesha.
    EmptyUpadesha(String),

    /// A generic error.
    Generic(&'static str),

    /// The caller's arguments are incompatible with the prakriya, so we aborted early.
    Abort(Prakriya),
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

    pub(crate) fn gana_parse_error(value: u8) -> Self {
        Error::GanaParseError(value)
    }

    pub(crate) fn missing_required_field(field: &'static str) -> Self {
        Error::MissingRequiredField(field)
    }

    pub(crate) fn invalid_upadesha(value: &str) -> Self {
        Error::EmptyUpadesha(value.to_string())
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(_) => write!(f, "I/O error"),
            Csv(_) => write!(f, "CSV error"),
            InvalidFile => write!(f, "The input file is invalid."),
            ParseInt(_) => write!(f, "Parse int error"),
            UnknownIt(c) => write!(f, "`{c}` could not be parsed as an it-samjan."),
            EmptyUpadesha(s) => write!(f, "The term `{s}` unexpectedly has an empty upadesha."),
            MissingRequiredField(s) => write!(f, "Please define the `{s}` field."),
            ParseError(v) => write!(f, "Could not parse `{v}` into an enum value."),
            GanaParseError(v) => write!(f, "Could not parse `{v}` as a dhatu gana."),
            Generic(msg) => write!(f, "{msg}"),
            Abort(_) => write!(f, "The given arguments cannot produce a valid prakriya."),
        }
    }
}
