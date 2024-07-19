// This is free and unencumbered software released into the public domain.

use crate::Block;

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

/// A system is a collection of blocks that are connected together.
#[derive(Default)]
pub struct System {
    /// The registered blocks in the system.
    blocks: Vec<Box<dyn Block>>,
}

pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    /// Instantiates a block in the system.
    pub fn block(&mut self, block: Box<dyn Block>) -> BlockID {
        self.blocks.push(block);
        self.blocks.len()
    }

    /// Connects two ports of two blocks in the system.
    pub fn connect(
        &mut self,
        _source_block: BlockID,
        _source_port: &str,
        _target_block: BlockID,
        _target_port: &str,
    ) {
        // TODO
    }
}
