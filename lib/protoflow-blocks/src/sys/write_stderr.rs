// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{prelude::Bytes, Block, BlockResult, BlockRuntime, InputPort};
use protoflow_derive::Block;

/// A block that writes bytes to standard error (aka stderr).
#[derive(Block, Clone)]
pub struct WriteStderr {
    /// The input message stream.
    #[input]
    pub input: InputPort<Bytes>,
}

impl WriteStderr {
    pub fn new(input: InputPort<Bytes>) -> Self {
        Self { input }
    }
}

impl Block for WriteStderr {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(_message) = self.input.recv()? {
            unimplemented!() // TODO
        }
        self.input.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::WriteStderr;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(WriteStderr::new(s.input()));
        });
    }
}
