// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{FromStr, String},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that reads the value of an environment variable.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/sys/read_env.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/sys/read_env.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let name_param = s.const_string("TERM");
///     let env_reader = s.read_env();
///     let line_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&name_param.output, &env_reader.name);
///     s.connect(&env_reader.output, &line_encoder.input);
///     s.connect(&line_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute ReadEnv name=TERM
/// ```
///
#[derive(Block, Clone)]
pub struct ReadEnv<T: Message + FromStr = String> {
    /// The name of the environment variable to read.
    #[input]
    pub name: InputPort<String>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Message + FromStr> ReadEnv<T> {
    pub fn new(name: InputPort<String>, output: OutputPort<T>) -> Self {
        Self { name, output }
    }
}

impl<T: Message + FromStr + 'static> ReadEnv<T> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Message + FromStr> Block for ReadEnv<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.name)?;
        let name = self.name.recv()?.unwrap();
        //self.name.close()?; // FIXME

        let value = std::env::var(&name).unwrap_or_default();
        let value = T::from_str(&value).unwrap_or_default();
        self.output.send(&value)?;

        self.output.close()?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message + FromStr> StdioSystem for ReadEnv<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SysBlocks, SystemBuilding};

        let name = config.get_string("name")?;

        Ok(System::build(|s| {
            let name_param = s.const_string(name);
            let env_reader = s.read_env();
            let line_encoder = s.encode_with(config.encoding);
            let stdout = config.write_stdout(s);
            s.connect(&name_param.output, &env_reader.name);
            s.connect(&env_reader.output, &line_encoder.input);
            s.connect(&line_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::ReadEnv;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(ReadEnv::<i32>::new(s.input(), s.output()));
        });
    }
}
