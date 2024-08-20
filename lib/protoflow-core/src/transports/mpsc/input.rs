// This is free and unencumbered software released into the public domain.

extern crate std;

use super::MpscTransportEvent;
use crate::PortState;
use parking_lot::Mutex;
use std::sync::mpsc::Receiver;

#[derive(Debug, Default)]
pub enum MpscTransportInputPortState {
    #[default]
    Open,
    Connected(Mutex<Receiver<MpscTransportEvent>>),
    Closed,
}

impl MpscTransportInputPortState {
    pub fn state(&self) -> PortState {
        match self {
            Self::Open => PortState::Open,
            Self::Connected(_) => PortState::Connected,
            Self::Closed => PortState::Closed,
        }
    }
}
