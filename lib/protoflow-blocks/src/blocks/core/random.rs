// This is free and unencumbered software released into the public domain.

use crate::{prelude::vec, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, Message, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block for generating and sending a random value.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/core/random.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/core/random.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let random_generator = s.random::<u64>();
///     let number_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&random_generator.output, &number_encoder.input);
///     s.connect(&number_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Random
/// ```
///
/// ```console
/// $ protoflow execute Random seed=42
/// ```
///
#[derive(Block, Clone)]
pub struct Random<T: Message = u64> {
    /// The port to send the value on.
    #[output]
    pub output: OutputPort<T>,

    /// A parameter for the random seed to use.
    #[parameter]
    pub seed: Option<u64>,
}

impl<T: Message> Random<T> {
    pub fn new(output: OutputPort<T>) -> Self {
        Self::with_params(output, None)
    }

    pub fn with_params(output: OutputPort<T>, seed: Option<u64>) -> Self {
        Self { output, seed }
    }
}

impl<T: Message + 'static> Random<T> {
    pub fn with_system(system: &System, seed: Option<u64>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.output(), seed)
    }
}

impl<T: Message + Default> Block for Random<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        self.output.send(&T::default())?; // TODO

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Random<u64> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SystemBuilding};

        config.allow_only(vec!["seed"])?;
        let seed = config.get_opt::<u64>("seed")?;

        Ok(System::build(|s| {
            let random_generator = s.random_seeded::<u64>(seed);
            let number_encoder = s.encode_with::<u64>(config.encoding);
            let stdout = config.write_stdout(s);
            s.connect(&random_generator.output, &number_encoder.input);
            s.connect(&number_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Random;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Random::<i32>::new(s.output()));
        });
    }
}
