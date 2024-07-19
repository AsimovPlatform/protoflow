// This is free and unencumbered software released into the public domain.

use crate::{Block, InputPort, Message, PortDescriptor};

/// A block that discards all messages it receives.
pub struct Drop<T: Message>(InputPort<T>);

impl<T: Message> Block for Drop<T> {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.0)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![] // no output ports
    }

    fn execute(&mut self) {
        loop {
            drop(self.0.receive());
        }
    }
}
