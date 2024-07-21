// This is free and unencumbered software released into the public domain.

use crate::Port;

pub use std::time::Duration;

pub type Instant = Duration;

pub trait Scheduler {
    fn is_alive(&self) -> bool;
    fn sleep_for(&self, duration: Duration) -> Result<(), ()>;
    fn sleep_until(&self, instant: Instant) -> Result<(), ()>; // TODO
    fn wait_for(&self, port: &dyn Port) -> Result<(), ()>;
    fn yield_now(&self) -> Result<(), ()>;
}
