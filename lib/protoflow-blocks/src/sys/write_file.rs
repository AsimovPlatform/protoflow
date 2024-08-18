// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{Bytes, String},
    Block, BlockResult, BlockRuntime, InputPort,
};
use protoflow_derive::Block;

/// A block that writes or appends to the contents of a file.
#[derive(Block, Clone)]
pub struct WriteFile {
    /// The path to the file to write to.
    #[input]
    pub path: InputPort<String>,

    /// The input message stream.
    #[input]
    pub input: InputPort<Bytes>,
}

impl WriteFile {
    pub fn new(path: InputPort<String>, input: InputPort<Bytes>) -> Self {
        Self { path, input }
    }
}

impl Block for WriteFile {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(_message) = self.input.recv()? {
            unimplemented!() // TODO
        }
        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for WriteFile {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        //use crate::{CoreBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|_s| todo!()))
    }
}

#[cfg(test)]
mod tests {
    use super::WriteFile;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(WriteFile::new(s.input(), s.input()));
        });
    }
}
