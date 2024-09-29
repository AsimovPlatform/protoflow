// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, FromStr, String};

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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Encoding::*;
        Ok(match input {
            "protobuf-with-length-prefix" | "protobuf" => ProtobufWithLengthPrefix,
            "protobuf-without-length-prefix" => ProtobufWithoutLengthPrefix,
            "text-with-newline-suffix" | "text" => TextWithNewlineSuffix,
            _ => return Err(String::from(input)),
        })
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Encoding::*;
        match self {
            ProtobufWithLengthPrefix => write!(f, "protobuf-with-length-prefix"),
            ProtobufWithoutLengthPrefix => write!(f, "protobuf-without-length-prefix"),
            TextWithNewlineSuffix => write!(f, "text-with-newline-suffix"),
        }
    }
}
