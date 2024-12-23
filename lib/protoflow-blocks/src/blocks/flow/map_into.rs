// This is free and unencumbered software released into the public domain.

use crate::{prelude::Bytes, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{info, Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block to map a message from one type to another.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/flow/map_into.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/flow/map_into.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     // TODO
/// });
/// # }
/// ```
///
#[derive(Block, Clone)]
pub struct MapInto<Input: Message + Into<Output>, Output: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<Input>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<Output>,
}

impl<Input: Message + Into<Output>, Output: Message> MapInto<Input, Output> {
    pub fn new(input: InputPort<Input>, output: OutputPort<Output>) -> Self {
        Self::with_params(input, output)
    }
}

impl<Input: Message + Into<Output>, Output: Message> MapInto<Input, Output> {
    pub fn with_params(input: InputPort<Input>, output: OutputPort<Output>) -> Self {
        Self { input, output }
    }
}

impl<Input: Message + Into<Output> + 'static, Output: Message + 'static> MapInto<Input, Output> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output())
    }
}

impl<Input: Message + Into<Output>, Output: Message> Block for MapInto<Input, Output> {
    fn execute(&mut self, _: &dyn BlockRuntime) -> BlockResult {
        while let Some(input) = self.input.recv()? {
            info!("Received input: {:?}", input);
            let output: Output = Into::into(input);
            info!("Sending output: {:?}", output);
            self.output.send(&output)?;
        }

        Ok(())
    }
}
#[cfg(feature = "std")]
impl<Input: Message + Into<Output>, Output: Message> StdioSystem for MapInto<Input, Output> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;

        config.reject_any()?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let map = s.block(MapInto::<Bytes, Bytes>::with_system(s));
            s.connect(&stdin.output, &map.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::MapInto;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(MapInto::<u32, u64>::with_params(s.input(), s.output()));
        });
    }
}
