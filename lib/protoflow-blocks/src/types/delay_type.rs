// This is free and unencumbered software released into the public domain.

use crate::prelude::{Duration, FromStr, Range, String};

/// The type of delay (fixed or random) to apply to message relay.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DelayType {
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "duration_str::deserialize_duration")
    )]
    Fixed(Duration),

    Random(Range<Duration>),
}

impl Default for DelayType {
    fn default() -> Self {
        Self::Fixed(Duration::from_secs(1))
    }
}

impl FromStr for DelayType {
    type Err = InvalidDelayType;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // TODO: parse random range parameters as well
        Ok(match input.trim() {
            "" => Self::default(),
            "random" | "rand" => Self::Random(Range {
                start: Duration::from_secs_f64(0.),
                end: Duration::from_secs_f64(1.),
            }),
            input => duration_str::parse_std(input).map(Self::Fixed)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvalidDelayType {
    InvalidDuration(String),
}

impl From<String> for InvalidDelayType {
    fn from(input: String) -> Self {
        InvalidDelayType::InvalidDuration(input)
    }
}
