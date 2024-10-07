// This is free and unencumbered software released into the public domain.

use async_trait::async_trait;

use crate::prelude::Box;
use crate::{BlockDescriptor, BlockHooks, BlockResult, BlockRuntime};

/// An async version of a regular block.
#[async_trait]
pub trait AsyncBlock: BlockDescriptor + BlockHooks + Send + Sync {
    /// Prepares this block for execution.
    ///
    /// This is called once before the first call to `execute`.
    /// This is where to open ports and allocate resources.
    fn prepare(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        Ok(())
    }

    /// Executes this block's computation asynchronously.
    async fn execute_async(&mut self, runtime: &dyn BlockRuntime) -> BlockResult;
}
