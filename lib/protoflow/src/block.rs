// This is free and unencumbered software released into the public domain.

use crate::Port;
use std::rc::Rc;

#[allow(unused)]
pub trait Block: AsBlock {
    fn name(&self) -> Option<String> {
        None
    }

    fn inputs(&self) -> Vec<Rc<dyn Port>>;

    fn outputs(&self) -> Vec<Rc<dyn Port>>;

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
