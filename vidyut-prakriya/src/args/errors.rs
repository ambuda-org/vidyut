use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Indicates a failure to parse a string representation of some `semantics` enum.
#[derive(Debug, Clone)]
pub struct ArgumentError {
    /// The error message.
    msg: String,
}

impl ArgumentError {
    pub(crate) fn new(s: &str) -> Self {
        ArgumentError { msg: s.to_owned() }
    }
    pub(crate) fn enum_parse_error(name: &str, value: &str) -> Self {
        ArgumentError {
            msg: format!("Could not parse value `{value}` into enum `{name}`."),
        }
    }
}

impl Error for ArgumentError {}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg)
    }
}
