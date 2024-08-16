// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use protoflow_core::{prelude::String, Block, BlockResult, BlockRuntime, InputPort, OutputPort};
use protoflow_derive::Block;

/// A block that reads file names from a file system directory.
#[derive(Block, Clone)]
pub struct ReadDir {
    /// The path to the directory to read.
    #[input]
    pub path: InputPort<String>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<String>,
}

impl ReadDir {
    pub fn new(path: InputPort<String>, output: OutputPort<String>) -> Self {
        Self { path, output }
    }
}

impl Block for ReadDir {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        unimplemented!() // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::ReadDir;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(ReadDir::new(s.input(), s.output()));
        });
    }
}
