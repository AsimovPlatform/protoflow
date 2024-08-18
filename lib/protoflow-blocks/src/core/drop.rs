// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, Message};
use protoflow_derive::Block;

/// A block that simply discards all messages it receives.
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
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Drop::<i32>::new(s.input()));
        });
    }
}
