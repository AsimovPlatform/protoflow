// This is free and unencumbered software released into the public domain.

use crate as protoflow;

use protoflow::derive::Block;
use protoflow::{
    prelude::{Duration, Range},
    Block, BlockError, InputPort, Message, OutputPort, Port, Runtime,
};

#[cfg(feature = "rand")]
use rand::Rng;

/// A block that passes messages through while delaying them by a fixed or
/// random duration.
#[derive(Block)]
pub struct Delay<T: Message> {
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

/// The type of delay (fixed or random) to apply to message relay.
pub enum DelayType {
    Fixed(Duration),
    Random(Range<Duration>),
}

impl<T: Message> Block for Delay<T> {
    fn execute(&mut self, runtime: &dyn Runtime) -> Result<(), BlockError> {
        while let Some(message) = self.input.receive()? {
            if !self.output.is_connected() {
                drop(message);
                continue;
            }

            let duration = match self.delay {
                DelayType::Fixed(duration) => duration,
                #[allow(unused_variables)]
                DelayType::Random(ref range) => {
                    #[cfg(feature = "rand")]
                    {
                        let mut rng = rand::thread_rng();
                        let low = range.start.as_nanos() as u64;
                        let high = range.end.as_nanos() as u64;
                        Duration::from_nanos(rng.gen_range(low..high))
                    }
                    #[cfg(not(feature = "rand"))]
                    let mut _rng = todo!();
                }
            };
            runtime.sleep_for(duration)?;

            self.output.send(&message)?;
        }
        Ok(())
    }
}
