// This is free and unencumbered software released into the public domain.

use protoflow_core::{prelude::VecDeque, Block, BlockResult, BlockRuntime, InputPort, Message};
use protoflow_derive::Block;

/// A block that simply stores all messages it receives.
#[derive(Block, Clone)]
pub struct Buffer<T: Message + Into<T>> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The internal state storing the messages received.
    #[state]
    messages: VecDeque<T>,
}

impl<T: Message> Buffer<T> {
    pub fn new(input: InputPort<T>) -> Self {
        Self {
            input,
            messages: VecDeque::new(),
        }
    }

    pub fn messages(&self) -> &VecDeque<T> {
        &self.messages
    }
}

impl<T: Message> Block for Buffer<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.messages.push_back(message);
        }
        self.input.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Buffer::<i32>::new(s.input()));
        });
    }
}
