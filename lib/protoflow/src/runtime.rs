// This is free and unencumbered software released into the public domain.

use crate::{BlockError, Scheduler};

pub trait Runtime: Scheduler {
    fn start(&mut self) -> Result<(), BlockError>;
    fn stop(&mut self) -> Result<(), BlockError>;
}
