// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Arc, Box, PhantomData, Rc, RefCell, VecDeque},
    runtimes::StdRuntime,
    transports::MockTransport,
    Block, BlockResult, InputPort, Message, OutputPort, Process, Runtime, Transport,
};

/// A machine-readable identifier for a block in a system.
///
/// Only valid within the scope of that system.
pub type BlockID = usize;

/// A system is a collection of blocks that are connected together.
pub struct System<X: Transport + Default + 'static = MockTransport> {
    _phantom: PhantomData<X>,
    pub(crate) runtime: Arc<StdRuntime<X>>,

    /// The registered blocks in the system.
    pub(crate) blocks: RefCell<VecDeque<Box<dyn Block>>>,
}

pub type Subsystem<X> = System<X>;

impl<X: Transport + Default + 'static> System<X> {
    /// Builds a new system.
    pub fn build<F: FnOnce(&mut System<X>)>(f: F) -> Self {
        let transport = X::default();
        let runtime = StdRuntime::new(transport).unwrap();
        let mut system = System::new(&runtime);
        f(&mut system);
        system
    }

    /// Instantiates a new system.
    pub fn new(runtime: &Arc<StdRuntime<X>>) -> Self {
        Self {
            _phantom: PhantomData,
            runtime: runtime.clone(),
            blocks: RefCell::new(VecDeque::new()),
        }
    }

    pub fn execute(self) -> BlockResult<Rc<dyn Process>> {
        let mut runtime = self.runtime.clone();
        runtime.execute(self)
    }

    /// Creates a new input port.
    pub fn input<M: Message + 'static>(&self) -> InputPort<M> {
        InputPort::new(self)
    }

    /// Creates a new output port.
    pub fn output<M: Message + Clone + 'static>(&self) -> OutputPort<M> {
        OutputPort::new(self)
    }

    /// Instantiates a block in the system.
    pub fn block<B: Block + Clone + 'static>(&self, block: B) -> B {
        self.blocks.borrow_mut().push_back(Box::new(block.clone()));
        block
    }

    /// Connects two ports of two blocks in the system.
    ///
    /// Both ports must be of the same message type.
    pub fn connect<M: Message>(&self, source: &OutputPort<M>, target: &InputPort<M>) -> bool {
        let runtime = self.runtime.as_ref();
        let transport = runtime.transport.as_ref();
        transport.connect(source.id, target.id).unwrap()
    }
}
