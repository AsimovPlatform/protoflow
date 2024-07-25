// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as alloc;

#[allow(unused)]
pub use alloc::{
    boxed::Box,
    collections::BTreeSet,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[allow(unused)]
pub use core::{
    fmt,
    marker::PhantomData,
    ops::Range,
    result::Result,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub type Instant = Duration;
