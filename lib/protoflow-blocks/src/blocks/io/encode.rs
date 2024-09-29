// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{types::Encoding, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{Bytes, String, ToString},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that encodes `T` messages to a byte stream.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/io/encode.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/io/encode.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let message_decoder = s.decode_lines();
///     let counter = s.count::<String>();
///     let count_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &message_decoder.input);
///     s.connect(&message_decoder.output, &counter.input);
///     s.connect(&counter.count, &count_encoder.input);
///     s.connect(&count_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Encode encoding=text
/// ```
///
/// ```console
/// $ protoflow execute Encode encoding=protobuf
/// ```
///
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
        Self::with_params(input, output, None)
    }

    pub fn with_params(
        input: InputPort<T>,
        output: OutputPort<Bytes>,
        encoding: Option<Encoding>,
    ) -> Self {
        Self {
            input,
            output,
            encoding: encoding.unwrap_or_default(),
        }
    }
}

impl<T: Message + ToString + 'static> Encode<T> {
    pub fn with_system(system: &System, encoding: Option<Encoding>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output(), encoding)
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
