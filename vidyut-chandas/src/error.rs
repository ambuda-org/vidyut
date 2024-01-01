use std::error::Error;
use std::fmt;

#[allow(unused)]
pub(crate) type Result<T> = std::result::Result<T, ChandasError>;

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum ChandasError {
    ParseError,
}

impl Error for ChandasError {}

impl fmt::Display for ChandasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse meter.")
    }
}
