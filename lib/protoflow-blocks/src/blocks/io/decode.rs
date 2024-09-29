// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{types::Encoding, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{Bytes, FromStr, String, ToString, Vec},
    Block, BlockError, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use std::io::BufRead;

/// A block that decodes `T` messages from a byte stream.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/io/decode.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/io/decode.seq.mmd" framed)]
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
/// $ protoflow execute Decode encoding=text
/// ```
///
/// ```console
/// $ protoflow execute Decode encoding=protobuf
/// ```
///
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
    pub encoding: Encoding,
}

impl<T: Message + FromStr> Decode<T> {
    pub fn new(input: InputPort<Bytes>, output: OutputPort<T>) -> Self {
        Self::with_params(input, output, None)
    }

    pub fn with_params(
        input: InputPort<Bytes>,
        output: OutputPort<T>,
        encoding: Option<Encoding>,
    ) -> Self {
        Self {
            input,
            output,
            encoding: encoding.unwrap_or_default(),
        }
    }
}

impl<T: Message + FromStr + 'static> Decode<T> {
    pub fn with_system(system: &System, encoding: Option<Encoding>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output(), encoding)
    }
}

impl<T: Message + FromStr> Block for Decode<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let mut buffer = Vec::<u8>::new();

        while let Some(chunk) = self.input.recv()? {
            buffer.extend_from_slice(&chunk);

            let mut cursor = std::io::Cursor::new(&buffer);

            let _message = match self.encoding {
                Encoding::ProtobufWithLengthPrefix => todo!(), // TODO
                Encoding::ProtobufWithoutLengthPrefix => todo!(), // TODO
                Encoding::TextWithNewlineSuffix => {
                    if !chunk.contains(&b'\n') {
                        continue; // skip useless chunks
                    }
                    let mut line = String::new();
                    while cursor.read_line(&mut line)? > 0 {
                        if !line.ends_with('\n') {
                            cursor.set_position(cursor.position() - line.len() as u64);
                            break;
                        }
                        let stripped_line =
                            line.strip_suffix('\n').expect("line ends with newline");
                        match T::from_str(stripped_line) {
                            Ok(message) => self.output.send(&message)?,
                            Err(_error) => {
                                BlockError::Other("decode error".to_string()); // FIXME
                            }
                        }
                        line.clear();
                    }
                }
            };

            buffer.drain(..cursor.position() as usize);
        }

        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Decode {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        //use crate::{CoreBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|_s| todo!()))
    }
}

#[cfg(test)]
mod tests {
    use super::Decode;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Decode::<i32>::new(s.input(), s.output()));
        });
    }
}
