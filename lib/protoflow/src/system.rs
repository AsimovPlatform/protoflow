// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Rc, Vec},
    Block, InputPort, Message, OutputPort,
};

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

/// A system is a collection of blocks that are connected together.
#[derive(Default)]
pub struct System {
    /// The registered blocks in the system.
    blocks: Vec<Rc<dyn Block>>,
}

pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    /// Instantiates a block in the system.
    pub fn block<T: Block + 'static>(&mut self, block: T) -> Rc<T> {
        let result = Rc::new(block);
        self.blocks.push(result.clone() as Rc<dyn Block>);
        result
    }

    /// Connects two ports of two blocks in the system.
    ///
    /// Both ports must be of the same message type.
    pub fn connect<T: Message>(
        &mut self,
        _source: &OutputPort<T>,
        _target: &InputPort<T>,
    ) -> Result<(), ()> {
        Ok(()) // TODO
    }
}

#[cfg(test)]
mod test {
    use crate::blocks::{Const, Drop};
    use crate::{InputPort, OutputPort, System};

    #[test]
    fn define_system() -> Result<(), ()> {
        let mut system = System::new();

        let constant = system.block(Const {
            output: OutputPort::<i64>::default(),
            value: 42,
        });
        let blackhole = system.block(Drop(InputPort::<i64>::default()));

        system.connect(&constant.output, &blackhole.0)?;

        Ok(())
    }
}
