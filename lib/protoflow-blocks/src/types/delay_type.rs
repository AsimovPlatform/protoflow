// This is free and unencumbered software released into the public domain.

use crate::prelude::{Duration, Range};

/// The type of delay (fixed or random) to apply to message relay.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DelayType {
    Fixed(Duration),
    Random(Range<Duration>),
}

impl Default for DelayType {
    fn default() -> Self {
        Self::Fixed(Duration::from_secs(1))
    }
}
