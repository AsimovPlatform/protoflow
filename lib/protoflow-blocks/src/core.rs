// This is free and unencumbered software released into the public domain.

use crate::prelude::{Duration, Range, String, ToString};
use protoflow_core::Message;

pub trait CoreBlocks {
    fn buffer<T: Message + Into<T> + 'static>(&self) -> Buffer<T>;

    fn const_string(&self, value: impl ToString) -> Const<String>;

    fn count<T: Message + 'static>(&self) -> Count<T>;

    fn delay<T: Message + 'static>(&self) -> Delay<T>;

    fn delay_by<T: Message + 'static>(&self, delay: DelayType) -> Delay<T>;

    fn delay_by_fixed<T: Message + 'static>(&self, delay: Duration) -> Delay<T> {
        self.delay_by(DelayType::Fixed(delay))
    }

    fn delay_by_random<T: Message + 'static>(&self, delay: Range<Duration>) -> Delay<T> {
        self.delay_by(DelayType::Random(delay))
    }

    fn drop<T: Message + 'static>(&self) -> Drop<T>;

    fn random<T: Message + 'static>(&self) -> Random<T>;

    fn random_seeded<T: Message + 'static>(&self, seed: Option<u64>) -> Random<T>;
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
