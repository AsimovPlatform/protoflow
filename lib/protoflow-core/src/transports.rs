// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
pub type MockTransport = MpscTransport;

#[cfg(feature = "std")]
mod mpsc;
#[cfg(feature = "std")]
pub use mpsc::*;
