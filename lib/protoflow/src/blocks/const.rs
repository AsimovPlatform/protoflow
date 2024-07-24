// This is free and unencumbered software released into the public domain.

use crate as protoflow;

use protoflow::derive::Block;
use protoflow::{Block, BlockError, Message, OutputPort, Scheduler};

/// A block for sending a constant value.
#[derive(Block)]
pub struct Const<T: Message> {
    /// The port to send the value on.
    #[output]
    output: OutputPort<T>,
    /// A parameter for the value to send.
    #[parameter]
    value: T,
}

impl<T: Message> Block for Const<T> {
    fn execute(&mut self, scheduler: &dyn Scheduler) -> Result<(), BlockError> {
        scheduler.wait_for(&self.output)?;

        self.output.send(&self.value)?;

        Ok(())
    }
}
