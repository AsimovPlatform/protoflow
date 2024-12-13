// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, Box, Cow, FromStr, Named, String, Vec},
    BlockInstantiation, System,
};
use enum_iterator::Sequence;
use protoflow_core::{types::Any, Block, ComparableAny};

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
    Batch,
    Concat,
    Distinct,
    Merge,
    Replicate,
    Sort,
    Split,
    // HashBlocks
    #[cfg(any(
        feature = "hash-blake3",
        feature = "hash-md5",
        feature = "hash-sha1",
        feature = "hash-sha2"
    ))]
    Hash,
    // IoBlocks
    Decode,
    DecodeHex,
    DecodeJson,
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
    #[cfg(all(feature = "std", feature = "serde"))]
    ReadSocket,
    #[cfg(feature = "std")]
    ReadStdin,
    #[cfg(feature = "std")]
    WriteFile,
    #[cfg(all(feature = "std", feature = "serde"))]
    WriteSocket,
    #[cfg(feature = "std")]
    WriteStderr,
    #[cfg(feature = "std")]
    WriteStdout,
    // TextBlocks
    ConcatStrings,
    DecodeCsv,
    EncodeCsv,
    SplitString,
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
            Batch => "Batch",
            Buffer => "Buffer",
            Concat => "Concat",
            Const => "Const",
            Count => "Count",
            Delay => "Delay",
            Drop => "Drop",
            Random => "Random",
            #[cfg(any(
                feature = "hash-blake3",
                feature = "hash-md5",
                feature = "hash-sha1",
                feature = "hash-sha2"
            ))]
            Hash => "Hash",
            Decode => "Decode",
            DecodeHex => "DecodeHex",
            DecodeJson => "DecodeJSON",
            Distinct => "Distinct",
            Encode => "Encode",
            EncodeHex => "EncodeHex",
            EncodeJson => "EncodeJSON",
            Merge => "Merge",
            #[cfg(feature = "std")]
            ReadDir => "ReadDir",
            #[cfg(feature = "std")]
            ReadEnv => "ReadEnv",
            #[cfg(feature = "std")]
            ReadFile => "ReadFile",
            #[cfg(all(feature = "std", feature = "serde"))]
            ReadSocket => "ReadSocket",
            #[cfg(feature = "std")]
            ReadStdin => "ReadStdin",
            Replicate => "Replicate",
            Sort => "Sort",
            Split => "Split",
            #[cfg(feature = "std")]
            WriteFile => "WriteFile",
            #[cfg(all(feature = "std", feature = "serde"))]
            WriteSocket => "WriteSocket",
            #[cfg(feature = "std")]
            WriteStderr => "WriteStderr",
            #[cfg(feature = "std")]
            WriteStdout => "WriteStdout",
            ConcatStrings => "ConcatStrings",
            DecodeCsv => "DecodeCSV",
            EncodeCsv => "EncodeCSV",
            SplitString => "SplitString",
        }
    }
}

impl FromStr for BlockTag {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use BlockTag::*;
        Ok(match input {
            "Batch" => Batch,
            "Buffer" => Buffer,
            "Concat" => Concat,
            "Const" => Const,
            "Count" => Count,
            "Delay" => Delay,
            "Distinct" => Distinct,
            "Drop" => Drop,
            "Random" => Random,
            #[cfg(any(
                feature = "hash-blake3",
                feature = "hash-md5",
                feature = "hash-sha1",
                feature = "hash-sha2"
            ))]
            "Hash" => Hash,
            "Decode" => Decode,
            "DecodeHex" => DecodeHex,
            "DecodeJSON" => DecodeJson,
            "Encode" => Encode,
            "EncodeHex" => EncodeHex,
            "EncodeJSON" => EncodeJson,
            "Merge" => Merge,
            #[cfg(feature = "std")]
            "ReadDir" => ReadDir,
            #[cfg(feature = "std")]
            "ReadEnv" => ReadEnv,
            #[cfg(feature = "std")]
            "ReadFile" => ReadFile,
            #[cfg(all(feature = "std", feature = "serde"))]
            "ReadSocket" => ReadSocket,
            #[cfg(feature = "std")]
            "ReadStdin" => ReadStdin,
            "Replicate" => Replicate,
            "Sort" => Sort,
            "Split" => Split,
            #[cfg(feature = "std")]
            "WriteFile" => WriteFile,
            #[cfg(all(feature = "std", feature = "serde"))]
            "WriteSocket" => WriteSocket,
            #[cfg(feature = "std")]
            "WriteStderr" => WriteStderr,
            #[cfg(feature = "std")]
            "WriteStdout" => WriteStdout,
            "ConcatStrings" => ConcatStrings,
            "DecodeCSV" => DecodeCsv,
            "EncodeCSV" => EncodeCsv,
            "SplitString" => SplitString,
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
            Batch => Box::new(super::Batch::<Any>::with_system(system, None)),
            Buffer => Box::new(super::Buffer::<Any>::with_system(system)),
            Concat => Box::new(super::Concat::<Any>::with_system(system)),
            Const => Box::new(super::Const::<String>::with_system(system, String::new())),
            Count => Box::new(super::Count::<Any>::with_system(system)),
            Delay => Box::new(super::Delay::<Any>::with_system(system, None)),
            Drop => Box::new(super::Drop::<Any>::with_system(system)),
            Random => Box::new(super::Random::<u64>::with_system(system, None)),
            #[cfg(any(
                feature = "hash-blake3",
                feature = "hash-md5",
                feature = "hash-sha1",
                feature = "hash-sha2"
            ))]
            Hash => Box::new(super::Hash::with_system(system, None)),
            Decode => Box::new(super::Decode::<String>::with_system(system, None)),
            DecodeHex => Box::new(super::DecodeHex::with_system(system)),
            DecodeJson => Box::new(super::DecodeJson::with_system(system)),
            Distinct => Box::new(super::Distinct::<Any>::with_system(system)),
            Encode => Box::new(super::Encode::<String>::with_system(system, None)),
            EncodeHex => Box::new(super::EncodeHex::with_system(system)),
            EncodeJson => Box::new(super::EncodeJson::with_system(system)),
            Merge => Box::new(super::Merge::<Any>::with_system(system)),
            #[cfg(feature = "std")]
            ReadDir => Box::new(super::ReadDir::with_system(system)),
            #[cfg(feature = "std")]
            ReadEnv => Box::new(super::ReadEnv::<String>::with_system(system)),
            #[cfg(feature = "std")]
            ReadFile => Box::new(super::ReadFile::with_system(system)),
            #[cfg(all(feature = "std", feature = "serde"))]
            ReadSocket => Box::new(super::ReadSocket::with_system(system, None)),
            #[cfg(feature = "std")]
            ReadStdin => Box::new(super::ReadStdin::with_system(system, None)),
            Replicate => Box::new(super::Replicate::<Any>::with_system(system)),
            Sort => Box::new(super::Sort::<ComparableAny>::with_system(system)),
            Split => Box::new(super::Split::<Any>::with_system(system)),
            #[cfg(feature = "std")]
            WriteFile => Box::new(super::WriteFile::with_system(system, None)),
            #[cfg(all(feature = "std", feature = "serde"))]
            WriteSocket => Box::new(super::WriteSocket::with_system(system, None)),
            #[cfg(feature = "std")]
            WriteStderr => Box::new(super::WriteStderr::with_system(system)),
            #[cfg(feature = "std")]
            WriteStdout => Box::new(super::WriteStdout::with_system(system)),
            ConcatStrings => Box::new(super::ConcatStrings::with_system(system, None)),
            DecodeCsv => Box::new(super::DecodeCsv::with_system(system)),
            EncodeCsv => Box::new(super::EncodeCsv::with_system(system)),
            SplitString => Box::new(super::SplitString::with_system(system, None)),
        }
    }
}
