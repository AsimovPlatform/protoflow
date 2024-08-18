// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{prelude::Bytes, Block, BlockResult, BlockRuntime, InputPort};
use protoflow_derive::Block;

/// A block that writes bytes to standard output (aka stdout).
///
/// # Examples
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
#[derive(Block, Clone)]
pub struct WriteStdout {
    /// The input message stream.
    #[input]
    pub input: InputPort<Bytes>,
}

impl WriteStdout {
    pub fn new(input: InputPort<Bytes>) -> Self {
        Self { input }
    }
}

impl Block for WriteStdout {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        let mut stdout = std::io::stdout().lock();

        runtime.wait_for(&self.input)?;

        while let Some(message) = self.input.recv()? {
            std::io::Write::write_all(&mut stdout, &message)?;
        }

        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for WriteStdout {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        use crate::{SysBlocks, SystemBuilding};

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let stdout = s.write_stdout();
            s.connect(&stdin.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::WriteStdout;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(WriteStdout::new(s.input()));
        });
    }
}
