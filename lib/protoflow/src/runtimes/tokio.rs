// This is free and unencumbered software released into the public domain.

use crate::{Port, Runtime, Scheduler, System};

pub struct Tokio {}

impl Runtime for Tokio {
    fn new(_system: &System) -> Self {
        Self {} // TODO
    }
}

impl Scheduler for Tokio {
    fn is_alive(&self) -> bool {
        true // TODO
    }

    fn sleep(&self, _duration: std::time::Duration) {
        todo!() // TODO
    }

    fn wait_for(&self, _port: &dyn Port) {
        todo!() // TODO
    }

    fn yield_now(&self) {
        todo!() // TODO
    }
}
