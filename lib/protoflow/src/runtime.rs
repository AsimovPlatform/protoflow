// This is free and unencumbered software released into the public domain.

use crate::{prelude::Arc, Block, BlockError, Process, System};

pub trait Runtime {
    fn execute_block(&mut self, block: Arc<dyn Block>) -> Result<Arc<dyn Process>, BlockError>;

    fn execute_system(&mut self, system: System) -> Result<Arc<dyn Process>, BlockError>;
}
