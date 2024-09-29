// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{String, ToString},
    Block, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that reads file names from a file system directory.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/sys/read_dir.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/sys/read_dir.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let path_param = s.const_string("/tmp");
///     let dir_reader = s.read_dir();
///     let line_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&path_param.output, &dir_reader.path);
///     s.connect(&dir_reader.output, &line_encoder.input);
///     s.connect(&line_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute ReadDir path=/tmp
/// ```
///
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

    pub fn with_system(system: &mut System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
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

#[cfg(feature = "std")]
impl StdioSystem for ReadDir {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SysBlocks, SystemBuilding};

        let path = config.get_string("path")?;

        Ok(System::build(|s| {
            let path_param = s.const_string(path);
            let dir_reader = s.read_dir();
            let line_encoder = s.encode_with(config.encoding);
            let stdout = config.write_stdout(s);
            s.connect(&path_param.output, &dir_reader.path);
            s.connect(&dir_reader.output, &line_encoder.input);
            s.connect(&line_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::ReadDir;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(ReadDir::new(s.input(), s.output()));
        });
    }
}
