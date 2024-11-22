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

/// A block that multiplies numbers.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/math/mul.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/math/mul.seq.mmd" framed)]
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
/// $ protoflow execute Mul
/// ```
///
#[derive(Block, Clone)]
pub struct Mul {
    /// The input number stream.
    #[input]
    pub input: InputPort<f64>,
    /// The output stream of running totals.
    #[output]
    pub output: OutputPort<f64>,
}

impl Mul {
    pub fn new(input: InputPort<f64>, output: OutputPort<f64>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl Block for Mul {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let mut product = 1.0;
        while let Some(input) = self.input.recv()? {
            product *= input;
            self.output.send(&product)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Mul {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {

        config.reject_any()?;

        Ok(System::build(|_s| { todo!() }))
    }
}

#[cfg(test)]
mod tests {

    use super::Mul;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Mul::new(s.input(), s.output()));
        });
    }
}
