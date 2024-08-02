// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, Box, Result, String, ToString},
    PortError,
};

#[cfg(feature = "std")]
extern crate std;

pub type BlockResult<T = ()> = Result<T, BlockError>;

#[derive(Debug)]
pub enum BlockError {
    Terminated,
    PortError(PortError),
    Other(String),
    #[cfg(feature = "std")]
    Panic(Box<dyn std::any::Any + Send>),
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Terminated => write!(f, "Execution terminated"),
            Self::PortError(e) => write!(f, "{}", e),
            Self::Other(message) => write!(f, "{}", message),
            #[cfg(feature = "std")]
            Self::Panic(e) => write!(f, "Panic: {:?}", e),
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

#[cfg(feature = "std")]
impl From<Box<dyn std::any::Any + Send>> for BlockError {
    fn from(error: Box<dyn std::any::Any + Send>) -> Self {
        Self::Panic(error)
    }
}

impl From<PortError> for BlockError {
    fn from(error: PortError) -> Self {
        Self::PortError(error)
    }
}
