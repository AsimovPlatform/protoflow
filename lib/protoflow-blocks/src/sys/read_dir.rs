// This is free and unencumbered software released into the public domain.

extern crate std;

use protoflow_core::{
    prelude::{String, ToString},
    Block, BlockResult, BlockRuntime, InputPort, OutputPort,
};
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
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.path)?;
        let dir_path = self.path.recv()?.unwrap();
        //self.path.close()?; // FIXME

        let dir = std::fs::read_dir(dir_path)?;
        for dir_entry in dir {
            let file_path = dir_entry?.path();
            //let file_path = file_path.strip_prefix("./").unwrap(); // TODO: parameter
            let file_path = file_path.to_string_lossy().to_string();
            self.output.send(&file_path)?;
        }

        self.output.close()?;
        Ok(())
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
