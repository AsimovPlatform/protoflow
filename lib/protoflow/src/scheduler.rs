// This is free and unencumbered software released into the public domain.

use crate::Port;
use std::time::Duration;

pub trait Scheduler {
    fn is_alive(&self) -> bool;
    fn sleep(&self, duration: Duration);
    fn wait_for(&self, port: &dyn Port);
    fn yield_now(&self);
}
