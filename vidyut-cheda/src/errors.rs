use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use vidyut_kosha::Error as KoshaError;
use vidyut_sandhi::Error as SandhiError;

/// A type alias for `Result<T, vidyut_cheda::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Models all of the errors this crate might produce.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An IO error.
    Io(io::Error),
    /// A CSV parsing error.
    Csv(csv::Error),
    /// A kosha read error.
    Kosha(KoshaError),
    /// A sandhi read error.
    Sandhi(SandhiError),
    /// Text is not encoded as ASCII.
    NotAscii,
    /// A DCS parse error.
    DcsParse {
        /// The field that couldn't be parsed.
        field: String,
        /// The value that couldn't be parsed.
        value: String,
    },
    /// A DCS parse error with a missing field.
    DcsUndefined(String),
    /// An integer parsing error.
    ParseInt(ParseIntError),
    /// A float parsing error.
    ParseFloat(ParseFloatError),
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

impl From<vidyut_sandhi::Error> for Error {
    fn from(err: vidyut_sandhi::Error) -> Error {
        use vidyut_sandhi::Error::*;
        match err {
            Io(e) => Error::Io(e),
            Csv(e) => Error::Csv(e),
            e => Error::Sandhi(e),
        }
    }
}

impl From<ParseIntError> for Error {
    #[inline]
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<ParseFloatError> for Error {
    #[inline]
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<KoshaError> for Error {
    #[inline]
    fn from(err: KoshaError) -> Error {
        Error::Kosha(err)
    }
}

impl Error {
    pub(crate) fn parse_dcs(field: &str, value: &str) -> Self {
        Error::DcsParse {
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    pub(crate) fn dcs_undefined(field: &str) -> Self {
        Error::DcsUndefined(field.to_string())
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(e) => e.fmt(f),
            Csv(e) => e.fmt(f),
            Kosha(e) => e.fmt(f),
            Sandhi(e) => e.fmt(f),
            DcsParse { field, value } => write!(f, "Could not parse {field} value `{value}`."),
            DcsUndefined(field) => write!(f, "Field {field} is missing."),
            ParseInt(_) => write!(f, "Int parsing error"),
            ParseFloat(_) => write!(f, "Float parsing error"),
            NotAscii => write!(f, "Input text must be an ASCII (SLP1) string."),
        }
    }
}
