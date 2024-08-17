// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{
    prelude::{Bytes, String},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;

/// A block that decodes `T` messages from a byte stream.
#[derive(Block, Clone)]
pub struct Read<T: Message = String> {
    /// The input byte stream.
    #[input]
    pub input: InputPort<Bytes>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,

    /// A configuration parameter for how to decode messages.
    #[parameter]
    pub encoding: ReadEncoding,
}

/// The encoding to use when deserializing messages from bytes.
#[derive(Clone, Debug, Default)]
pub enum ReadEncoding {
    #[default]
    ProtobufWithLengthPrefix,
    ProtobufWithoutLengthPrefix,
    TextWithNewlineSuffix,
}

impl<T: Message> Read<T> {
    pub fn new(input: InputPort<Bytes>, output: OutputPort<T>) -> Self {
        Self::with_params(input, output, ReadEncoding::default())
    }

    pub fn with_params(
        input: InputPort<Bytes>,
        output: OutputPort<T>,
        encoding: ReadEncoding,
    ) -> Self {
        Self {
            input,
            output,
            encoding,
        }
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
