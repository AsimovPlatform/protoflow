// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort, Port};

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

impl<T: Message + 'static> Count<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>, count: OutputPort<u64>) -> Self {
        Self {
            input,
            output,
            count,
            counter: 0,
        }
    }
}

impl<T: Message + 'static> Block for Count<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
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

#[cfg(test)]
mod tests {
    use super::Count;
    use crate::{transports::MockTransport, System};

    #[test]
    fn instantiate_count_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Count::<i32>::new(s.input(), s.output(), s.output()));
        });
    }
}
