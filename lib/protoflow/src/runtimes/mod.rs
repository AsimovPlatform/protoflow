// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
mod std_thread;
#[cfg(feature = "std")]
pub use std_thread::*;

#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
pub use tokio::*;
