// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{slice, Arc, BTreeSet, BTreeSetIter, Box, RefCell, VecDeque},
    Block, InputPort, InputPortID, Message, OutputPort, OutputPortID, PortID,
};

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

/// A system is a collection of blocks that are connected together.
#[derive(Default)]
pub struct System {
    /// The registered blocks in the system.
    pub(crate) blocks: RefCell<VecDeque<Box<dyn Block>>>,
    pub(crate) connections: RefCell<BTreeSet<(OutputPortID, InputPortID)>>,
}

pub type Subsystem = System;

impl System {
    /// Instantiates a new system.
    pub fn new() -> Self {
        Self {
            blocks: RefCell::new(VecDeque::new()),
            connections: RefCell::new(BTreeSet::new()),
            ..Default::default()
        }
    }

    pub fn blocks(&self) -> slice::Iter<Arc<dyn Block>> {
        todo!() // self.blocks.iter()
    }

    pub fn connections(&self) -> BTreeSetIter<(PortID, PortID)> {
        todo!() //self.connections.iter()
    }

    /// Instantiates a block in the system.
    pub fn block<T: Block + Clone + 'static>(&self, block: T) -> T {
        self.blocks.borrow_mut().push_back(Box::new(block.clone()));
        block
    }

    /// Connects two ports of two blocks in the system.
    ///
    /// Both ports must be of the same message type.
    pub fn connect<T: Message>(
        &self,
        source: &OutputPort<T>,
        target: &InputPort<T>,
    ) -> Result<bool, ()> {
        Ok(self.connections.borrow_mut().insert((source.id, target.id)))
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use crate::blocks::{Const, Drop};
    use crate::runtimes::StdRuntime;
    use crate::transports::MockTransport;
    use crate::{InputPort, OutputPort, Runtime, System};

    #[test]
    fn define_system() -> Result<(), ()> {
        let system = System::new();

        let constant = system.block(Const {
            output: OutputPort::new(&system),
            value: 42,
        });
        let blackhole = system.block(Drop(InputPort::new(&system)));

        system.connect(&constant.output, &blackhole.0)?;

        let transport = MockTransport::new();
        let mut runtime = StdRuntime::new(transport).unwrap();
        let _ = runtime.execute_system(system).unwrap();

        Ok(())
    }
}
