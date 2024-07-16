// This is free and unencumbered software released into the public domain.

use crate::Block;

#[allow(unused)]
#[derive(Default)]
pub struct System {
    blocks: Vec<Box<dyn Block>>,
}

impl System {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Box<dyn Block>) -> usize {
        self.blocks.push(block);
        self.blocks.len()
    }
}
