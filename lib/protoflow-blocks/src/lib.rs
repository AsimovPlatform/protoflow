// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

mod core;
pub use core::*;

mod flow;
pub use flow::*;

mod io;
pub use io::*;

mod math;
pub use math::*;

#[cfg(feature = "std")]
mod sys;
#[cfg(feature = "std")]
pub use sys::*;

#[cfg(not(feature = "std"))]
pub trait SysBlocks {}

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
    ("io", "Read"),
    ("io", "Write"),
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
