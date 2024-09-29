// This is free and unencumbered software released into the public domain.

use crate::{IoBlocks, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{format, Bytes, String},
    Block, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that encodes a byte stream into hexadecimal form.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/io/encode_hex.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/io/encode_hex.seq.mmd" framed)]
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
///     let hex_encoder = s.encode_hex();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &hex_encoder.input);
///     s.connect(&hex_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute EncodeHex
/// ```
///
#[derive(Block, Clone)]
pub struct EncodeHex {
    /// The input byte stream.
    #[input]
    pub input: InputPort<Bytes>,

    /// The output text stream.
    #[output]
    pub output: OutputPort<Bytes>,
}

impl EncodeHex {
    pub fn new(input: InputPort<Bytes>, output: OutputPort<Bytes>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl Block for EncodeHex {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        while let Some(message) = self.input.recv()? {
            let mut buffer = String::with_capacity(message.len() * 2);
            for byte in message.iter() {
                buffer.push_str(format!("{:02x}", byte).as_str()); // TODO: optimize
            }
            let message = Bytes::from(buffer);
            self.output.send(&message)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for EncodeHex {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let hex_encoder = s.encode_hex();
            let stdout = config.write_stdout(s);
            s.connect(&stdin.output, &hex_encoder.input);
            s.connect(&hex_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::EncodeHex;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(EncodeHex::new(s.input(), s.output()));
        });
    }
}
