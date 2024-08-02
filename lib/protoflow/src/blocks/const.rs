// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{Block, BlockResult, BlockRuntime, Message, OutputPort};

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

impl<T: Message + Clone + 'static> Const<T> {
    pub fn new(output: OutputPort<T>, value: T) -> Self {
        Self { output, value }
    }
}

impl<T: Message + Clone + 'static> Block for Const<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        self.output.send(&self.value)?;
        self.output.close()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::blocks::Const;
    use crate::transports::MockTransport;
    use crate::System;

    #[test]
    fn const_block() -> Result<(), ()> {
        let system = System::<MockTransport>::build(|s| {
            let _const_1 = s.block(Const {
                output: s.output(),
                value: 42,
            });
        });
        let process = system.execute().unwrap();
        process.join().unwrap();
        Ok(())
    }
}
