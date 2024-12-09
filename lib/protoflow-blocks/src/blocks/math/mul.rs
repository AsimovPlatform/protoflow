// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

pub trait Multipliable: Message + core::ops::Mul<Output = Self> + num_traits::One {}
impl<T: Message + core::ops::Mul<Output = T> + num_traits::One> Multipliable for T {}

/// A block that multiplies numbers.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/math/mul.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/math/mul.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```no_run
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let config = StdioConfig {
///         encoding: Default::default(),
///         params: Default::default(),
///     };
///     let input = s.read_stdin();
///     let decode = s.decode_with::<f64>(config.encoding);
///     let mul = s.mul();
///     let encode = s.encode_with::<f64>(config.encoding);
///     let output = config.write_stdout(s);
///     s.connect(&input.output, &decode.input);
///     s.connect(&decode.output, &mul.input);
///     s.connect(&mul.output, &encode.input);
///     s.connect(&encode.output, &output.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Mul
/// ```
///
#[derive(Block, Clone)]
pub struct Mul<T: Multipliable = f64> {
    /// The input number stream.
    #[input]
    pub input: InputPort<T>,
    /// The output port to send the result on.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Multipliable + 'static> Mul<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Multipliable> Block for Mul<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let mut product = T::one();
        while let Some(input) = self.input.recv()? {
            product = product * input;
        }

        self.output.send(&product)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Multipliable> StdioSystem for Mul<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{IoBlocks, MathBlocks, SysBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|s| {
            let input = s.read_stdin();
            let decode = s.decode_with::<f64>(config.encoding);
            let mul = s.mul();
            let encode = s.encode_with::<f64>(config.encoding);
            let output = config.write_stdout(s);
            s.connect(&input.output, &decode.input);
            s.connect(&decode.output, &mul.input);
            s.connect(&mul.output, &encode.input);
            s.connect(&encode.output, &output.input);
        }))
    }
}

#[cfg(test)]
mod tests {

    use protoflow_core::{runtimes::StdRuntime, transports::MpscTransport};

    use super::Mul;
    use crate::{System, SystemBuilding, SystemExecution};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Mul::<f32>::new(s.input(), s.output()));
            let _ = s.block(Mul::<f64>::new(s.input(), s.output()));

            let _ = s.block(Mul::<u32>::new(s.input(), s.output()));
            let _ = s.block(Mul::<u64>::new(s.input(), s.output()));

            let _ = s.block(Mul::<i32>::new(s.input(), s.output()));
            let _ = s.block(Mul::<i64>::new(s.input(), s.output()));
        });
    }

    #[test]
    fn run_block() {
        let mut s = System::new(&StdRuntime::new(MpscTransport::new()).unwrap());

        let mut values = s.output();
        let mul = s.block(Mul::<u64>::with_system(&s));
        let result = s.input();

        assert!(s.connect(&values, &mul.input));
        assert!(s.connect(&mul.output, &result));

        let exec = s.execute().unwrap();

        values.send(&3).unwrap();
        values.send(&5).unwrap();
        values.close().unwrap();

        assert_eq!(Ok(Some(15)), result.recv());

        exec.join().unwrap();
    }
}
