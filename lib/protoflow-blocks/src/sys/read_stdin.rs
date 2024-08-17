// This is free and unencumbered software released into the public domain.

extern crate std;

use protoflow_core::{
    prelude::{vec, Bytes},
    Block, BlockResult, BlockRuntime, OutputPort,
};
use protoflow_derive::Block;
use std::io::Read;

/// The default buffer size for reading from standard input.
const DEFAULT_BUFFER_SIZE: usize = 1024;

/// A block that reads bytes from standard input (aka stdin).
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
}

impl Block for ReadStdin {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        let stdin = std::io::stdin().lock();
        let mut reader = std::io::BufReader::new(stdin);
        let mut buffer = vec![0; self.buffer_size];

        runtime.wait_for(&self.output)?;

        loop {
            match reader.read(&mut buffer) {
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err.into()),
                Ok(0) => break, // EOF
                Ok(buffer_len) => {
                    buffer.resize(buffer_len, b'\0');
                    let bytes = Bytes::from(buffer.clone());
                    self.output.send(&bytes)?;
                    buffer.clear();
                }
            }
        }

        self.output.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ReadStdin;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(ReadStdin::new(s.output()));
        });
    }
}
