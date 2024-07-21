// This is free and unencumbered software released into the public domain.

use crate::{Scheduler, System};

pub trait Runtime: Scheduler {
    fn new(system: &System) -> Self;
}
