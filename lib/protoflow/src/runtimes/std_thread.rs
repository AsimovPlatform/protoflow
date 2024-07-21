// This is free and unencumbered software released into the public domain.

use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    scheduler::{Duration, Instant},
    Port, Runtime, Scheduler, System,
};

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

    fn sleep_for(&self, duration: Duration) -> Result<(), ()> {
        std::thread::sleep(duration);
        Ok(())
    }

    fn sleep_until(&self, _instant: Instant) -> Result<(), ()> {
        todo!() // TODO
    }

    fn wait_for(&self, port: &dyn Port) -> Result<(), ()> {
        while self.is_alive() && !port.is_connected() {
            self.yield_now()?;
        }
        if self.is_alive() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn yield_now(&self) -> Result<(), ()> {
        std::thread::yield_now();
        Ok(())
    }
}
