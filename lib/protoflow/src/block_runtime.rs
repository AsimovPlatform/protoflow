// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Duration, Instant},
    BlockError, Port,
};

pub trait BlockRuntime: Send + Sync {
    fn is_alive(&self) -> bool;
    fn sleep_for(&self, duration: Duration) -> Result<(), BlockError>;
    fn sleep_until(&self, instant: Instant) -> Result<(), BlockError>; // TODO
    fn wait_for(&self, port: &dyn Port) -> Result<(), BlockError>;
    fn yield_now(&self) -> Result<(), BlockError>;
}
