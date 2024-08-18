// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{prelude::String, Block, BlockResult, BlockRuntime, Message, OutputPort};
use protoflow_derive::Block;

/// A block for sending a constant value.
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

impl<T: Message> Block for Const<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        self.output.send(&self.value)?;
        self.output.close()?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Const<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SysBlocks, SystemBuilding};

        let Some(value) = config.params.get("value").map(String::clone) else {
            return Err(StdioError::MissingParameter("value"))?;
        };

        Ok(System::build(|s| {
            let const_source = s.const_string(value); // FIXME
            let line_encoder = s.encode_with(config.encoding);
            let stdout = s.write_stdout();
            s.connect(&const_source.output, &line_encoder.input);
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
