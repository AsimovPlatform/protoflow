// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{prelude::VecDeque, Block, BlockResult, BlockRuntime, InputPort, Message};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that simply stores all messages it receives.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/core/buffer.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/core/buffer.seq.mmd" framed)]
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
///     let buffer = s.buffer();
///     s.connect(&stdin.output, &buffer.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Buffer
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
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Buffer<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, SystemBuilding};

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
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
