// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort, Port};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that counts the number of messages it receives, while optionally
/// passing them through.
#[doc = mermaid!("../../doc/core/count.mmd")]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let message_decoder = s.decode_lines();
///     let counter = s.count::<String>();
///     let count_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &message_decoder.input);
///     s.connect(&message_decoder.output, &counter.input);
///     s.connect(&counter.count, &count_encoder.input);
///     s.connect(&count_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Count
/// ```
///
#[derive(Block, Clone)]
pub struct Count<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The (optional) output target for the stream being passed through.
    #[output]
    pub output: OutputPort<T>,

    /// The output port for the message count.
    #[output]
    pub count: OutputPort<u64>,

    /// The internal state counting the number of messages received.
    #[state]
    counter: u64,
}

impl<T: Message> Count<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>, count: OutputPort<u64>) -> Self {
        Self {
            input,
            output,
            count,
            counter: 0,
        }
    }
}

impl<T: Message> Block for Count<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.counter += 1;

            if self.output.is_connected() {
                self.output.send(&message)?;
            } else {
                drop(message);
            }
        }

        runtime.wait_for(&self.count)?;

        self.count.send(&self.counter)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message + crate::prelude::FromStr + crate::prelude::ToString + 'static> StdioSystem
    for Count<T>
{
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SysBlocks, SystemBuilding};

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let message_decoder = s.decode_with::<T>(config.encoding);
            let counter = s.count::<T>();
            let count_encoder = s.encode_with::<u64>(config.encoding);
            let stdout = s.write_stdout();
            s.connect(&stdin.output, &message_decoder.input);
            s.connect(&message_decoder.output, &counter.input);
            s.connect(&counter.count, &count_encoder.input);
            s.connect(&count_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Count;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Count::<i32>::new(s.input(), s.output(), s.output()));
        });
    }
}
