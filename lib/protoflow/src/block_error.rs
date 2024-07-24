// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, String, ToString},
    PortError,
};

#[cfg(feature = "std")]
extern crate std;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BlockError {
    Terminated,
    PortError(PortError),
    Other(String),
}

impl fmt::Debug for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Terminated => write!(f, "BlockError::Terminated"),
            Self::PortError(e) => write!(f, "BlockError::PortError({})", e),
            Self::Other(message) => write!(f, "BlockError::Other(\"{}\")", message),
        }
    }
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Terminated => write!(f, "Execution terminated"),
            Self::PortError(e) => write!(f, "{}", e),
            Self::Other(message) => write!(f, "{}", message),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlockError {}

#[cfg(feature = "std")]
impl From<std::io::Error> for BlockError {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.to_string())
    }
}
