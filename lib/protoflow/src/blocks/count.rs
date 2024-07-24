// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Vec},
    Block, BlockDescriptor, BlockError, InputPort, Message, OutputPort, Port, PortDescriptor,
    Scheduler,
};

/// A block that counts the number of messages it receives, while optionally
/// passing them through.
pub struct Count<T: Message> {
    /// The input message stream.
    input: InputPort<T>,
    /// The (optional) output target for the stream being passed through.
    output: OutputPort<T>,
    /// The output port for the message count.
    count: OutputPort<u64>,
    /// The internal state counting the number of messages received.
    counter: u64,
}

impl<T: Message> BlockDescriptor for Count<T> {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.input)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![
            PortDescriptor::from(&self.output),
            PortDescriptor::from(&self.count),
        ]
    }
}

impl<T: Message> Block for Count<T> {
    fn execute(&mut self, scheduler: &dyn Scheduler) -> Result<(), BlockError> {
        while let Some(message) = self.input.receive()? {
            self.counter += 1;

            if self.output.is_connected() {
                self.output.send(&message)?;
            } else {
                drop(message);
            }
        }

        scheduler.wait_for(&self.count)?;

        self.count.send(&self.counter)?;

        Ok(())
    }
}
