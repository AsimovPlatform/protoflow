// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{AtomicBool, Duration, Instant, Ordering},
    BlockError, Port, Runtime, Scheduler, System,
};

#[cfg(feature = "std")]
extern crate std;

pub struct StdThread {
    is_alive: AtomicBool,
}

impl Runtime for StdThread {
    fn new(_system: &System) -> Self {
        Self {
            is_alive: AtomicBool::new(true),
        }
    }
}

impl Scheduler for StdThread {
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

    fn wait_for(&self, port: &dyn Port) -> Result<(), BlockError> {
        while self.is_alive() && !port.is_connected() {
            self.yield_now()?;
        }
        if self.is_alive() {
            Ok(())
        } else {
            Err(BlockError::Terminated)
        }
    }

    fn yield_now(&self) -> Result<(), BlockError> {
        #[cfg(feature = "std")]
        std::thread::yield_now();
        #[cfg(not(feature = "std"))]
        unimplemented!("std::thread::yield_now requires the 'std' feature");
        Ok(())
    }
}
