// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, BTreeSet, Rc, Vec},
    Block, InputPort, Message, OutputPort, Port, PortID,
};

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

/// A system is a collection of blocks that are connected together.
#[derive(Default)]
pub struct System {
    /// The registered blocks in the system.
    pub blocks: Vec<Rc<dyn Block>>,
    pub connections: BTreeSet<(PortID, PortID)>,
}

pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            connections: BTreeSet::new(),
        }
    }

    pub fn blocks(&self) -> &[Rc<dyn Block>] {
        &self.blocks
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
        source: &OutputPort<T>,
        target: &InputPort<T>,
    ) -> Result<bool, ()> {
        // TODO: assign port IDs
        match (source.id(), target.id()) {
            (Some(source_id), Some(target_id)) => {
                Ok(self.connections.insert((source_id, target_id)))
            }
            _ => Err(()),
        }
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
