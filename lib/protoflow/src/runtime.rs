// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Arc, Rc},
    Block, BlockError, Process, System,
};

pub trait Runtime {
    fn execute<T: Block + 'static>(
        &mut self,
        block: Arc<T>,
    ) -> Result<Arc<dyn Process>, BlockError>;

    fn execute_block(&mut self, block: Arc<dyn Block>) -> Result<Arc<dyn Process>, BlockError>;

    fn execute_system(&mut self, system: Rc<System>) -> Result<Arc<dyn Process>, BlockError>;
}
