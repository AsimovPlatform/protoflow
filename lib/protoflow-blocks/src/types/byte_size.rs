// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, FromStr};

/// A byte size value.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteSize(ubyte::ByteUnit);

impl ByteSize {
    pub const fn new(value: u64) -> Self {
        Self(ubyte::ByteUnit::Byte(value))
    }

    pub const fn as_u64(self) -> u64 {
        self.0.as_u64()
    }

    pub const fn as_usize(self) -> usize {
        self.0.as_u64() as _
    }
}

impl Into<u64> for ByteSize {
    fn into(self) -> u64 {
        self.as_u64()
    }
}

impl Into<usize> for ByteSize {
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl From<u64> for ByteSize {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<usize> for ByteSize {
    fn from(value: usize) -> Self {
        Self::new(value as _)
    }
}

impl FromStr for ByteSize {
    type Err = InvalidByteSize;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(input).map(ByteSize)
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub type InvalidByteSize = ubyte::Error;
