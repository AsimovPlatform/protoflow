// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, String, ToString};

#[cfg(feature = "std")]
extern crate std;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortError {
    NotOpen,
    NotConnected,
    RecvFailed,
    SendFailed,
    Other(String),
}

impl fmt::Debug for PortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotOpen => write!(f, "PortError::NotOpen"),
            Self::NotConnected => write!(f, "Error::NotConnected"),
            Self::RecvFailed => write!(f, "Error::RecvFailed"),
            Self::SendFailed => write!(f, "Error::SendFailed"),
            Self::Other(message) => write!(f, "Error::Other(\"{}\")", message),
        }
    }
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotOpen => write!(f, "Port is not open"),
            Self::NotConnected => write!(f, "Port is not connected"),
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
