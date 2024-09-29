// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, FromStr, String};

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
        use HashAlgorithm::*;
        Ok(match input {
            "blake3" | "b3" => BLAKE3,
            _ => return Err(String::from(input)),
        })
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HashAlgorithm::*;
        match self {
            BLAKE3 => write!(f, "blake3"),
        }
    }
}
