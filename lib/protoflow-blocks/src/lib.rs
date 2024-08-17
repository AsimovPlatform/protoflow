// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

mod core;
pub use core::*;

mod encoding;
pub use encoding::*;

mod flow;
pub use flow::*;

mod io;
pub use io::*;

mod math;
pub use math::*;

#[cfg(not(feature = "std"))]
pub trait SysBlocks {}

#[cfg(feature = "std")]
mod sys;
#[cfg(feature = "std")]
pub use sys::*;

mod system;
pub use system::*;

mod text;
pub use text::*;

pub trait AllBlocks:
    CoreBlocks + FlowBlocks + IoBlocks + MathBlocks + SysBlocks + TextBlocks
{
}

/// The set of block types that are enabled in this build of the crate.
pub static BLOCKS: &[(&str, &str)] = &[
    // CoreBlocks
    ("core", "Buffer"),
    ("core", "Const"),
    ("core", "Count"),
    ("core", "Delay"),
    ("core", "Drop"),
    ("core", "Random"),
    // FlowBlocks
    // IoBlocks
    ("io", "Decode"),
    ("io", "Encode"),
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
