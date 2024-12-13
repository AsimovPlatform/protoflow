// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{vec, Vec},
    types::Any,
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// Batches input strem into chunks of a specified size.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/flow/batch.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/flow/batch.seq.mmd" framed)]
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
///     let batch = s.batch().batch(2);
///     s.connect(&stdin.output, &batch.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Batch
/// ```
///
#[derive(Block, Clone)]
pub struct Batch<T: Message = Any> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,

    /// Batch size
    #[parameter]
    pub batch_size: usize,

    /// The internal state storing the messages received.
    #[state]
    messages: Vec<T>,
}

impl<T: Message> Batch<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self::with_params(input, output, None)
    }
    pub fn with_params(
        input: InputPort<T>,
        output: OutputPort<T>,
        batch_size: Option<usize>,
    ) -> Self {
        Self {
            input,
            output,
            batch_size: batch_size.unwrap_or(1),
            messages: Vec::new(),
        }
    }
    pub fn messages(&self) -> &Vec<T> {
        &self.messages
    }
}

impl<T: Message + 'static> Batch<T> {
    pub fn with_system(system: &System, batch_size: Option<usize>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output(), batch_size)
    }
}

impl<T: Message> Block for Batch<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            #[cfg(feature = "tracing")]
            tracing::info!("Buffered one message");
            self.messages.push(message);

            if self.batch_size == self.messages().len() {
                #[cfg(feature = "tracing")]
                tracing::info!("Sending messages");
                for message in self.messages.drain(..) {
                    self.output.send(&message)?
                }
            }
        }

        //send remaining messages
        tracing::info!("Sending remaining messages");
        for message in self.messages.drain(..) {
            self.output.send(&message)?
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Batch<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;

        config.allow_only(vec!["batch-size"])?;

        Ok(System::build(|s| {
            let batch_size = config.get::<usize>("batch-size").unwrap_or(1);
            let stdin = config.read_stdin(s);
            let batch = Batch::with_system(&s, Some(batch_size));
            s.connect(&stdin.output, &batch.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Batch;
    use crate::{FlowBlocks, System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Batch::<i32>::new(s.input(), s.output()));
        });
    }

    #[test]
    #[ignore = "requires stdin"]
    fn run_batch_stdout() {
        use super::*;
        use crate::SysBlocks;
        use protoflow_core::SystemBuilding;
        #[cfg(feature = "tracing")]
        use tracing::error;

        if let Err(e) = System::run(|s| {
            let stdin = s.read_stdin();
            let batch = s.batch(2);
            s.connect(&stdin.output, &batch.input);

            let stdout_1 = s.write_stdout();
            s.connect(&batch.output, &stdout_1.input);
        }) {
            #[cfg(feature = "tracing")]
            error!("{}", e)
        }
    }
}
