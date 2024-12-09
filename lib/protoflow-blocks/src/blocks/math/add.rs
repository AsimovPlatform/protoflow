// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

pub trait Addable: Message + core::ops::Add<Output = Self> + num_traits::Zero {}
impl<T: Message + core::ops::Add<Output = T> + num_traits::Zero> Addable for T {}

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
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let config = StdioConfig {
///         encoding: Default::default(),
///         params: Default::default(),
///     };
///     let input = s.read_stdin();
///     let decode = s.decode_with::<f64>(config.encoding);
///     let add = s.add();
///     let encode = s.encode_with::<f64>(config.encoding);
///     let output = config.write_stdout(s);
///     s.connect(&input.output, &decode.input);
///     s.connect(&decode.output, &add.input);
///     s.connect(&add.output, &encode.input);
///     s.connect(&encode.output, &output.input);
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
pub struct Add<T: Addable = f64> {
    /// The input number stream.
    #[input]
    pub input: InputPort<T>,
    /// The output stream of running totals.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Addable + 'static> Add<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Addable> Block for Add<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let mut sum = T::zero();
        while let Some(input) = self.input.recv()? {
            sum = sum + input;
        }
        self.output.send(&sum)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Addable> StdioSystem for Add<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{IoBlocks, MathBlocks, SysBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|s| {
            let input = s.read_stdin();
            let decode = s.decode_with::<f64>(config.encoding);
            let add = s.add();
            let encode = s.encode_with::<f64>(config.encoding);
            let output = config.write_stdout(s);
            s.connect(&input.output, &decode.input);
            s.connect(&decode.output, &add.input);
            s.connect(&add.output, &encode.input);
            s.connect(&encode.output, &output.input);
        }))
    }
}

#[cfg(test)]
mod tests {

    use protoflow_core::{runtimes::StdRuntime, transports::MpscTransport};

    use super::Add;
    use crate::{System, SystemBuilding, SystemExecution};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Add::<f32>::new(s.input(), s.output()));
            let _ = s.block(Add::<f64>::new(s.input(), s.output()));

            let _ = s.block(Add::<u32>::new(s.input(), s.output()));
            let _ = s.block(Add::<u64>::new(s.input(), s.output()));

            let _ = s.block(Add::<i32>::new(s.input(), s.output()));
            let _ = s.block(Add::<i64>::new(s.input(), s.output()));
        });
    }

    #[test]
    fn run_block() {
        let mut s = System::new(&StdRuntime::new(MpscTransport::new()).unwrap());

        let mut values = s.output();
        let add = s.block(Add::<u64>::with_system(&s));
        let result = s.input();

        assert!(s.connect(&values, &add.input));
        assert!(s.connect(&add.output, &result));

        let exec = s.execute().unwrap();

        values.send(&3).unwrap();
        values.send(&5).unwrap();
        values.close().unwrap();

        assert_eq!(Ok(Some(8)), result.recv());

        exec.join().unwrap();
    }
}
