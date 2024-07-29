// This is free and unencumbered software released into the public domain.

use crate::OutputPortID;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortState {
    #[default]
    Closed,
    Open,
    Connected(OutputPortID),
}

impl PortState {
    /// Checks whether the port state is currently closed.
    pub fn is_closed(&self) -> bool {
        *self == PortState::Closed
    }

    /// Checks whether the port state is currently open.
    pub fn is_open(&self) -> bool {
        *self == PortState::Open
    }

    /// Checks whether the port state is currently connected.
    pub fn is_connected(&self) -> bool {
        matches!(self, PortState::Connected(_))
    }

    pub fn to_str(&self) -> &str {
        use PortState::*;
        match self {
            Closed => "closed",
            Open => "open",
            Connected(_) => "connected",
        }
    }
}

impl AsRef<str> for PortState {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}
