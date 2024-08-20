// This is free and unencumbered software released into the public domain.

extern crate std;

use super::MpscTransportEvent;
use crate::PortState;
use std::sync::mpsc::SyncSender;

#[derive(Clone, Debug, Default)]
pub enum MpscTransportOutputPortState {
    #[default]
    Open,
    Connected(SyncSender<MpscTransportEvent>),
    Closed,
}

impl MpscTransportOutputPortState {
    pub fn state(&self) -> PortState {
        match self {
            Self::Open => PortState::Open,
            Self::Connected(_) => PortState::Connected,
            Self::Closed => PortState::Closed,
        }
    }
}
