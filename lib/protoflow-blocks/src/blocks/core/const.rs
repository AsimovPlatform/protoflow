// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, String},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{Block, BlockResult, BlockRuntime, Message, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block for sending a constant value.
///
/// This block sends a constant value on its output port.
/// It can also be used to send a constant value to multiple blocks.
///
/// The value to send is specified as a parameter, and can be of any
/// type that implements the [`Message`] trait.
///
/// The block waits for the output port to be connected before sending
/// the value, and closes the port after the value is sent.
///
/// The block does not have any input ports nor state.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/core/const.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/core/const.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let const_value = s.const_string("Hello, world!");
///     let line_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&const_value.output, &line_encoder.input);
///     s.connect(&line_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Const value=Hello
/// ```
///
#[derive(Block, Clone)]
pub struct Const<T: Message = String> {
    /// The port to send the value on.
    #[output]
    pub output: OutputPort<T>,

    /// A parameter for the value to send.
    #[parameter]
    pub value: T,
}

impl<T: Message + Default> Const<T> {
    pub fn new(output: OutputPort<T>) -> Self {
        Self::with_params(output, T::default())
    }
}

impl<T: Message> Const<T> {
    pub fn with_params(output: OutputPort<T>, value: T) -> Self {
        Self { output, value }
    }
}

impl<T: Message + 'static> Const<T> {
    pub fn with_system(system: &System, value: T) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.output(), value)
    }
}

impl<T: Message> Block for Const<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        self.output.send(&self.value)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Const<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SystemBuilding};

        config.allow_only(vec!["value"])?;
        let value = config.get_string("value")?;

        Ok(System::build(|s| {
            let const_value = s.const_string(value); // FIXME
            let line_encoder = s.encode_with(config.encoding);
            let stdout = config.write_stdout(s);
            s.connect(&const_value.output, &line_encoder.input);
            s.connect(&line_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Const;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Const::<i32>::with_params(s.output(), 0x00BAB10C));
        });
    }
}
