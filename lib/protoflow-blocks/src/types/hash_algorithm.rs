// This is free and unencumbered software released into the public domain.

use crate::prelude::{FromStr, String};

/// The cryptographic hash algorithm to use.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HashAlgorithm {
    #[default]
    BLAKE3,
}

impl FromStr for HashAlgorithm {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "blake3" => Ok(Self::BLAKE3),
            _ => Err(String::from(input)),
        }
    }
}
