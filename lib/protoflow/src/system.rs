// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{slice, vec, Arc, BTreeSet, BTreeSetIter, Rc, RefCell, Vec},
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
    pub(crate) blocks: RefCell<Vec<Arc<dyn Block>>>,
    pub(crate) connections: RefCell<BTreeSet<(PortID, PortID)>>,
    pub(crate) source_id: RefCell<PortID>,
    pub(crate) target_id: RefCell<PortID>,
}

pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            blocks: RefCell::new(vec![]),
            connections: RefCell::new(BTreeSet::new()),
            source_id: RefCell::new(-1),
            target_id: RefCell::new(1),
            ..Default::default()
        })
    }

    pub fn blocks(&self) -> slice::Iter<Arc<dyn Block>> {
        todo!() // self.blocks.iter()
    }

    pub fn connections(&self) -> BTreeSetIter<(PortID, PortID)> {
        todo!() //self.connections.iter()
    }

    /// Instantiates a block in the system.
    pub fn block<T: Block + 'static>(&self, block: T) -> Arc<T> {
        let result = Arc::new(block);
        self.blocks
            .borrow_mut()
            .push(result.clone() as Arc<dyn Block>);
        result
    }

    /// Connects two ports of two blocks in the system.
    ///
    /// Both ports must be of the same message type.
    pub fn connect<T: Message>(
        &self,
        source: &OutputPort<T>,
        target: &InputPort<T>,
    ) -> Result<bool, ()> {
        Ok(self
            .connections
            .borrow_mut()
            .insert((source.id().unwrap(), target.id().unwrap())))
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use crate::blocks::{Const, Drop};
    use crate::{InputPort, OutputPort, System};

    #[test]
    fn define_system() -> Result<(), ()> {
        let system = System::new();

        let constant = system.block(Const {
            output: OutputPort::new(&system),
            value: 42,
        });
        let blackhole = system.block(Drop(InputPort::new(&system)));

        system.connect(&constant.output, &blackhole.0)?;

        Ok(())
    }
}
