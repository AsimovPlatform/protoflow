// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{vec, Bytes},
    Block, BlockResult, BlockRuntime, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use std::io::Read;

/// The default buffer size for reading from standard input.
const DEFAULT_BUFFER_SIZE: usize = 1024;

/// A block that reads bytes from standard input (aka stdin).
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/sys/read_stdin.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/sys/read_stdin.seq.mmd" framed)]
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
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute ReadStdin < input.txt
/// ```
///
/// ```console
/// $ protoflow execute ReadStdin buffer-size=1024 < input.txt
/// ```
///
#[derive(Block, Clone)]
pub struct ReadStdin {
    /// The output message stream.
    #[output]
    pub output: OutputPort<Bytes>,

    /// The maximum number of bytes to read at a time.
    #[parameter]
    pub buffer_size: usize,
}

impl ReadStdin {
    pub fn new(output: OutputPort<Bytes>) -> Self {
        Self::with_params(output, None)
    }

    pub fn with_params(output: OutputPort<Bytes>, buffer_size: Option<usize>) -> Self {
        Self {
            output,
            buffer_size: buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
        }
    }

    pub fn with_system(system: &mut System, buffer_size: Option<usize>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.output(), buffer_size)
    }
}

impl Block for ReadStdin {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        let stdin = std::io::stdin().lock();
        let mut reader = std::io::BufReader::new(stdin);
        let mut buffer = vec![0; self.buffer_size];

        runtime.wait_for(&self.output)?;

        loop {
            buffer.resize(self.buffer_size, b'\0'); // reinitialize the buffer
            buffer.fill(b'\0');

            match reader.read(&mut buffer) {
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err.into()),
                Ok(0) => break, // EOF
                Ok(buffer_len) => {
                    buffer.resize(buffer_len, b'\0'); // truncate the buffer
                    let bytes = Bytes::from(buffer.clone());
                    self.output.send(&bytes)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for ReadStdin {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let stdout = config.write_stdout(s);
            s.connect(&stdin.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::ReadStdin;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(ReadStdin::new(s.output()));
        });
    }
}
