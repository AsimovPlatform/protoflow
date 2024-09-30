// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, Cow, FromStr, Named};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    Encode,
    EncodeHex,
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
            "Encode" => Encode,
            "EncodeHex" => EncodeHex,
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
        use BlockTag::*;
        Cow::Borrowed(match self {
            Buffer => "Buffer",
            Const => "Const",
            Count => "Count",
            Delay => "Delay",
            Drop => "Drop",
            Random => "Random",
            #[cfg(feature = "hash")]
            Hash => "Hash",
            Decode => "Decode",
            Encode => "Encode",
            EncodeHex => "EncodeHex",
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
        })
    }
}

impl fmt::Display for BlockTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
