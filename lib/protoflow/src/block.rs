// This is free and unencumbered software released into the public domain.

use crate::{InputPort, OutputPort};
//use prost::Message;

#[allow(unused)]
pub trait Block: AsBlock {
    fn inputs(&self) -> Vec<InputPort>;
    fn outputs(&self) -> Vec<OutputPort>;
    fn execute(&mut self);
}

pub trait AsBlock {
    fn as_block(&self) -> &dyn Block;
}

impl<T: Sized + Block> AsBlock for T {
    fn as_block(&self) -> &dyn Block {
        self
    }
}
