// This is free and unencumbered software released into the public domain.

use crate::{prelude::Vec, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    types::Any, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that stores all messages it receives,
/// and sends them downstream when triggered.
///
/// When triggered, the block will send all messages it received so far,
/// _WITHOUT_ clearing the internal buffer.
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
pub struct Buffer<Input: Message = Any, Trigger: Message = ()> {
    /// The input message stream.
    #[input]
    pub input: InputPort<Input>,

    /// The trigger port.
    #[input]
    pub trigger: InputPort<Trigger>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<Input>,

    /// The internal state storing the messages received.
    #[state]
    messages: Vec<Input>,
}

impl<Input: Message, Trigger: Message> Buffer<Input, Trigger> {
    pub fn new(
        input: InputPort<Input>,
        trigger: InputPort<Trigger>,
        output: OutputPort<Input>,
    ) -> Self {
        Self {
            input,
            trigger,
            output,
            messages: Vec::new(),
        }
    }

    pub fn messages(&self) -> &Vec<Input> {
        &self.messages
    }
}

impl<Input: Message + 'static, Trigger: Message + 'static> Buffer<Input, Trigger> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.input(), system.output())
    }
}

impl<Input: Message, Trigger: Message> Block for Buffer<Input, Trigger> {
    fn execute(&mut self, _: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.messages.push(message);
        }

        while let Some(_) = self.trigger.recv()? {
            for message in &self.messages {
                self.output.send(message)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Buffer {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let buffer = s.buffer::<_, ()>();
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
            let _ = s.block(Buffer::<i32>::new(s.input(), s.input(), s.output()));
        });
    }
}
