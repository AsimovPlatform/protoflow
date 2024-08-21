// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{
    prelude::{Arc, FromStr, Rc, String, ToString},
    AllBlocks, Buffer, Const, CoreBlocks, Count, Decode, Delay, DelayType, Drop, Encode, Encoding,
    FlowBlocks, Hash, HashAlgorithm, HashBlocks, IoBlocks, MathBlocks, Random, ReadDir, ReadEnv,
    ReadFile, ReadStdin, SysBlocks, TextBlocks, WriteFile, WriteStderr, WriteStdout,
};
use protoflow_core::{
    Block, BlockResult, InputPort, Message, OutputPort, Process, SystemBuilding, SystemExecution,
};

type Transport = protoflow_core::transports::MpscTransport;
type Runtime = protoflow_core::runtimes::StdRuntime<Transport>;

pub struct System(protoflow_core::System<Transport>);

impl System {
    /// Builds and executes a system, blocking until completion.
    pub fn run<F: FnOnce(&mut System)>(f: F) -> BlockResult {
        Self::build(f).execute()?.join()
    }

    /// Builds and executes a system, returning immediately.
    pub fn spawn<F: FnOnce(&mut System)>(f: F) -> BlockResult<Rc<dyn Process>> {
        Self::build(f).execute()
    }

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
    fn buffer<T: Message + Into<T> + 'static>(&self) -> Buffer<T> {
        self.0.block(Buffer::<T>::new(self.0.input()))
    }

    fn const_string(&self, value: impl ToString) -> Const<String> {
        self.0.block(Const::<String>::with_params(
            self.0.output(),
            value.to_string(),
        ))
    }

    fn count<T: Message + 'static>(&self) -> Count<T> {
        self.0.block(Count::<T>::new(
            self.0.input(),
            self.0.output(),
            self.0.output(),
        ))
    }

    fn delay<T: Message + 'static>(&self) -> Delay<T> {
        self.0
            .block(Delay::<T>::new(self.0.input(), self.0.output()))
    }

    fn delay_by<T: Message + 'static>(&self, delay: DelayType) -> Delay<T> {
        self.0.block(Delay::<T>::with_params(
            self.0.input(),
            self.0.output(),
            delay,
        ))
    }

    fn drop<T: Message + 'static>(&self) -> Drop<T> {
        self.0.block(Drop::<T>::new(self.0.input()))
    }

    fn random<T: Message + 'static>(&self) -> Random<T> {
        self.0.block(Random::<T>::new(self.0.output()))
    }

    fn random_seeded<T: Message + 'static>(&self, seed: Option<u64>) -> Random<T> {
        self.0
            .block(Random::<T>::with_params(self.0.output(), seed))
    }
}

impl FlowBlocks for System {}

#[cfg(not(feature = "hash"))]
impl HashBlocks for System {}

#[cfg(feature = "hash")]
impl HashBlocks for System {
    fn hash_blake3(&self) -> Hash {
        self.0.block(Hash::with_params(
            self.0.input(),
            self.0.output(),
            self.0.output(),
            HashAlgorithm::BLAKE3,
        ))
    }
}

impl IoBlocks for System {
    fn decode<T: Message + FromStr + 'static>(&self) -> Decode<T> {
        self.0
            .block(Decode::<T>::new(self.0.input(), self.0.output()))
    }

    fn decode_with<T: Message + FromStr + 'static>(&self, encoding: Encoding) -> Decode<T> {
        self.0.block(Decode::<T>::with_params(
            self.0.input(),
            self.0.output(),
            encoding,
        ))
    }

    fn encode<T: Message + ToString + 'static>(&self) -> Encode<T> {
        self.0
            .block(Encode::<T>::new(self.0.input(), self.0.output()))
    }

    fn encode_with<T: Message + ToString + 'static>(&self, encoding: Encoding) -> Encode<T> {
        self.0.block(Encode::<T>::with_params(
            self.0.input(),
            self.0.output(),
            encoding,
        ))
    }
}

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
