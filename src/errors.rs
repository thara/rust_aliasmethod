use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AliasMethodError {
    ZeroTotalWeights,
    Internal { text: String },
}

impl fmt::Display for AliasMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AliasMethodError::ZeroTotalWeights => write!(f, "Total of weights is 0."),
            AliasMethodError::Internal { ref text } => write!(f, "Internal error: {}", text),
        }
    }
}

impl error::Error for AliasMethodError {
    fn description(&self) -> &str {
        match *self {
            AliasMethodError::ZeroTotalWeights => "Total of weights is 0.",
            AliasMethodError::Internal { ref text } => text,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AliasMethodError::ZeroTotalWeights => None,
            AliasMethodError::Internal { text: _ } => None,
        }
    }
}
