// This is free and unencumbered software released into the public domain.

use crate::{
    scheduler::{Duration, Instant},
    Port, Runtime, Scheduler, System,
};

pub struct Web {}

impl Runtime for Web {
    fn new(_system: &System) -> Self {
        Self {} // TODO
    }
}

impl Scheduler for Web {
    fn is_alive(&self) -> bool {
        false // TODO
    }

    fn sleep_for(&self, _duration: Duration) -> Result<(), ()> {
        todo!() // TODO
    }

    fn sleep_until(&self, _instant: Instant) -> Result<(), ()> {
        todo!() // TODO
    }

    fn wait_for(&self, _port: &dyn Port) -> Result<(), ()> {
        todo!() // TODO
    }

    fn yield_now(&self) -> Result<(), ()> {
        todo!() // TODO
    }
}
