// This is free and unencumbered software released into the public domain.

use crate::{prelude::Vec, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    types::Any, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that keeps all messages it receives,
/// and sends them downstream when triggered.
///
/// When triggered, the block will send all messages it received since last trigger,
/// and _WILL_ clean the internal buffer.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/flow/gate.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/flow/gate.seq.mmd" framed)]
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
///     let hello = s.const_string("Hello, World!");
///     let encode = s.encode_lines();
///     let gate = s.gate();
///     let stdout = s.write_stdout();
///     s.connect(&hello.output, &encode.input);
///     s.connect(&encode.output, &gate.input);
///     s.connect(&stdin.output, &gate.trigger);
///     s.connect(&gate.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Gate
/// ```
///
#[derive(Block, Clone)]
pub struct Gate<Input: Message = Any, Trigger: Message = ()> {
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

impl<Input: Message, Trigger: Message> Gate<Input, Trigger> {
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

impl<Input: Message + 'static, Trigger: Message + 'static> Gate<Input, Trigger> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.input(), system.output())
    }
}

impl<Input: Message, Trigger: Message> Block for Gate<Input, Trigger> {
    fn execute(&mut self, _: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.messages.push(message);
        }

        while let Some(_) = self.trigger.recv()? {
            let iter = self.messages.drain(..);
            for message in iter {
                self.output.send(&message)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<Input: Message, Trigger: Message> StdioSystem for Gate<Input, Trigger> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{FlowBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let gate = s.gate::<_, ()>();
            s.connect(&stdin.output, &gate.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Gate;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Gate::<i32>::new(s.input(), s.input(), s.output()));
        });
    }
}
