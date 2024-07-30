// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{Block, BlockError, BlockRuntime, InputPort, Message, OutputPort, Port};

/// A block that counts the number of messages it receives, while optionally
/// passing them through.
#[derive(Block, Clone)]
pub struct Count<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The (optional) output target for the stream being passed through.
    #[output]
    pub output: OutputPort<T>,

    /// The output port for the message count.
    #[output]
    pub count: OutputPort<u64>,

    /// The internal state counting the number of messages received.
    #[state]
    counter: u64,
}

impl<T: Message> Block for Count<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> Result<(), BlockError> {
        while let Some(message) = self.input.recv()? {
            self.counter += 1;

            if self.output.is_connected() {
                self.output.send(&message)?;
            } else {
                drop(message);
            }
        }

        runtime.wait_for(&self.count)?;

        self.count.send(&self.counter)?;

        Ok(())
    }
}
