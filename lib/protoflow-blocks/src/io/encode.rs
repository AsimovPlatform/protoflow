// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{Encoding, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{Bytes, String, ToString},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;

/// A block that encodes `T` messages to a byte stream.
#[derive(Block, Clone)]
pub struct Encode<T: Message + ToString = String> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output byte stream.
    #[output]
    pub output: OutputPort<Bytes>,

    /// A configuration parameter for how to encode messages.
    #[parameter]
    pub encoding: Encoding,
}

impl<T: Message + ToString> Encode<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<Bytes>) -> Self {
        Self::with_params(input, output, Encoding::default())
    }

    pub fn with_params(input: InputPort<T>, output: OutputPort<Bytes>, encoding: Encoding) -> Self {
        Self {
            input,
            output,
            encoding,
        }
    }
}

impl<T: Message + ToString> Block for Encode<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        while let Some(message) = self.input.recv()? {
            use Encoding::*;
            let bytes = match self.encoding {
                ProtobufWithLengthPrefix => Bytes::from(message.encode_length_delimited_to_vec()),
                ProtobufWithoutLengthPrefix => Bytes::from(message.encode_to_vec()),
                TextWithNewlineSuffix => {
                    let mut string = message.to_string();
                    string.push('\n');
                    Bytes::from(string)
                }
            };
            self.output.send(&bytes)?;
        }

        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Encode {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        //use crate::{CoreBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|_s| todo!()))
    }
}

#[cfg(test)]
mod tests {
    use super::Encode;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Encode::<i32>::new(s.input(), s.output()));
        });
    }
}
