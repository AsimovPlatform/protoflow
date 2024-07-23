// This is free and unencumbered software released into the public domain.

extern crate alloc;

#[allow(unused)]
pub use alloc::{
    boxed::Box,
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
