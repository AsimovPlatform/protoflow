// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{Block, BlockResult, BlockRuntime, InputPort, Message};

/// A block that simply discards all messages it receives.
#[derive(Block, Clone)]
pub struct Drop<T: Message>(#[input] pub InputPort<T>);

impl<T: Message> Drop<T> {
    pub fn new(input: InputPort<T>) -> Self {
        Self(input)
    }
}

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.0.recv()? {
            drop(message);
        }
        self.0.close()?;
        Ok(())
    }
}
