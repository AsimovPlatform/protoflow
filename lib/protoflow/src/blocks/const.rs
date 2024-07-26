// This is free and unencumbered software released into the public domain.

use crate as protoflow;

use protoflow::derive::Block;
use protoflow::{Block, BlockError, BlockRuntime, Message, OutputPort};

/// A block for sending a constant value.
#[derive(Block)]
pub struct Const<T: Message> {
    /// The port to send the value on.
    #[output]
    pub output: OutputPort<T>,

    /// A parameter for the value to send.
    #[parameter]
    pub value: T,
}

impl<T: Message> Block for Const<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> Result<(), BlockError> {
        runtime.wait_for(&self.output)?;

        self.output.send(&self.value)?;

        Ok(())
    }
}
