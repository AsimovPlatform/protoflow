// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{Bytes, String},
    Block, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;

/// A block that reads the contents of a file.
#[derive(Block, Clone)]
pub struct ReadFile {
    /// The path to the file to read from.
    #[input]
    pub path: InputPort<String>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<Bytes>,
}

impl ReadFile {
    pub fn new(path: InputPort<String>, output: OutputPort<Bytes>) -> Self {
        Self { path, output }
    }
}

impl Block for ReadFile {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        unimplemented!() // TODO
    }
}

#[cfg(feature = "std")]
impl StdioSystem for ReadFile {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        //use crate::{CoreBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|_s| todo!()))
    }
}

#[cfg(test)]
mod tests {
    use super::ReadFile;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(ReadFile::new(s.input(), s.output()));
        });
    }
}
