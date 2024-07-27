// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
mod std;
#[cfg(feature = "std")]
pub use std::*;

//#[cfg(feature = "tokio")]
//mod tokio;
//#[cfg(feature = "tokio")]
//pub use tokio::*;

//#[cfg(feature = "web")]
//mod web;
//#[cfg(feature = "web")]
//pub use web::*;
