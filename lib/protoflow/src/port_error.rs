// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, String, ToString},
    PortID,
};

#[cfg(feature = "std")]
extern crate std;

pub type PortResult<T> = Result<T, PortError>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortError {
    Invalid(PortID),
    Closed,
    Disconnected,
    RecvFailed,
    SendFailed,
    Other(String),
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Invalid(port) => write!(f, "Port #{} is invalid", port),
            Self::Closed => write!(f, "Port is closed"),
            Self::Disconnected => write!(f, "Port is not connected"),
            Self::RecvFailed => write!(f, "Port receive failed"),
            Self::SendFailed => write!(f, "Port send failed"),
            Self::Other(message) => write!(f, "{}", message),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PortError {}

#[cfg(feature = "std")]
impl From<std::io::Error> for PortError {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.to_string())
    }
}
