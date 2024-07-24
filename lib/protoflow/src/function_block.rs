// This is free and unencumbered software released into the public domain.

use crate::{prelude::Result, BlockError};

pub trait FunctionBlock<I, O> {
    fn compute(&self, input: I) -> Result<O, BlockError>;
}
