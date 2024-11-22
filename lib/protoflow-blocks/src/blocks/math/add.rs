// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{
    Block, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that adds numbers.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/math/add.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/math/add.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```no_run
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     // TODO
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Add
/// ```
///
#[derive(Block, Clone)]
pub struct Add {
    /// The input number stream.
    #[input]
    pub input: InputPort<f64>,
    /// The output stream of running totals.
    #[output]
    pub output: OutputPort<f64>,
}

impl Add {
    pub fn new(input: InputPort<f64>, output: OutputPort<f64>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl Block for Add {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let mut sum = 0.0;
        while let Some(input) = self.input.recv()? {
            sum += input;
            self.output.send(&sum)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Add {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {

        config.reject_any()?;

        Ok(System::build(|_s| { todo!() }))
    }
}

#[cfg(test)]
mod tests {

    use super::Add;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Add::new(s.input(), s.output()));
        });
    }
}
