// This is free and unencumbered software released into the public domain.

use super::{prelude::Box, System};
use protoflow_core::Block;

/// A trait for instantiating a block in a given system.
pub trait BlockInstantiation {
    fn instantiate(&self, _system: &mut System) -> Box<dyn Block> {
        unimplemented!()
    }
}
