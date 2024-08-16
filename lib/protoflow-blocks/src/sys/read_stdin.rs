// This is free and unencumbered software released into the public domain.

extern crate std;

use protoflow_core::{
    prelude::{Bytes, Vec},
    Block, BlockResult, BlockRuntime, OutputPort,
};
use protoflow_derive::Block;

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
        let mut buffer = Vec::with_capacity(self.buffer_size);
        let mut stdin = std::io::stdin().lock();

        runtime.wait_for(&self.output)?;

        while std::io::Read::read(&mut stdin, &mut buffer).is_ok() {
            let bytes = Bytes::from(buffer.clone());
            self.output.send(&bytes)?;
            buffer.clear();
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
