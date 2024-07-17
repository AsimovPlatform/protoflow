// This is free and unencumbered software released into the public domain.

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortState {
    #[default]
    Closed,
    Open,
    Connected,
}

impl AsRef<str> for PortState {
    fn as_ref(&self) -> &str {
        use PortState::*;
        match self {
            Closed => "closed",
            Open => "open",
            Connected => "connected",
        }
    }
}
