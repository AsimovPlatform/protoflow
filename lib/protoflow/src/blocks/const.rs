// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{Block, BlockError, BlockRuntime, Message, OutputPort};

/// A block for sending a constant value.
#[derive(Block, Clone)]
pub struct Const<T: Message> {
    /// The port to send the value on.
    #[output]
    pub output: OutputPort<T>,

    /// A parameter for the value to send.
    #[parameter]
    pub value: T,
}

impl<T: Message + Clone + 'static> Block for Const<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> Result<(), BlockError> {
        runtime.wait_for(&self.output)?;

        self.output.send(&self.value)?;
        self.output.close()?;

        Ok(())
    }
}
