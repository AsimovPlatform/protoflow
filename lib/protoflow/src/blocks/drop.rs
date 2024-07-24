// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Vec},
    Block, BlockDescriptor, BlockError, InputPort, Message, PortDescriptor, Scheduler,
};

/// A block that simply discards all messages it receives.
pub struct Drop<T: Message>(InputPort<T>);

impl<T: Message> BlockDescriptor for Drop<T> {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.0)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![] // no output ports
    }
}

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _scheduler: &dyn Scheduler) -> Result<(), BlockError> {
        while let Some(message) = self.0.receive()? {
            drop(message);
        }
        Ok(())
    }
}
