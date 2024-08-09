// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{
        Arc, AtomicBool, AtomicUsize, Box, Duration, Instant, Ordering, Range, Rc, RefCell,
        ToString, Vec,
    },
    transport::Transport,
    transports::MockTransport,
    Block, BlockError, BlockResult, BlockRuntime, Port, Process, ProcessID, Runtime, System,
};

#[cfg(feature = "std")]
extern crate std;

#[allow(unused)]
pub struct StdRuntime<T: Transport = MockTransport> {
    pub(crate) transport: Arc<T>,
    is_alive: AtomicBool,
    process_id: AtomicUsize,
}

#[allow(unused)]
impl<T: Transport> StdRuntime<T> {
    pub fn new(transport: T) -> Result<Arc<Self>, BlockError> {
        Ok(Arc::new(Self {
            transport: Arc::new(transport),
            is_alive: AtomicBool::new(true),
            process_id: AtomicUsize::new(1),
        }))
    }
}

impl<T: Transport + 'static> Runtime for Arc<StdRuntime<T>> {
    fn execute_block(&mut self, block: Box<dyn Block>) -> BlockResult<Rc<dyn Process>> {
        let block_runtime = Arc::new((*self).clone()) as Arc<dyn BlockRuntime>;
        let block_process = Rc::new(RunningBlock {
            id: self.process_id.fetch_add(1, Ordering::SeqCst),
            runtime: self.clone(),
            handle: RefCell::new(Some(
                std::thread::Builder::new()
                    .name(block.name().unwrap_or_else(|| "<unnamed>".to_string()))
                    .spawn(move || {
                        let mut block = block;
                        std::thread::park();
                        Block::prepare(block.as_mut(), block_runtime.as_ref())
                            .and_then(|_| Block::execute(block.as_mut(), block_runtime.as_ref()))
                    })
                    .unwrap(),
            )),
        });
        block_process
            .handle
            .borrow()
            .as_ref()
            .unwrap()
            .thread()
            .unpark();
        Ok(block_process)
    }

    fn execute<X: Transport + Default>(
        &mut self,
        system: System<X>,
    ) -> BlockResult<Rc<dyn Process>> {
        let mut system_process = RunningSystem {
            id: self.process_id.fetch_add(1, Ordering::SeqCst),
            runtime: self.clone(),
            transport: self.transport.clone(),
            blocks: Vec::new(),
        };

        while let Some(block) = system.blocks.borrow_mut().pop_front() {
            system_process.blocks.push(self.execute_block(block)?);
        }

        Ok(Rc::new(system_process))
    }
}

impl<T: Transport> BlockRuntime for Arc<StdRuntime<T>> {
    fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::SeqCst)
    }

    fn sleep_for(&self, duration: Duration) -> BlockResult {
        #[cfg(feature = "std")]
        std::thread::sleep(duration);
        #[cfg(not(feature = "std"))]
        unimplemented!("std::thread::sleep requires the 'std' feature");
        Ok(())
    }

    fn sleep_until(&self, _instant: Instant) -> BlockResult {
        todo!() // TODO
    }

    fn wait_for(&self, _port: &dyn Port) -> BlockResult {
        // while self.is_alive() && !port.is_connected() {
        //     self.yield_now()?;
        // }
        // if self.is_alive() {
        //     Ok(())
        // } else {
        //     Err(BlockError::Terminated)
        // }
        Ok(()) // TODO
    }

    fn yield_now(&self) -> Result<(), BlockError> {
        #[cfg(feature = "std")]
        std::thread::yield_now();
        #[cfg(not(feature = "std"))]
        unimplemented!("std::thread::yield_now requires the 'std' feature");
        Ok(())
    }

    fn random_duration(&self, range: Range<Duration>) -> Duration {
        #[cfg(all(feature = "std", feature = "rand"))]
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let low = range.start.as_nanos() as u64;
            let high = range.end.as_nanos() as u64;
            Duration::from_nanos(rng.gen_range(low..high))
        }
        #[cfg(not(all(feature = "std", feature = "rand")))]
        let mut _rng = todo!();
    }
}

#[allow(unused)]
struct RunningBlock<T: Transport> {
    id: ProcessID,
    runtime: Arc<StdRuntime<T>>,
    handle: RefCell<Option<std::thread::JoinHandle<BlockResult>>>,
}

#[allow(unused)]
impl<T: Transport> RunningBlock<T> {
    //fn thread(&self) -> Option<&std::thread::Thread> {
    //    self.handle.borrow().as_ref().map(|handle| handle.thread())
    //}
}

impl<T: Transport> Process for RunningBlock<T> {
    fn id(&self) -> ProcessID {
        self.id
    }

    fn is_alive(&self) -> bool {
        self.handle
            .borrow()
            .as_ref()
            .map(|handle| !handle.is_finished())
            .unwrap_or(false)
    }

    fn join(&self) -> BlockResult {
        let handle = self.handle.take().unwrap();
        handle
            .join()
            .map_err(<Box<dyn core::any::Any + Send>>::from)?
    }
}

#[allow(unused)]
struct RunningSystem<T: Transport> {
    id: ProcessID,
    runtime: Arc<StdRuntime<T>>,
    transport: Arc<T>,
    blocks: Vec<Rc<dyn Process>>,
}

impl<T: Transport> Process for RunningSystem<T> {
    fn id(&self) -> ProcessID {
        self.id
    }

    fn is_alive(&self) -> bool {
        self.blocks.iter().any(|block| block.is_alive())
    }

    fn join(&self) -> BlockResult {
        for block in self.blocks.iter() {
            block.join()?;
        }
        Ok(())
    }
}
