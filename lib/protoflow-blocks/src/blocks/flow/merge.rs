// This is free and unencumbered software released into the public domain.
extern crate std;

use crate::prelude::{format, Arc};
use crate::{FlowBlocks, StdioConfig, StdioError, StdioSystem, SysBlocks, System};
use protoflow_core::BlockError;
use protoflow_core::{
    types::Any, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// Combines multiple input message streams into a single output stream by interleaving messages as they arrive.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/flow/merge.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/flow/merge.seq.mmd" framed)]
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
///
///     let replicate = s.replicate();
///     s.connect(&stdin.output, &replicate.input);
///
///     let merge = s.block(Merge::new(s.input(), s.input(), s.output()));
///     s.connect(&replicate.output_1, &merge.input_1);
///     s.connect(&replicate.output_2, &merge.input_2);
///
///     let stdout_1 = s.write_stdout();
///     s.connect(&merge.output, &stdout_1.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Merge
/// ```
///
#[derive(Block, Clone)]
pub struct Merge<T: Message = Any> {
    /// The input message stream.
    #[input]
    pub input_1: InputPort<T>,
    #[output]
    pub input_2: InputPort<T>,
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Message> Merge<T> {
    pub fn new(input_1: InputPort<T>, input_2: InputPort<T>, output: OutputPort<T>) -> Self {
        Self {
            input_1,
            input_2,
            output,
        }
    }
}
impl<T: Message + 'static> Merge<T> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.input(), system.output())
    }
}

impl<T: Message + Send + 'static> Block for Merge<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        let input1 = Arc::new(self.input_1.clone());
        let input2 = Arc::new(self.input_2.clone());
        let output = Arc::new(self.output.clone());

        fn process_port<T: Message + Send + 'static>(
            input: Arc<InputPort<T>>,
            output: Arc<OutputPort<T>>,
        ) -> Result<(), BlockError> {
            while let Ok(Some(message)) = input.recv() {
                if let Err(err) = output.send(&message) {
                    #[cfg(feature = "tracing")]
                    tracing::error!("Error sending message: {}", err);
                    return Err(BlockError::Other(format!("Error sending message: {}", err)));
                }
            }
            Ok(())
        }

        let input1_thread = {
            let input1_clone = Arc::clone(&input1);
            let output_clone = Arc::clone(&output);
            std::thread::spawn(move || process_port(input1_clone, output_clone))
        };

        let input2_thread = {
            let input2_clone = Arc::clone(&input2);
            let output_clone = Arc::clone(&output);
            std::thread::spawn(move || process_port(input2_clone, output_clone))
        };

        if let Err(_) = input1_thread.join() {
            #[cfg(feature = "tracing")]
            tracing::error!("Thread for input1 panicked");
            return Err(BlockError::Other("Thread for input1 panicked".into()));
        }

        if let Err(_) = input2_thread.join() {
            #[cfg(feature = "tracing")]
            tracing::error!("Thread for input2 panicked");
            return Err(BlockError::Other("Thread for input2 panicked".into()));
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Merge<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;
        config.reject_any()?;

        Ok(System::build(|s| {
            let stdin = s.read_stdin();

            let replicate = s.replicate();
            s.connect(&stdin.output, &replicate.input);

            let merge = s.block(Merge::new(s.input(), s.input(), s.output()));

            s.connect(&replicate.output_1, &merge.input_1);
            s.connect(&replicate.output_2, &merge.input_2);

            let stdout_1 = s.write_stdout();
            s.connect(&merge.output, &stdout_1.input);
        }))
    }
}

#[cfg(test)]
mod merge_tests {
    use crate::{FlowBlocks, SysBlocks, System};
    use protoflow_core::{prelude::String, SystemBuilding};
    #[cfg(feature = "tracing")]
    use tracing::error;

    use super::Merge;

    extern crate std;

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.merge::<String>();
        });
    }
    #[test]
    #[ignore = "requires stdin"]
    fn run_block() {
        if let Err(e) = System::run(|s| {
            let stdin = s.read_stdin();

            let replicate = s.replicate();
            s.connect(&stdin.output, &replicate.input);

            let merge = s.block(Merge::new(s.input(), s.input(), s.output()));

            s.connect(&replicate.output_1, &merge.input_1);
            s.connect(&replicate.output_2, &merge.input_2);

            let stdout_1 = s.write_stdout();
            s.connect(&merge.output, &stdout_1.input);
        }) {
            #[cfg(feature = "tracing")]
            error!("{}", e)
        }
    }
}
