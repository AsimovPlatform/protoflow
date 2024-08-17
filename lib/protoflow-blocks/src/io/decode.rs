// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{
    prelude::{Bytes, FromStr, String},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;

/// A block that decodes `T` messages from a byte stream.
#[derive(Block, Clone)]
pub struct Decode<T: Message + FromStr = String> {
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

impl<T: Message + FromStr> Decode<T> {
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

impl<T: Message + FromStr> Block for Decode<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        unimplemented!() // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::Decode;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Decode::<i32>::new(s.input(), s.output()));
        });
    }
}
