// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{
    prelude::Bytes, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;

/// A block that serializes `T` messages to a byte stream.
#[derive(Block, Clone)]
pub struct Write<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output byte stream.
    #[output]
    pub output: OutputPort<Bytes>,
}

impl<T: Message> Write<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<Bytes>) -> Self {
        Self { input, output }
    }
}

impl<T: Message> Block for Write<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        while let Some(message) = self.input.recv()? {
            let bytes = Bytes::from(message.encode_length_delimited_to_vec());
            self.output.send(&bytes)?;
        }

        self.input.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Write;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Write::<i32>::new(s.input(), s.output()));
        });
    }
}
