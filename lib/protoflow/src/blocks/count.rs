// This is free and unencumbered software released into the public domain.

use crate::{Block, InputPort, Message, OutputPort, Port, PortDescriptor, Scheduler};

/// A block that counts the number of messages it receives, while optionally
/// passing them through.
pub struct Count<T: Message, C: Message> {
    /// The input message stream.
    input: InputPort<T>,
    /// The (optional) output target for the stream being passed through.
    output: OutputPort<T>,
    /// The output port for the message count.
    count: OutputPort<C>,
    /// The internal state counting the number of messages received.
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

    fn execute(&mut self, scheduler: &dyn Scheduler) {
        while let Some(message) = self.input.receive() {
            self.counter += 1;

            if self.output.is_connected() {
                self.output.send(&message);
            } else {
                drop(message);
            }
        }

        if self.count.is_connected() {
            //self.count.send(C::from(self.counter)); // FIXME
        }
    }
}
