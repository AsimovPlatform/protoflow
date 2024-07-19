// This is free and unencumbered software released into the public domain.

use crate::PortDescriptor;

/// A block is an autonomous unit of computation in a system.
pub trait Block: AsBlock {
    /// The machine-readable name of this block.
    fn name(&self) -> Option<String> {
        None
    }

    /// A human-readable label for this block.
    fn label(&self) -> Option<String> {
        None
    }

    /// A description of this block's input ports.
    fn inputs(&self) -> Vec<PortDescriptor>;

    /// A description of this block's output ports.
    fn outputs(&self) -> Vec<PortDescriptor>;

    /// Prepares this block for execution.
    ///
    /// This is called once before the first call to `execute`.
    /// This is where to open ports and allocate resources.
    fn prepare(&mut self) {}

    /// Executes this block's computation.
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
