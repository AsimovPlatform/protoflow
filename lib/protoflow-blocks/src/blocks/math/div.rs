// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    Block, BlockError, BlockResult, BlockRuntime, InputPort, Message, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

pub trait Divisible:
    Message + core::ops::Div<Output = Self> + PartialEq + num_traits::One + num_traits::Zero
{
}
impl<T: Message + core::ops::Div<Output = T> + PartialEq + num_traits::One + num_traits::Zero>
    Divisible for T
{
}

/// A block that divides numbers.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/math/div.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/math/div.seq.mmd" framed)]
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
///     let div = s.div();
///     let encode = s.encode_with::<f64>(config.encoding);
///     let output = config.write_stdout(s);
///     s.connect(&input.output, &decode.input);
///     s.connect(&decode.output, &div.input);
///     s.connect(&div.output, &encode.input);
///     s.connect(&encode.output, &output.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Div
/// ```
///
#[derive(Block, Clone)]
pub struct Div<T: Divisible = f64> {
    /// The input number stream.
    #[input]
    pub input: InputPort<T>,
    /// The output port to send the result on.
    #[output]
    pub output: OutputPort<T>,
}

impl<T: Divisible + 'static> Div<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl<T: Divisible> Block for Div<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let result = if let Some(mut result) = self.input.recv()? {
            while let Some(input) = self.input.recv()? {
                if input == T::zero() {
                    return Err(BlockError::Other("Division by zero".into()));
                }
                result = result / input;
            }
            result
        } else {
            T::one()
        };

        self.output.send(&result)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Div {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{IoBlocks, MathBlocks, SysBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|s| {
            let input = s.read_stdin();
            let decode = s.decode_with::<f64>(config.encoding);
            let div = s.div();
            let encode = s.encode_with::<f64>(config.encoding);
            let output = config.write_stdout(s);
            s.connect(&input.output, &decode.input);
            s.connect(&decode.output, &div.input);
            s.connect(&div.output, &encode.input);
            s.connect(&encode.output, &output.input);
        }))
    }
}

#[cfg(test)]
mod tests {

    use protoflow_core::{runtimes::StdRuntime, transports::MpscTransport, SystemExecution};

    use super::Div;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Div::<f32>::new(s.input(), s.output()));
            let _ = s.block(Div::<f64>::new(s.input(), s.output()));

            let _ = s.block(Div::<u32>::new(s.input(), s.output()));
            let _ = s.block(Div::<u64>::new(s.input(), s.output()));

            let _ = s.block(Div::<i32>::new(s.input(), s.output()));
            let _ = s.block(Div::<i64>::new(s.input(), s.output()));
        });
    }

    #[test]
    fn run_block() {
        let mut s = System::new(&StdRuntime::new(MpscTransport::new()).unwrap());

        let mut values = s.output();
        let div = s.block(Div::<f64>::with_system(&s));
        let result = s.input();

        assert!(s.connect(&values, &div.input));
        assert!(s.connect(&div.output, &result));

        let exec = s.execute().unwrap();

        values.send(&3.0).unwrap();
        values.send(&5.0).unwrap();
        values.close().unwrap();

        assert_eq!(Ok(Some(3.0 / 5.0)), result.recv());

        exec.join().unwrap();
    }
}
