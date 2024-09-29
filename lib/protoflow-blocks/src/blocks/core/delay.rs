// This is free and unencumbered software released into the public domain.

use crate::{types::DelayType, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::Duration, types::Any, Block, BlockResult, BlockRuntime, InputPort, Message,
    OutputPort, Port,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that passes messages through while delaying them by a fixed or
/// random duration.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/core/delay.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/core/delay.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # use std::time::Duration;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let line_decoder = s.decode_lines();
///     let delay = Duration::from_secs(1);
///     let delayer = s.delay_by_fixed::<String>(delay);
///     let line_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &line_decoder.input);
///     s.connect(&line_decoder.output, &delayer.input);
///     s.connect(&delayer.output, &line_encoder.input);
///     s.connect(&line_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Delay fixed=2
/// ```
///
/// ```console
/// $ protoflow execute Delay random=1..5
/// ```
///
#[derive(Block, Clone)]
pub struct Delay<T: Message = Any> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output target for the stream being passed through.
    #[output]
    pub output: OutputPort<T>,

    /// A configuration parameter for which type of delay to add.
    #[parameter]
    pub delay: DelayType,
}

impl<T: Message> Delay<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self::with_params(input, output, None)
    }

    pub fn with_params(
        input: InputPort<T>,
        output: OutputPort<T>,
        delay: Option<DelayType>,
    ) -> Self {
        Self {
            input,
            output,
            delay: delay.unwrap_or_default(),
        }
    }
}

impl<T: Message + 'static> Delay<T> {
    pub fn with_system(system: &System, delay: Option<DelayType>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output(), delay)
    }
}

impl<T: Message> Block for Delay<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            if !self.output.is_connected() {
                drop(message);
                continue;
            }

            let duration = match self.delay {
                DelayType::Fixed(duration) => duration,
                DelayType::Random(ref range) => runtime.random_duration(range.clone()),
            };
            runtime.sleep_for(duration)?;

            self.output.send(&message)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message + crate::prelude::FromStr + crate::prelude::ToString + 'static> StdioSystem
    for Delay<T>
{
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SystemBuilding};

        let fixed_delay = config.get_opt::<f64>("fixed")?;
        // TODO: parse "random" parameter as well.
        let delay = DelayType::Fixed(Duration::from_secs_f64(fixed_delay.unwrap_or(1.)));

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let message_decoder = s.decode_with::<T>(config.encoding);
            let delayer = s.delay_by(delay);
            let message_encoder = s.encode_with::<T>(config.encoding);
            let stdout = config.write_stdout(s);
            s.connect(&stdin.output, &message_decoder.input);
            s.connect(&message_decoder.output, &delayer.input);
            s.connect(&delayer.output, &message_encoder.input);
            s.connect(&message_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Delay;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Delay::<i32>::with_system(s, None));
        });
    }
}
