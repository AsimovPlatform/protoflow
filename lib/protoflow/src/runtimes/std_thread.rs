// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Arc, AtomicBool, AtomicUsize, Duration, Instant, Ordering, Rc, ToString},
    Block, BlockError, BlockRuntime, Port, Process, ProcessID, Runtime, System,
};

#[cfg(feature = "std")]
extern crate std;

#[allow(unused)]
pub struct StdThread {
    is_alive: AtomicBool,
    process_id: AtomicUsize,
}

#[allow(unused)]
impl StdThread {
    pub fn new() -> Result<Arc<Self>, BlockError> {
        Ok(Arc::new(Self {
            is_alive: AtomicBool::new(true),
            process_id: AtomicUsize::new(1),
        }))
    }
}

impl Runtime for Arc<StdThread> {
    fn execute<T: Block + Sized + 'static>(
        &mut self,
        block: Arc<T>,
    ) -> Result<Arc<dyn Process>, BlockError> {
        self.execute_block(block)
    }

    fn execute_block(&mut self, block: Arc<dyn Block>) -> Result<Arc<dyn Process>, BlockError> {
        let block_runtime = Arc::new((*self).clone()) as Arc<dyn BlockRuntime>;
        let block_process = Arc::new(RunningBlock {
            id: self.process_id.fetch_add(1, Ordering::SeqCst),
            runtime: self.clone(),
            handle: Some(
                std::thread::Builder::new()
                    .name(block.name().unwrap_or_else(|| "<unnamed>".to_string()))
                    .spawn(move || {
                        let mut block = block;
                        let block = Arc::get_mut(&mut block).unwrap(); // FIXME
                        std::thread::park();
                        Block::prepare(block, block_runtime.as_ref()).unwrap();
                        Block::execute(block, block_runtime.as_ref()).unwrap();
                        ()
                    })
                    .unwrap(),
            ),
        });
        block_process.handle.as_ref().unwrap().thread().unpark();
        Ok(block_process)
    }

    fn execute_system(&mut self, system: Rc<System>) -> Result<Arc<dyn Process>, BlockError> {
        for block in system.blocks.borrow().iter() {
            let _block_process = self.execute_block(block.clone())?; // FIXME
        }
        todo!() // TODO
    }
}

impl BlockRuntime for Arc<StdThread> {
    fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::SeqCst)
    }

    fn sleep_for(&self, duration: Duration) -> Result<(), BlockError> {
        #[cfg(feature = "std")]
        std::thread::sleep(duration);
        #[cfg(not(feature = "std"))]
        unimplemented!("std::thread::sleep requires the 'std' feature");
        Ok(())
    }

    fn sleep_until(&self, _instant: Instant) -> Result<(), BlockError> {
        todo!() // TODO
    }

    fn wait_for(&self, _port: &dyn Port) -> Result<(), BlockError> {
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
}

#[allow(unused)]
struct RunningBlock {
    id: ProcessID,
    runtime: Arc<StdThread>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Process for RunningBlock {
    fn id(&self) -> ProcessID {
        self.id
    }

    fn is_alive(&self) -> bool {
        self.handle
            .as_ref()
            .map(|handle| !handle.is_finished())
            .unwrap_or(false)
    }

    fn join(&mut self) -> Result<(), ()> {
        self.handle.take().unwrap().join().map_err(|_| ())
    }
}

#[allow(unused)]
struct RunningSystem {
    id: ProcessID,
    runtime: Arc<StdThread>,
}

impl Process for RunningSystem {
    fn id(&self) -> ProcessID {
        self.id
    }

    fn is_alive(&self) -> bool {
        self.runtime.is_alive()
    }

    fn join(&mut self) -> Result<(), ()> {
        Err(()) // TODO
    }
}
