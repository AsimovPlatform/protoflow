// This is free and unencumbered software released into the public domain.

use crate::{prelude::Box, InputPortID, Message, OutputPortID, PortID, PortResult, PortState};

#[allow(unused)]
pub trait Transport: AsTransport + Send + Sync {
    fn state(&self, port: PortID) -> PortResult<PortState>;
    fn open_input(&self) -> PortResult<InputPortID>;
    fn open_output(&self) -> PortResult<OutputPortID>;

    fn close(&self, port: PortID) -> PortResult<bool> {
        Ok(match port {
            PortID::Input(input) => self.close_input(input)?,
            PortID::Output(output) => self.close_output(output)?,
        })
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool>;
    fn close_output(&self, output: OutputPortID) -> PortResult<bool>;
    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool>;
    fn send(&self, output: OutputPortID, message: Box<dyn Message>) -> PortResult<()>;
    fn recv(&self, input: InputPortID) -> PortResult<Option<Box<dyn Message>>>;
    fn try_recv(&self, input: InputPortID) -> PortResult<Option<Box<dyn Message>>>;
}

pub trait AsTransport {
    fn as_transport(&self) -> &dyn Transport;
}

impl<T: Transport + Sized> AsTransport for T {
    fn as_transport(&self) -> &dyn Transport {
        self
    }
}
