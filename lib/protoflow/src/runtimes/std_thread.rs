// This is free and unencumbered software released into the public domain.

use crate::{Port, Runtime, Scheduler, System};

pub struct StdThread {}

impl Runtime for StdThread {
    fn new(_system: &System) -> Self {
        Self {} // TODO
    }
}

impl Scheduler for StdThread {
    fn is_alive(&self) -> bool {
        true // TODO
    }

    fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }

    fn wait_for(&self, port: &dyn Port) {
        while self.is_alive() && !port.is_connected() {
            self.yield_now();
        }
    }

    fn yield_now(&self) {
        std::thread::yield_now();
    }
}
