// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

pub trait Subtractable: Message + core::ops::Sub<Output = Self> + num_traits::Zero {}
impl<T: Message + core::ops::Sub<Output = T> + num_traits::Zero> Subtractable for T {}

/// A block that subtracts numbers.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/math/sub.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/math/sub.seq.mmd" framed)]
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
///     let sub = s.sub();
///     let encode = s.encode_with::<f64>(config.encoding);
///     let output = config.write_stdout(s);
///     s.connect(&input.output, &decode.input);
///     s.connect(&decode.output, &sub.input);
///     s.connect(&sub.output, &encode.input);
///     s.connect(&encode.output, &output.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Sub
/// ```
///
#[derive(Block, Clone)]
pub struct Sub<T: Subtractable = f64> {
    /// The input number stream.
    #[input]
    pub input: InputPort<T>,
    /// The output stream of running totals.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Subtractable + 'static> Sub<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Subtractable> Block for Sub<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let result = if let Some(mut result) = self.input.recv()? {
            while let Some(input) = self.input.recv()? {
                result = result - input
            }
            result
        } else {
            T::zero()
        };
        self.output.send(&result)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Subtractable> StdioSystem for Sub<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{IoBlocks, MathBlocks, SysBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|s| {
            let input = s.read_stdin();
            let decode = s.decode_with::<f64>(config.encoding);
            let sub = s.sub();
            let encode = s.encode_with::<f64>(config.encoding);
            let output = config.write_stdout(s);
            s.connect(&input.output, &decode.input);
            s.connect(&decode.output, &sub.input);
            s.connect(&sub.output, &encode.input);
            s.connect(&encode.output, &output.input);
        }))
    }
}

#[cfg(test)]
mod tests {

    use protoflow_core::{runtimes::StdRuntime, transports::MpscTransport, SystemExecution};

    use super::Sub;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Sub::<f32>::new(s.input(), s.output()));
            let _ = s.block(Sub::<f64>::new(s.input(), s.output()));

            let _ = s.block(Sub::<u32>::new(s.input(), s.output()));
            let _ = s.block(Sub::<u64>::new(s.input(), s.output()));

            let _ = s.block(Sub::<i32>::new(s.input(), s.output()));
            let _ = s.block(Sub::<i64>::new(s.input(), s.output()));
        });
    }

    #[test]
    fn run_block() {
        let mut s = System::new(&StdRuntime::new(MpscTransport::new()).unwrap());

        let mut values = s.output();
        let sub = s.block(Sub::<i64>::with_system(&s));
        let result = s.input();

        assert!(s.connect(&values, &sub.input));
        assert!(s.connect(&sub.output, &result));

        let exec = s.execute().unwrap();

        values.send(&3).unwrap();
        values.send(&5).unwrap();
        values.close().unwrap();

        assert_eq!(Ok(Some(-2)), result.recv());

        exec.join().unwrap();
    }
}
