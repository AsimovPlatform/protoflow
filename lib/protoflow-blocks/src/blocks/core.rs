// This is free and unencumbered software released into the public domain.

pub mod core {
    use super::{
        prelude::{vec, Cow, Named, Vec},
        BlockConfigConnections, InputPortName, OutputPortName,
    };
    use crate::prelude::{Duration, Range, String, ToString};
    use protoflow_core::Message;

    pub trait CoreBlocks {
        fn buffer<T: Message + Into<T> + 'static>(&mut self) -> Buffer<T>;

        fn const_string(&mut self, value: impl ToString) -> Const<String>;

        fn count<T: Message + 'static>(&mut self) -> Count<T>;

        fn delay<T: Message + 'static>(&mut self) -> Delay<T>;

        fn delay_by<T: Message + 'static>(&mut self, delay: DelayType) -> Delay<T>;

        fn delay_by_fixed<T: Message + 'static>(&mut self, delay: Duration) -> Delay<T> {
            self.delay_by(DelayType::Fixed(delay))
        }

        fn delay_by_random<T: Message + 'static>(&mut self, delay: Range<Duration>) -> Delay<T> {
            self.delay_by(DelayType::Random(delay))
        }

        fn drop<T: Message + 'static>(&mut self) -> Drop<T>;

        fn random<T: Message + 'static>(&mut self) -> Random<T>;

        fn random_seeded<T: Message + 'static>(&mut self, seed: Option<u64>) -> Random<T>;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum CoreBlocksConfig {
        Buffer {
            input: InputPortName,
        },

        Const {
            output: OutputPortName,
            value: String,
        },

        Count {
            input: InputPortName,
            output: Option<OutputPortName>,
            count: OutputPortName,
        },

        Delay {
            input: InputPortName,
            output: OutputPortName,
            delay: DelayType,
        },

        Drop {
            input: InputPortName,
        },

        Random {
            output: OutputPortName,
            seed: Option<u64>,
        },
    }

    impl Named for CoreBlocksConfig {
        fn name(&self) -> Cow<str> {
            use CoreBlocksConfig::*;
            Cow::Borrowed(match self {
                Buffer { .. } => "Buffer",
                Const { .. } => "Const",
                Count { .. } => "Count",
                Delay { .. } => "Delay",
                Drop { .. } => "Drop",
                Random { .. } => "Random",
            })
        }
    }

    impl BlockConfigConnections for CoreBlocksConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use CoreBlocksConfig::*;
            match self {
                Buffer { .. } => vec![],
                Const { output, .. } => vec![("output", Some(output.clone()))],
                Count { output, count, .. } => {
                    vec![("output", output.clone()), ("count", Some(count.clone()))]
                }
                Delay { output, .. } => vec![("output", Some(output.clone()))],
                Drop { .. } => vec![],
                Random { output, .. } => vec![("output", Some(output.clone()))],
            }
        }
    }

    mod buffer;
    pub use buffer::*;

    mod r#const;
    pub use r#const::*;

    mod count;
    pub use count::*;

    mod delay;
    pub use delay::*;

    mod drop;
    pub use drop::*;

    mod random;
    pub use random::*;
}

pub use core::*;
