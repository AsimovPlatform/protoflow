// This is free and unencumbered software released into the public domain.

use core::cmp::Ordering;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    error, info, prelude::Vec, types::Any, Block, BlockResult, BlockRuntime, InputPort, Message,
    OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// Sorts a single input message stream in ascending order.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/flow/sort.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/flow/sort.seq.mmd" framed)]
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
///     let sort = s.sort();
///     s.connect(&stdin.output, &sort.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Sort
/// ```
///
#[derive(Block, Clone)]
pub struct Sort<T: Message = Any> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<T>,

    /// The internal state storing the messages received.
    #[state]
    messages: Vec<T>,
}

impl<T: Message> Sort<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self {
            input,
            output,
            messages: Vec::new(),
        }
    }

    pub fn messages(&self) -> &Vec<T> {
        &self.messages
    }
}

impl<T: Message + 'static> Sort<T> {
    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Message + PartialOrd> Block for Sort<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.messages.push(message);
        }

        info!("Sorting messages");
        self.messages.sort_by(|x, y| {
            if let Some(ordering) = x.partial_cmp(y) {
                ordering
            } else {
                error!("Incomparable values: {:?} and {:?}", x, y);
                Ordering::Equal
            }
        });

        for message in self.messages.drain(..) {
            self.output.send(&message)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message> StdioSystem for Sort<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::SystemBuilding;

        config.reject_any()?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let sort = s.block(Sort::new(s.input(), s.output()));
            s.connect(&stdin.output, &sort.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Sort;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Sort::<i32>::new(s.input(), s.output()));
        });
    }

    #[test]
    #[ignore = "requires stdin"]
    fn run_sort_stdout() {
        use super::*;
        use crate::SysBlocks;
        use protoflow_core::{error, SystemBuilding};

        if let Err(e) = System::run(|s| {
            let stdin = s.read_stdin();
            let sort = s.block(Sort::new(s.input(), s.output()));
            s.connect(&stdin.output, &sort.input);

            let stdout_1 = s.write_stdout();
            s.connect(&sort.output, &stdout_1.input);
        }) {
            error!("{}", e)
        }
    }
}
