// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, Message};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that simply discards all messages it receives.
#[doc = mermaid!("../../doc/core/drop.mmd")]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let dropper = s.drop();
///     s.connect(&stdin.output, &dropper.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Drop
/// ```
///
#[derive(Block, Clone)]
pub struct Drop<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,
}

impl<T: Message> Drop<T> {
    pub fn new(input: InputPort<T>) -> Self {
        Self { input }
    }
}

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            drop(message);
            self.input.close()?;
        }
        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Drop<T> {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let dropper = s.drop();
            s.connect(&stdin.output, &dropper.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Drop;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Drop::<i32>::new(s.input()));
        });
    }
}
