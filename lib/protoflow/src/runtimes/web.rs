// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Duration, Instant},
    BlockError, Port, Runtime, Scheduler, System,
};

pub struct Web {}

#[allow(unused)]
impl Web {
    fn new(_system: System) -> Result<Box<Self>, BlockError> {
        Ok(Box::new(Self {}))
    }
}

impl Runtime for Web {
    fn start(&mut self) -> Result<(), BlockError> {
        // TODO
        Ok(())
    }

    fn stop(&mut self) -> Result<(), BlockError> {
        // TODO
        Ok(())
    }
}

impl Scheduler for Web {
    fn is_alive(&self) -> bool {
        false // TODO
    }

    fn sleep_for(&self, _duration: Duration) -> Result<(), BlockError> {
        todo!() // TODO
    }

    fn sleep_until(&self, _instant: Instant) -> Result<(), BlockError> {
        todo!() // TODO
    }

    fn wait_for(&self, _port: &dyn Port) -> Result<(), BlockError> {
        todo!() // TODO
    }

    fn yield_now(&self) -> Result<(), BlockError> {
        todo!() // TODO
    }
}
