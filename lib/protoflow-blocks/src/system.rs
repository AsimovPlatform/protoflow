// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

use crate::{
    prelude::{fmt, Arc, Box, FromStr, Rc, String, ToString},
    types::{DelayType, Encoding},
    AllBlocks, Buffer, Const, CoreBlocks, Count, Decode, Delay, Drop, Encode, EncodeHex,
    EncodeJson, FlowBlocks, HashBlocks, IoBlocks, MathBlocks, Random, ReadDir, ReadEnv, ReadFile,
    ReadStdin, SysBlocks, TextBlocks, WriteFile, WriteStderr, WriteStdout,
};
use protoflow_core::{
    Block, BlockID, BlockResult, InputPort, Message, OutputPort, PortID, PortResult, Process,
    SystemBuilding, SystemExecution,
};

#[cfg(feature = "hash")]
use crate::{types::HashAlgorithm, Hash};

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

    #[doc(hidden)]
    pub fn add_block(&mut self, block: Box<dyn Block>) -> BlockID {
        self.0.add_block(block)
    }

    #[doc(hidden)]
    pub fn get_block(&self, block_id: BlockID) -> Option<&Box<dyn Block>> {
        self.0.get_block(block_id)
    }

    #[doc(hidden)]
    pub fn connect_by_id(&mut self, source_id: PortID, target_id: PortID) -> PortResult<bool> {
        self.0.connect_by_id(source_id, target_id)
    }
}

impl fmt::Debug for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl SystemExecution for System {
    fn execute(self) -> BlockResult<Rc<dyn Process>> {
        self.0.execute()
    }
}

impl SystemBuilding for System {
    fn input<M: Message + 'static>(&self) -> InputPort<M> {
        self.0.input()
    }

    fn output<M: Message + 'static>(&self) -> OutputPort<M> {
        self.0.output()
    }

    fn block<B: Block + Clone + 'static>(&mut self, block: B) -> B {
        self.0.block(block)
    }

    fn connect<M: Message>(&mut self, source: &OutputPort<M>, target: &InputPort<M>) -> bool {
        self.0.connect(source, target)
    }
}

impl AllBlocks for System {}

impl CoreBlocks for System {
    fn buffer<T: Message + Into<T> + 'static>(&mut self) -> Buffer<T> {
        self.0.block(Buffer::<T>::with_system(self))
    }

    fn const_string(&mut self, value: impl ToString) -> Const<String> {
        self.0
            .block(Const::<String>::with_system(self, value.to_string()))
    }

    fn count<T: Message + 'static>(&mut self) -> Count<T> {
        self.0.block(Count::<T>::with_system(self))
    }

    fn delay<T: Message + 'static>(&mut self) -> Delay<T> {
        self.0.block(Delay::<T>::with_system(self, None))
    }

    fn delay_by<T: Message + 'static>(&mut self, delay: DelayType) -> Delay<T> {
        self.0.block(Delay::<T>::with_system(self, Some(delay)))
    }

    fn drop<T: Message + 'static>(&mut self) -> Drop<T> {
        self.0.block(Drop::<T>::with_system(self))
    }

    fn random<T: Message + 'static>(&mut self) -> Random<T> {
        self.0.block(Random::<T>::with_system(self, None))
    }

    fn random_seeded<T: Message + 'static>(&mut self, seed: Option<u64>) -> Random<T> {
        self.0.block(Random::<T>::with_system(self, seed))
    }
}

impl FlowBlocks for System {}

#[cfg(not(feature = "hash"))]
impl HashBlocks for System {}

#[cfg(feature = "hash")]
impl HashBlocks for System {
    fn hash_blake3(&mut self) -> Hash {
        self.0
            .block(Hash::with_system(self, Some(HashAlgorithm::BLAKE3)))
    }
}

impl IoBlocks for System {
    fn decode<T: Message + FromStr + 'static>(&mut self) -> Decode<T> {
        self.0.block(Decode::<T>::with_system(self, None))
    }

    fn decode_with<T: Message + FromStr + 'static>(&mut self, encoding: Encoding) -> Decode<T> {
        self.0.block(Decode::<T>::with_system(self, Some(encoding)))
    }

    fn encode<T: Message + ToString + 'static>(&mut self) -> Encode<T> {
        self.0.block(Encode::<T>::with_system(self, None))
    }

    fn encode_with<T: Message + ToString + 'static>(&mut self, encoding: Encoding) -> Encode<T> {
        self.0.block(Encode::<T>::with_system(self, Some(encoding)))
    }

    fn encode_hex(&mut self) -> EncodeHex {
        self.0.block(EncodeHex::with_system(self))
    }

    fn encode_json(&mut self) -> EncodeJson {
        self.0.block(EncodeJson::with_system(self))
    }
}

impl MathBlocks for System {}

#[cfg(not(feature = "std"))]
impl SysBlocks for System {}

#[cfg(feature = "std")]
impl SysBlocks for System {
    fn read_dir(&mut self) -> ReadDir {
        self.0.block(ReadDir::with_system(self))
    }

    fn read_env(&mut self) -> ReadEnv {
        self.0.block(ReadEnv::with_system(self))
    }

    fn read_file(&mut self) -> ReadFile {
        self.0.block(ReadFile::with_system(self))
    }

    fn read_stdin(&mut self) -> ReadStdin {
        self.0.block(ReadStdin::with_system(self, None))
    }

    fn write_file(&mut self) -> WriteFile {
        self.0.block(WriteFile::with_system(self, None))
    }

    fn write_stderr(&mut self) -> WriteStderr {
        self.0.block(WriteStderr::with_system(self))
    }

    fn write_stdout(&mut self) -> WriteStdout {
        self.0.block(WriteStdout::with_system(self))
    }
}

impl TextBlocks for System {}
