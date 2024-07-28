// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{
    prelude::{Duration, Range},
    Block, BlockError, BlockRuntime, InputPort, Message, OutputPort, Port,
};

/// A block that passes messages through while delaying them by a fixed or
/// random duration.
#[derive(Block, Clone)]
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
#[derive(Clone, Debug)]
pub enum DelayType {
    Fixed(Duration),
    Random(Range<Duration>),
}

impl<T: Message> Block for Delay<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> Result<(), BlockError> {
        while let Some(message) = self.input.receive()? {
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
