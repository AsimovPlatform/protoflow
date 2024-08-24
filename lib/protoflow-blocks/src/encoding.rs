// This is free and unencumbered software released into the public domain.

use crate::prelude::{FromStr, String};

/// The encoding to use when (de)serializing messages from/to bytes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Encoding {
    #[default]
    ProtobufWithLengthPrefix,
    ProtobufWithoutLengthPrefix,
    TextWithNewlineSuffix,
}

impl FromStr for Encoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "protobuf-with-length-prefix" | "protobuf" => Ok(Self::ProtobufWithLengthPrefix),
            "protobuf-without-length-prefix" => Ok(Self::ProtobufWithoutLengthPrefix),
            "text-with-newline-suffix" | "text" => Ok(Self::TextWithNewlineSuffix),
            _ => Err(String::from(s)),
        }
    }
}
