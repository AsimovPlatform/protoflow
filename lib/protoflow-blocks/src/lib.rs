// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

mod config;
pub use config::*;

mod encoding;
pub use encoding::*;

#[cfg(feature = "std")]
mod stdio;
#[cfg(feature = "std")]
pub use stdio::*;

mod system;
pub use system::*;

pub use protoflow_core::{SystemBuilding, SystemExecution};

include!("blocks/core.rs"); // CoreBlocks
include!("blocks/flow.rs"); // FlowBlocks
include!("blocks/hash.rs"); // HashBlocks
include!("blocks/io.rs"); // IoBlocks
include!("blocks/math.rs"); // MathBlocks
include!("blocks/sys.rs"); // SysBlocks
include!("blocks/text.rs"); // TextBlocks

pub trait AllBlocks:
    CoreBlocks + FlowBlocks + HashBlocks + IoBlocks + MathBlocks + SysBlocks + TextBlocks
{
}

/// The set of block types that are enabled in this build of the crate.
#[doc(hidden)]
pub static BLOCKS: &[(&str, &str)] = &[
    // CoreBlocks
    ("core", "Buffer"),
    ("core", "Const"),
    ("core", "Count"),
    ("core", "Delay"),
    ("core", "Drop"),
    ("core", "Random"),
    // FlowBlocks
    // HashBlocks
    #[cfg(feature = "hash")]
    ("hash", "Hash"),
    // IoBlocks
    ("io", "Decode"),
    ("io", "Encode"),
    ("io", "EncodeHex"),
    // MathBlocks
    // SysBlocks
    #[cfg(feature = "std")]
    ("sys", "ReadDir"),
    #[cfg(feature = "std")]
    ("sys", "ReadEnv"),
    #[cfg(feature = "std")]
    ("sys", "ReadFile"),
    #[cfg(feature = "std")]
    ("sys", "ReadStdin"),
    #[cfg(feature = "std")]
    ("sys", "WriteFile"),
    #[cfg(feature = "std")]
    ("sys", "WriteStderr"),
    #[cfg(feature = "std")]
    ("sys", "WriteStdout"),
    // TextBlocks
];

#[cfg(feature = "std")]
#[doc(hidden)]
pub fn build_stdio_system(
    system_name: prelude::String,
    config: StdioConfig,
) -> Result<System, StdioError> {
    use prelude::String;
    Ok(match system_name.as_ref() {
        // CoreBlocks
        "Buffer" => Buffer::<String>::build_system(config)?,
        "Const" => Const::<String>::build_system(config)?,
        "Count" => Count::<String>::build_system(config)?,
        "Delay" => Delay::<String>::build_system(config)?,
        "Drop" => Drop::<String>::build_system(config)?,
        "Random" => Random::<u64>::build_system(config)?,
        // FlowBlocks
        // HashBlocks
        #[cfg(feature = "hash")]
        "Hash" => Hash::build_system(config)?,
        // IoBlocks
        "Decode" => Decode::build_system(config)?,
        "Encode" => Encode::build_system(config)?,
        "EncodeHex" => EncodeHex::build_system(config)?,
        // MathBlocks
        // SysBlocks
        "ReadDir" => ReadDir::build_system(config)?,
        "ReadEnv" => ReadEnv::<String>::build_system(config)?,
        "ReadFile" => ReadFile::build_system(config)?,
        "ReadStdin" => ReadStdin::build_system(config)?,
        "WriteFile" => WriteFile::build_system(config)?,
        "WriteStderr" => WriteStderr::build_system(config)?,
        "WriteStdout" => WriteStdout::build_system(config)?,
        // TextBlocks
        _ => return Err(StdioError::UnknownSystem(system_name))?,
    })
}
