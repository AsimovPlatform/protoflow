// This is free and unencumbered software released into the public domain.

use crate as protoflow;

use protoflow::derive::Block;
use protoflow::{Block, BlockError, InputPort, Message, Scheduler};

/// A block that simply discards all messages it receives.
#[derive(Block)]
pub struct Drop<T: Message>(#[input] pub InputPort<T>);

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _scheduler: &dyn Scheduler) -> Result<(), BlockError> {
        while let Some(message) = self.0.receive()? {
            drop(message);
        }
        Ok(())
    }
}
