// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{prelude::Bytes, Block, BlockResult, BlockRuntime, InputPort};
use protoflow_derive::Block;

/// A block that writes bytes to standard error (aka stderr).
///
/// # Examples
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let stderr = s.write_stderr();
///     s.connect(&stdin.output, &stderr.input);
/// });
/// # }
/// ```
///
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
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        let mut stderr = std::io::stderr().lock();

        runtime.wait_for(&self.input)?;

        while let Some(message) = self.input.recv()? {
            std::io::Write::write_all(&mut stderr, &message)?;
        }

        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for WriteStderr {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        use crate::{SysBlocks, SystemBuilding};

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let stderr = s.write_stderr();
            s.connect(&stdin.output, &stderr.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::WriteStderr;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(WriteStderr::new(s.input()));
        });
    }
}
