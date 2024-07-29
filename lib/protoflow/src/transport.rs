// This is free and unencumbered software released into the public domain.

use crate::{prelude::Box, InputPortID, Message, OutputPortID, PortID, PortResult, PortState};

#[allow(unused)]
pub trait Transport: AsTransport + Send + Sync {
    fn state(&self, port: PortID) -> PortResult<PortState>;
    fn close(&self, port: PortID) -> PortResult<bool>;
    fn open_input(&self) -> PortResult<InputPortID>;
    fn open_output(&self) -> PortResult<OutputPortID>;
    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool>;
    fn send(&self, output: OutputPortID, message: Box<dyn Message>) -> PortResult<()>;
    fn recv(&self, input: InputPortID) -> PortResult<Box<dyn Message>>;
}

pub trait AsTransport {
    fn as_transport(&self) -> &dyn Transport;
}

impl<T: Transport + Sized> AsTransport for T {
    fn as_transport(&self) -> &dyn Transport {
        self
    }
}
