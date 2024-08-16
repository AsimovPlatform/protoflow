// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{
    prelude::Bytes, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;

/// A block that parses `T` messages from a byte stream.
#[derive(Block, Clone)]
pub struct Read<T: Message> {
    /// The input byte stream.
    #[input]
    pub input: InputPort<Bytes>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Message> Read<T> {
    pub fn new(input: InputPort<Bytes>, output: OutputPort<T>) -> Self {
        Self { input, output }
    }
}

impl<T: Message> Block for Read<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        unimplemented!() // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::Read;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Read::<i32>::new(s.input(), s.output()));
        });
    }
}
