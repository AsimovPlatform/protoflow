// This is free and unencumbered software released into the public domain.

pub trait IoBlocks {}

mod read;
pub use read::*;

mod write;
pub use write::*;
