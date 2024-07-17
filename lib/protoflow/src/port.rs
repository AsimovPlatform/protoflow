// This is free and unencumbered software released into the public domain.

use crate::PortState;

pub trait Port {
    fn state(&self) -> PortState;
    fn name(&self) -> &str;
    fn label(&self) -> Option<&str>;

    fn is_closed(&self) -> bool {
        self.state() == PortState::Closed
    }

    fn is_open(&self) -> bool {
        self.state() == PortState::Open
    }

    fn is_connected(&self) -> bool {
        self.state() == PortState::Connected
    }
}
