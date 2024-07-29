// This is free and unencumbered software released into the public domain.

use crate::{prelude::Box, InputPortID, Message, OutputPortID, PortID, PortResult, PortState};

#[allow(unused)]
pub trait Transport: Send + Sync {
    fn state(&self, port: PortID) -> PortResult<PortState>;
    fn close(&self, port: PortID) -> PortResult<bool>;
    fn open_input(&self) -> PortResult<InputPortID>;
    fn open_output(&self) -> PortResult<OutputPortID>;
    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool>;
    fn send(&self, output: OutputPortID, message: Box<dyn Message>) -> PortResult<()>;
    fn recv(&self, input: InputPortID) -> PortResult<Box<dyn Message>>;
}
