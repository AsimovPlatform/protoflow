// This is free and unencumbered software released into the public domain.

use crate::{Block, InputPort, Message, OutputPort, PortDescriptor};

/// A block that counts the number of messages it receives.
pub struct Count<T: Message, C: Message> {
    input: InputPort<T>,
    output: OutputPort<T>,
    count: OutputPort<C>,
    counter: u64,
}

impl<T: Message, C: Message> Block for Count<T, C> {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.input)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![
            PortDescriptor::from(&self.output),
            PortDescriptor::from(&self.count),
        ]
    }

    fn execute(&mut self) {
        loop {
            let Some(message) = self.input.receive() else {
                break;
            };
            self.output.send(message);
            self.counter += 1;
        }
        //self.count.send(C::from(self.counter)); // FIXME
    }
}
