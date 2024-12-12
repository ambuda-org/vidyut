use std::fmt;

/// A wrapper for `std::fmt::Result`.
#[allow(unused)]
pub type Result<T> = std::result::Result<T, LipiError>;

/// Models the error states of `vidyut-lipi`.
#[allow(unused)]
#[derive(Copy, Clone, Debug)]
pub enum LipiError {
    /// Could not parse an input value.
    ParseError,
}

impl std::error::Error for LipiError {}

impl fmt::Display for LipiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LipiError::*;

        match self {
            ParseError => write!(f, "parse error"),
        }
    }
}
