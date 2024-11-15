// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{String, Vec, vec},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{
    Block, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that concat strings.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/text/concat_strings.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/text/concat_strings.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let joiner = ",";
///     let stdin = s.read_stdin();
///     let line_decoder = s.decode_with(config.encoding);
///     let concat_strings = s.concat_strings_by(&joiner);
///     let line_encoder = s.encode_with(config.encoding);
///     let stdout = config.write_stdout(s);
///     s.connect(&stdin.output, &line_decoder.input);
///     s.connect(&line_decoder.output, &concat_strings.input);
///     s.connect(&concat_strings.output, &line_encoder.input);
///     s.connect(&line_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute ConcatStrings joiner=","
/// ```
///
#[derive(Block, Clone)]
pub struct ConcatStrings {
    /// The input message string stream.
    #[input]
    pub input: InputPort<String>,
    /// The output string.
    #[output]
    pub output: OutputPort<String>,
    /// A parameter placed between each input parameter
    #[parameter]
    pub joiner: String
}

impl ConcatStrings {
    pub fn new(input: InputPort<String>, output: OutputPort<String>) -> Self {
        Self::with_params(input, output, None)
    }

    pub fn with_system(system: &System, joiner: Option<String>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output(), joiner)
    }

    pub fn with_params(input: InputPort<String>, output: OutputPort<String>, joiner: Option<String>) -> Self {
        Self {
            input,
            output,
            joiner: joiner.unwrap_or_default()
        }
    }
}

impl Block for ConcatStrings {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        let mut inputs = Vec::new();
        while let Some(input) = self.input.recv()? {
            if !input.is_empty() { inputs.push(input); }
        }

        self.output.send(&inputs.join(&self.joiner))?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for ConcatStrings {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{TextBlocks, IoBlocks, SysBlocks, SystemBuilding};

        config.allow_only(vec!["joiner"])?;
        let joiner = config.get_string("joiner")?;

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let line_decoder = s.decode_with(config.encoding);
            let concat_strings = s.concat_strings_by(&joiner);
            let line_encoder = s.encode_with(config.encoding);
            let stdout = config.write_stdout(s);
            s.connect(&stdin.output, &line_decoder.input);
            s.connect(&line_decoder.output, &concat_strings.input);
            s.connect(&concat_strings.output, &line_encoder.input);
            s.connect(&line_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(ConcatStrings::new(s.input(), s.output()));
        });
    }
}
