use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GenericError {
    pub details: String,
}

impl GenericError {
    pub(crate) fn new(msg: &str) -> GenericError {
        GenericError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::io::Error> for GenericError {
    fn from(x: std::io::Error) -> Self {
        GenericError {
            details: x.to_string(),
        }
    }
}

impl From<std::ffi::NulError> for GenericError {
    fn from(x: std::ffi::NulError) -> Self {
        GenericError {
            details: x.to_string(),
        }
    }
}
