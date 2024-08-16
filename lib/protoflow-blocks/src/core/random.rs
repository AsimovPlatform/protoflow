// This is free and unencumbered software released into the public domain.

use protoflow_core::{Block, BlockResult, BlockRuntime, Message, OutputPort};
use protoflow_derive::Block;

/// A block for sending a random value.
#[derive(Block, Clone)]
pub struct Random<T: Message> {
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

impl<T: Message + Default> Block for Random<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        self.output.send(&T::default())?; // TODO
        self.output.close()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Random;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Random::<i32>::new(s.output()));
        });
    }
}
