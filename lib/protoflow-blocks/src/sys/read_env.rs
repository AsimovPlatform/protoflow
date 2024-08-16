// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

extern crate std;

use protoflow_core::{
    prelude::{FromStr, String},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;

/// A block that reads the value of an environment variable.
#[derive(Block, Clone)]
pub struct ReadEnv<T: Message + FromStr = String> {
    /// The name of the environment variable to read.
    #[input]
    pub name: InputPort<String>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Message + FromStr> ReadEnv<T> {
    pub fn new(name: InputPort<String>, output: OutputPort<T>) -> Self {
        Self { name, output }
    }
}

impl<T: Message + FromStr> Block for ReadEnv<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.name)?;
        let name = self.name.recv()?.unwrap();
        //self.name.close()?; // FIXME

        let value = std::env::var(&name).unwrap_or_default();
        let value = T::from_str(&value).unwrap_or_default();
        self.output.send(&value)?;

        self.output.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ReadEnv;
    use protoflow_core::{transports::MockTransport, System};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(ReadEnv::<i32>::new(s.input(), s.output()));
        });
    }
}
