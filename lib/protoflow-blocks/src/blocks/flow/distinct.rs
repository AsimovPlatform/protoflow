// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::Vec, types::Any, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// Removes duplicate values from the input stream.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/flow/distinct.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/flow/distinct.seq.mmd" framed)]
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
///     let distinct = s.distinct();
///     s.connect(&stdin.output, &distinct.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Distinct
/// ```
///
#[derive(Block, Clone)]
pub struct Distinct<T: Message = Any> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,

    /// The internal state storing the messages received.
    #[state]
    messages: Vec<T>,
}

impl<T: Message> Distinct<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self {
            input,
            output,
            messages: Vec::new(),
        }
    }

    pub fn messages(&self) -> &Vec<T> {
        &self.messages
    }
}

impl<T: Message + 'static> Distinct<T> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Message + PartialEq> Block for Distinct<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            #[cfg(feature = "tracing")]
            tracing::info!("Buffered one message");
            if !self.messages.contains(&message) {
                self.messages.push(message);
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!("Sending messages");
        for message in self.messages.drain(..) {
            self.output.send(&message)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Distinct<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;

        config.reject_any()?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let distinct = s.block(Distinct::new(s.input(), s.output()));
            s.connect(&stdin.output, &distinct.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Distinct;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Distinct::<i32>::new(s.input(), s.output()));
        });
    }

    #[test]
    #[ignore = "requires stdin"]
    fn run_distinct_stdout() {
        use super::*;
        use crate::SysBlocks;
        use protoflow_core::SystemBuilding;
        #[cfg(feature = "tracing")]
        use tracing::error;

        if let Err(e) = System::run(|s| {
            let stdin = s.read_stdin();
            let distinct = s.block(Distinct::new(s.input(), s.output()));
            s.connect(&stdin.output, &distinct.input);

            let stdout_1 = s.write_stdout();
            s.connect(&distinct.output, &stdout_1.input);
        }) {
            #[cfg(feature = "tracing")]
            error!("{}", e)
        }
    }
}
