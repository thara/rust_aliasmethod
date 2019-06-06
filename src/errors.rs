use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ZeroTotalWeights,
    Internal { text: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ZeroTotalWeights => write!(f, "Total of weights is 0."),
            Error::Internal { ref text } => write!(f, "Internal error: {}", text),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ZeroTotalWeights => "Total of weights is 0.",
            Error::Internal { ref text } => text,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ZeroTotalWeights => None,
            Error::Internal { text: _ } => None,
        }
    }
}
