// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{prelude::Bytes, Block, BlockResult, BlockRuntime, OutputPort};
use protoflow_derive::Block;

/// A block that reads bytes from standard input (aka stdin).
#[derive(Block, Clone)]
pub struct ReadStdin {
    /// The output message stream.
    #[output]
    pub output: OutputPort<Bytes>,
}

impl ReadStdin {
    pub fn new(output: OutputPort<Bytes>) -> Self {
        Self { output }
    }
}

impl Block for ReadStdin {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        unimplemented!() // TODO
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
