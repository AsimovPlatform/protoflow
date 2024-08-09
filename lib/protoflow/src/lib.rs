// This is free and unencumbered software released into the public domain.

#![no_std]

extern crate self as protoflow;

#[doc(hidden)]
pub use protoflow_core::prelude;

pub use protoflow_core::*;

/// Default blocks are available if the crate was built with a
/// `features = ["blocks"]` configuration.
#[cfg(feature = "blocks")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocks")))]
pub use protoflow_blocks as blocks;

/// Derive macros are available if the crate was built with a
/// `features = ["derive"]` configuration.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use protoflow_derive as derive;

mod feature;
pub use feature::*;

/// The parser is available if the crate was built with a
/// `features = ["syntax"]` configuration.
#[cfg(feature = "syntax")]
#[cfg_attr(docsrs, doc(cfg(feature = "syntax")))]
pub use protoflow_syntax as syntax;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
