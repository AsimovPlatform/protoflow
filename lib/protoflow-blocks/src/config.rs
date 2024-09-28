// This is free and unencumbered software released into the public domain.

use super::{prelude::String, DelayType, Encoding, HashAlgorithm};

pub type InputPortName = String;
pub type OutputPortName = String;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlockConfig {
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

    Decode {
        input: InputPortName,
        output: OutputPortName,
        encoding: Encoding,
    },

    Delay {
        input: InputPortName,
        output: OutputPortName,
        delay: DelayType,
    },

    Drop {
        input: InputPortName,
    },

    Encode {
        input: InputPortName,
        output: OutputPortName,
        encoding: Encoding,
    },

    EncodeHex {
        input: InputPortName,
        output: OutputPortName,
    },

    #[cfg(feature = "hash")]
    Hash {
        input: InputPortName,
        output: Option<OutputPortName>,
        hash: OutputPortName,
        algorithm: HashAlgorithm,
    },

    Random {
        output: OutputPortName,
        seed: Option<u64>,
    },

    #[cfg(feature = "std")]
    ReadDir {
        path: InputPortName,
        output: OutputPortName,
    },

    #[cfg(feature = "std")]
    ReadEnv {
        name: InputPortName,
        output: OutputPortName,
    },

    #[cfg(feature = "std")]
    ReadFile {
        path: InputPortName,
        output: OutputPortName,
    },

    #[cfg(feature = "std")]
    ReadStdin {
        output: OutputPortName,
        buffer_size: Option<usize>,
    },

    #[cfg(feature = "std")]
    WriteFile {
        path: InputPortName,
        input: InputPortName,
    },

    #[cfg(feature = "std")]
    WriteStderr {
        input: InputPortName,
    },

    #[cfg(feature = "std")]
    WriteStdout {
        input: InputPortName,
    },
}

impl BlockConfig {
    pub fn r#type(&self) -> &'static str {
        use BlockConfig::*;
        match self {
            Buffer { .. } => "Buffer",
            Const { .. } => "Const",
            Count { .. } => "Count",
            Decode { .. } => "Decode",
            Delay { .. } => "Delay",
            Drop { .. } => "Drop",
            Encode { .. } => "Encode",
            EncodeHex { .. } => "EncodeHex",
            Hash { .. } => "Hash",
            Random { .. } => "Random",
            ReadDir { .. } => "ReadDir",
            ReadEnv { .. } => "ReadEnv",
            ReadFile { .. } => "ReadFile",
            ReadStdin { .. } => "ReadStdin",
            WriteFile { .. } => "WriteFile",
            WriteStderr { .. } => "WriteStderr",
            WriteStdout { .. } => "WriteStdout",
        }
    }
}
