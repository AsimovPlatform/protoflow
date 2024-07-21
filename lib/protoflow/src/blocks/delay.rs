// This is free and unencumbered software released into the public domain.

use crate::{Block, InputPort, Message, OutputPort, Port, PortDescriptor, Scheduler};
use std::{ops::Range, time::Duration};

#[cfg(feature = "std")]
use rand::Rng;

/// A block that passes messages through while delaying them by a fixed or
/// random duration.
pub struct Delay<T: Message> {
    /// The input message stream.
    input: InputPort<T>,
    /// The output target for the stream being passed through.
    output: OutputPort<T>,
    /// A configuration parameter for which type of delay to add.
    delay: DelayType,
}

/// The type of delay (fixed or random) to apply to message relay.
pub enum DelayType {
    Fixed(Duration),
    Random(Range<Duration>),
}

impl<T: Message> Block for Delay<T> {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.input)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.output)]
    }

    fn execute(&mut self, scheduler: &dyn Scheduler) {
        while let Some(message) = self.input.receive() {
            if !self.output.is_connected() {
                drop(message);
                continue;
            }

            let duration = match self.delay {
                DelayType::Fixed(duration) => duration,
                DelayType::Random(ref range) => {
                    #[cfg(feature = "std")]
                    {
                        let mut rng = rand::thread_rng();
                        let low = range.start.as_nanos() as u64;
                        let high = range.end.as_nanos() as u64;
                        Duration::from_nanos(rng.gen_range(low, high))
                    }
                    #[cfg(not(feature = "std"))]
                    let mut _rng = todo!();
                }
            };
            scheduler.sleep(duration);

            self.output.send(&message);
        }
    }
}
