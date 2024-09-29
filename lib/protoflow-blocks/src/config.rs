// This is free and unencumbered software released into the public domain.

use super::prelude::{Cow, Named, String};
use crate::{
    CoreBlocksConfig, FlowBlocksConfig, HashBlocksConfig, IoBlocksConfig, MathBlocksConfig,
    SysBlocksConfig, TextBlocksConfig,
};

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

impl Named for BlockConfig {
    fn name(&self) -> Cow<str> {
        use BlockConfig::*;
        match self {
            Core(config) => config.name(),
            Flow(config) => config.name(),
            #[cfg(feature = "hash")]
            Hash(config) => config.name(),
            Io(config) => config.name(),
            Math(config) => config.name(),
            #[cfg(feature = "std")]
            Sys(config) => config.name(),
            Text(config) => config.name(),
        }
    }
}
