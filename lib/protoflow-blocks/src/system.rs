// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{
    prelude::{Arc, Rc, String, ToString},
    AllBlocks, Const, CoreBlocks, FlowBlocks, IoBlocks, MathBlocks, ReadDir, ReadEnv, ReadFile,
    ReadStdin, SysBlocks, TextBlocks, WriteFile, WriteStderr, WriteStdout,
};
use protoflow_core::{
    Block, BlockResult, InputPort, Message, OutputPort, Process, SystemBuilding, SystemExecution,
};

type Transport = protoflow_core::transports::MpscTransport;
type Runtime = protoflow_core::runtimes::StdRuntime<Transport>;

pub struct System(protoflow_core::System<Transport>);

impl System {
    /// Builds a new system.
    pub fn build<F: FnOnce(&mut System)>(f: F) -> Self {
        let transport = Transport::default();
        let runtime = Runtime::new(transport).unwrap();
        let mut system = System::new(&runtime);
        f(&mut system);
        system
    }

    /// Instantiates a new system.
    pub fn new(runtime: &Arc<Runtime>) -> Self {
        Self(protoflow_core::System::<Transport>::new(runtime))
    }
}

impl AllBlocks for System {}

impl CoreBlocks for System {
    fn const_string(&self, value: impl ToString) -> Const<String> {
        self.0.block(Const::<String>::with_params(
            self.0.output(),
            value.to_string(),
        ))
    }
}

impl FlowBlocks for System {}

impl IoBlocks for System {}

impl MathBlocks for System {}

#[cfg(not(feature = "std"))]
impl SysBlocks for System {}

#[cfg(feature = "std")]
impl SysBlocks for System {
    fn read_dir(&self) -> ReadDir {
        self.0.block(ReadDir::new(self.0.input(), self.0.output()))
    }

    fn read_env(&self) -> ReadEnv {
        self.0.block(ReadEnv::new(self.0.input(), self.0.output()))
    }

    fn read_file(&self) -> ReadFile {
        self.0.block(ReadFile::new(self.0.input(), self.0.output()))
    }

    fn read_stdin(&self) -> ReadStdin {
        self.0.block(ReadStdin::new(self.0.output()))
    }

    fn write_file(&self) -> WriteFile {
        self.0.block(WriteFile::new(self.0.input(), self.0.input()))
    }

    fn write_stderr(&self) -> WriteStderr {
        self.0.block(WriteStderr::new(self.0.input()))
    }

    fn write_stdout(&self) -> WriteStdout {
        self.0.block(WriteStdout::new(self.0.input()))
    }
}

impl TextBlocks for System {}

impl SystemBuilding for System {
    fn input<M: Message + 'static>(&self) -> InputPort<M> {
        self.0.input()
    }

    fn output<M: Message + 'static>(&self) -> OutputPort<M> {
        self.0.output()
    }

    fn block<B: Block + Clone + 'static>(&self, block: B) -> B {
        self.0.block(block)
    }

    fn connect<M: Message>(&self, source: &OutputPort<M>, target: &InputPort<M>) -> bool {
        self.0.connect(source, target)
    }
}

impl SystemExecution for System {
    fn execute(self) -> BlockResult<Rc<dyn Process>> {
        self.0.execute()
    }
}
