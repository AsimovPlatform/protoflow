// This is free and unencumbered software released into the public domain.

use crate::{BlockDescriptor, BlockResult, BlockRuntime};

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

/// A block is an autonomous unit of computation in a system.
pub trait Block: AsBlock + BlockDescriptor + Send + Sync {
    /// Prepares this block for execution.
    ///
    /// This is called once before the first call to `execute`.
    /// This is where to open ports and allocate resources.
    fn prepare(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        Ok(())
    }

    /// Executes this block's computation.
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult;
}

pub trait AsBlock {
    fn as_block(&self) -> &dyn Block;
}

impl<T: Block + Sized> AsBlock for T {
    fn as_block(&self) -> &dyn Block {
        self
    }
}
