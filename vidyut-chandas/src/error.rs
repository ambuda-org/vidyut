use std::fmt;

#[allow(unused)]
pub(crate) type Result<T> = std::result::Result<T, ChandasError>;

#[allow(unused)]
#[derive(Debug)]
pub enum ChandasError {
    ParseError,
    IoError(std::io::Error),
}

impl From<std::io::Error> for ChandasError {
    #[inline]
    fn from(err: std::io::Error) -> ChandasError {
        ChandasError::IoError(err)
    }
}

impl fmt::Display for ChandasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ChandasError::*;

        match self {
            ParseError => write!(f, "Could not parse meter."),
            IoError(_) => write!(f, "Could not open input file."),
        }
    }
}
