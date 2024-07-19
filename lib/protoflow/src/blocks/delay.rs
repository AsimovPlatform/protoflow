// This is free and unencumbered software released into the public domain.

use crate::{Block, InputPort, Message, OutputPort, PortDescriptor};
use std::{ops::Range, time::Duration};

#[cfg(feature = "std")]
use rand::Rng;

/// A block that delays messages by a fixed duration.
pub struct Delay<T: Message> {
    input: InputPort<T>,
    output: OutputPort<T>,
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

    fn execute(&mut self) {
        while let Some(message) = self.input.receive() {
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
            std::thread::sleep(duration);

            self.output.send(message);
        }
    }
}
