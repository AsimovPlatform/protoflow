// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

mod core;
pub use core::*;

mod io;
pub use io::*;

#[cfg(feature = "std")]
mod sys;
#[cfg(feature = "std")]
pub use sys::*;

//mod text;
//pub use text::*;

/// The set of block types that are enabled in this build of the crate.
pub static BLOCKS: &[&str] = &[
    "Buffer",
    "Const",
    "Count",
    "Delay",
    "Drop",
    "Random",
    "Read",
    #[cfg(feature = "std")]
    "ReadDir",
    #[cfg(feature = "std")]
    "ReadEnv",
    #[cfg(feature = "std")]
    "ReadFile",
    #[cfg(feature = "std")]
    "ReadStdin",
    "Write",
    #[cfg(feature = "std")]
    "WriteFile",
    #[cfg(feature = "std")]
    "WriteStderr",
    #[cfg(feature = "std")]
    "WriteStdout",
];
