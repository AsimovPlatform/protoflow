// This is free and unencumbered software released into the public domain.

use crate::{Block, InputPort, Message, OutputPort, PortDescriptor};
use std::time::Duration;

pub struct Delay<T: Message> {
    input: InputPort<T>,
    output: OutputPort<T>,
    delay: Duration,
}

impl<T: Message> Block for Delay<T> {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.input)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.output)]
    }

    fn execute(&mut self) {
        loop {
            let Some(message) = self.input.receive() else {
                break;
            };
            std::thread::sleep(self.delay);
            self.output.send(message);
        }
    }
}
