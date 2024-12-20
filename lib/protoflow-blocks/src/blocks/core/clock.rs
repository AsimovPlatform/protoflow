// This is free and unencumbered software released into the public domain.

use crate::{prelude::String, types::DelayType, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockResult, BlockRuntime, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that periodically sends current timestamp.
///
/// This block sends current timestamp on its output port, with interval specified by the parameter.
///
/// The timestamp is a Unix UTC timestamp in microseconds passed as a [`i64`] value.
///
/// The block waits for the output port to be connected before sending the value.
///
/// The block does not have any input ports nor state.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/core/clock.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/core/clock.seq.mmd" framed)]
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
///     let stdin = s.clock_fixed(Duration::from_secs(1));
///     let encode_lines = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &encode_lines.input);
///     s.connect(&encode_lines.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Clock fixed=2
/// ```
///
/// ```console
/// $ protoflow execute Clock fixed=0.5
/// ```
///
/// ```console
/// $ protoflow execute Clock random=1..5
/// ```
///
/// ```console
/// $ protoflow execute Clock random=0.5..1.5
/// ```
///
#[derive(Block, Clone)]
pub struct Clock {
    /// The port to send the timestamp on.
    #[output]
    pub output: OutputPort<i64>,

    /// A delay between outputs.
    #[parameter]
    pub delay: DelayType,
}

impl Clock {
    pub fn new(output: OutputPort<i64>, delay: DelayType) -> Self {
        Self::with_params(output, delay)
    }
}

impl Clock {
    pub fn with_params(output: OutputPort<i64>, delay: DelayType) -> Self {
        Self { output, delay }
    }
}

impl Clock {
    pub fn with_system(system: &System, delay: DelayType) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.output(), delay)
    }
}

impl Block for Clock {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.output)?;

        loop {
            let now = chrono::Utc::now().timestamp_micros();
            self.output.send(&now)?;

            let duration = match self.delay {
                DelayType::Fixed(duration) => duration,
                DelayType::Random(ref range) => runtime.random_duration(range.clone()),
            };
            runtime.sleep_for(duration)?;
        }
    }
}

fn parse_range(range_str: &String) -> Option<(f64, f64)> {
    if let Some(range_str) = range_str.split_once("..") {
        match (range_str.0.parse::<f64>(), range_str.1.parse::<f64>()) {
            (Ok(range0), Ok(range1)) => Some((range0, range1)),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Clock {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{prelude::Duration, CoreBlocks, IoBlocks, SysBlocks, SystemBuilding};

        let delay_type = if let Some(delay) = config.get_opt::<f64>("fixed")? {
            DelayType::Fixed(Duration::from_secs_f64(delay))
        } else if let Some(delay) = config.get_opt::<String>("random")? {
            if let Some(range) = parse_range(&delay) {
                DelayType::Random(
                    Duration::from_secs_f64(range.0)..Duration::from_secs_f64(range.1),
                )
            } else {
                return Err(StdioError::InvalidParameter("random"));
            }
        } else {
            return Err(StdioError::MissingParameter("fixed or random"));
        };

        Ok(System::build(|s| {
            let stdin = s.clock(delay_type);
            let encode_lines = s.encode_lines();
            let stdout = s.write_stdout();
            s.connect(&stdin.output, &encode_lines.input);
            s.connect(&encode_lines.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Clock;
    use crate::{prelude::Duration, DelayType, System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Clock::with_params(
                s.output(),
                DelayType::Fixed(Duration::from_secs(1)),
            ));
        });
    }
}
