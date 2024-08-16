// This is free and unencumbered software released into the public domain.

use protoflow_core::{Block, BlockResult, BlockRuntime, Message, OutputPort};
use protoflow_derive::Block;

/// A block for sending a constant value.
#[derive(Block, Clone)]
pub struct Const<T: Message> {
    /// The port to send the value on.
    #[output]
    pub output: OutputPort<T>,

    /// A parameter for the value to send.
    #[parameter]
    pub value: T,
}

impl<T: Message> Const<T> {
    pub fn new(output: OutputPort<T>, value: T) -> Self {
        Self { output, value }
    }
}

impl<T: Message> Block for Const<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        self.output.send(&self.value)?;
        self.output.close()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Const;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Const::<i32>::new(s.output(), 0x00BAB10C));
        });
    }
}
