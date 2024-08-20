// This is free and unencumbered software released into the public domain.

use crate::prelude::Bytes;

#[derive(Clone, Debug)]
pub enum MpscTransportEvent {
    #[allow(unused)]
    Connect,
    Message(Bytes),
    Disconnect,
}
