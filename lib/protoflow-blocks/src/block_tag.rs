// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, Box, Cow, FromStr, Named, String, Vec},
    BlockInstantiation, System,
};
use enum_iterator::Sequence;
use protoflow_core::{types::Any, Block};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Sequence)]
pub enum BlockTag {
    // CoreBlocks
    Buffer,
    Const,
    Count,
    Delay,
    Drop,
    Random,
    // FlowBlocks
    // HashBlocks
    #[cfg(feature = "hash")]
    Hash,
    // IoBlocks
    Decode,
    DecodeHex,
    Encode,
    EncodeHex,
    EncodeJson,
    // MathBlocks
    // SysBlocks
    #[cfg(feature = "std")]
    ReadDir,
    #[cfg(feature = "std")]
    ReadEnv,
    #[cfg(feature = "std")]
    ReadFile,
    #[cfg(feature = "std")]
    ReadStdin,
    #[cfg(feature = "std")]
    WriteFile,
    #[cfg(feature = "std")]
    WriteStderr,
    #[cfg(feature = "std")]
    WriteStdout,
    // TextBlocks
}

impl BlockTag {
    pub fn count() -> usize {
        enum_iterator::cardinality::<Self>()
    }

    pub fn all() -> Vec<Self> {
        enum_iterator::all::<Self>().collect()
    }

    pub fn as_str(&self) -> &'static str {
        use BlockTag::*;
        match self {
            Buffer => "Buffer",
            Const => "Const",
            Count => "Count",
            Delay => "Delay",
            Drop => "Drop",
            Random => "Random",
            #[cfg(feature = "hash")]
            Hash => "Hash",
            Decode => "Decode",
            DecodeHex => "DecodeHex",
            Encode => "Encode",
            EncodeHex => "EncodeHex",
            EncodeJson => "EncodeJSON",
            #[cfg(feature = "std")]
            ReadDir => "ReadDir",
            #[cfg(feature = "std")]
            ReadEnv => "ReadEnv",
            #[cfg(feature = "std")]
            ReadFile => "ReadFile",
            #[cfg(feature = "std")]
            ReadStdin => "ReadStdin",
            #[cfg(feature = "std")]
            WriteFile => "WriteFile",
            #[cfg(feature = "std")]
            WriteStderr => "WriteStderr",
            #[cfg(feature = "std")]
            WriteStdout => "WriteStdout",
        }
    }
}

impl FromStr for BlockTag {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use BlockTag::*;
        Ok(match input {
            "Buffer" => Buffer,
            "Const" => Const,
            "Count" => Count,
            "Delay" => Delay,
            "Drop" => Drop,
            "Random" => Random,
            #[cfg(feature = "hash")]
            "Hash" => Hash,
            "Decode" => Decode,
            "DecodeHex" => DecodeHex,
            "Encode" => Encode,
            "EncodeHex" => EncodeHex,
            "EncodeJSON" => EncodeJson,
            #[cfg(feature = "std")]
            "ReadDir" => ReadDir,
            #[cfg(feature = "std")]
            "ReadEnv" => ReadEnv,
            #[cfg(feature = "std")]
            "ReadFile" => ReadFile,
            #[cfg(feature = "std")]
            "ReadStdin" => ReadStdin,
            #[cfg(feature = "std")]
            "WriteFile" => WriteFile,
            #[cfg(feature = "std")]
            "WriteStderr" => WriteStderr,
            #[cfg(feature = "std")]
            "WriteStdout" => WriteStdout,
            _ => return Err(()),
        })
    }
}

impl Named for BlockTag {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(self.as_str())
    }
}

impl fmt::Display for BlockTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl BlockInstantiation for BlockTag {
    fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
        use BlockTag::*;
        match self {
            Buffer => Box::new(super::Buffer::<Any>::with_system(system)),
            Const => Box::new(super::Const::<String>::with_system(system, String::new())),
            Count => Box::new(super::Count::<Any>::with_system(system)),
            Delay => Box::new(super::Delay::<Any>::with_system(system, None)),
            Drop => Box::new(super::Drop::<Any>::with_system(system)),
            Random => Box::new(super::Random::<u64>::with_system(system, None)),
            #[cfg(feature = "hash")]
            Hash => Box::new(super::Hash::with_system(system, None)),
            Decode => Box::new(super::Decode::<String>::with_system(system, None)),
            DecodeHex => Box::new(super::DecodeHex::with_system(system)),
            Encode => Box::new(super::Encode::<String>::with_system(system, None)),
            EncodeHex => Box::new(super::EncodeHex::with_system(system)),
            EncodeJson => Box::new(super::EncodeJson::with_system(system)),
            #[cfg(feature = "std")]
            ReadDir => Box::new(super::ReadDir::with_system(system)),
            #[cfg(feature = "std")]
            ReadEnv => Box::new(super::ReadEnv::<String>::with_system(system)),
            #[cfg(feature = "std")]
            ReadFile => Box::new(super::ReadFile::with_system(system)),
            #[cfg(feature = "std")]
            ReadStdin => Box::new(super::ReadStdin::with_system(system, None)),
            #[cfg(feature = "std")]
            WriteFile => Box::new(super::WriteFile::with_system(system, None)),
            #[cfg(feature = "std")]
            WriteStderr => Box::new(super::WriteStderr::with_system(system)),
            #[cfg(feature = "std")]
            WriteStdout => Box::new(super::WriteStdout::with_system(system)),
        }
    }
}
