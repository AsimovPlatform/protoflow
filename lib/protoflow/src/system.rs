// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{slice, vec, BTreeSet, BTreeSetIter, Rc, Vec},
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
    pub source_id: PortID,
    pub target_id: PortID,
}

pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            connections: BTreeSet::new(),
            ..Default::default()
        }
    }

    pub fn blocks(&self) -> slice::Iter<Rc<dyn Block>> {
        self.blocks.iter()
    }

    pub fn connections(&self) -> BTreeSetIter<(PortID, PortID)> {
        self.connections.iter()
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
        if source.id().is_none() {
            self.source_id -= 1;
            source.id.replace(Some(self.source_id));
        }
        if target.id().is_none() {
            self.target_id += 1;
            target.id.replace(Some(self.target_id));
        }
        Ok(self
            .connections
            .insert((source.id().unwrap(), target.id().unwrap())))
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
            output: OutputPort::default(),
            value: 42,
        });
        let blackhole = system.block(Drop(InputPort::default()));

        system.connect(&constant.output, &blackhole.0)?;

        Ok(())
    }
}
