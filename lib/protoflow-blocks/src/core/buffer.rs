// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{prelude::VecDeque, Block, BlockResult, BlockRuntime, InputPort, Message};
use protoflow_derive::Block;

/// A block that simply stores all messages it receives.
///
/// # Examples
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let buffer = s.buffer();
///     s.connect(&stdin.output, &buffer.input);
/// });
/// # }
/// ```
///
#[derive(Block, Clone)]
pub struct Buffer<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The internal state storing the messages received.
    #[state]
    messages: VecDeque<T>,
}

impl<T: Message> Buffer<T> {
    pub fn new(input: InputPort<T>) -> Self {
        Self {
            input,
            messages: VecDeque::new(),
        }
    }

    pub fn messages(&self) -> &VecDeque<T> {
        &self.messages
    }
}

impl<T: Message> Block for Buffer<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.messages.push_back(message);
        }
        self.input.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Buffer<T> {
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let buffer = s.buffer();
            s.connect(&stdin.output, &buffer.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Buffer::<i32>::new(s.input()));
        });
    }
}
