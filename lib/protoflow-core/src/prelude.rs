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
    collections::btree_set::Iter as BTreeSetIter,
    collections::BTreeSet,
    collections::VecDeque,
    rc::Rc,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};

#[allow(unused)]
pub use core::{
    cell::RefCell,
    convert::TryFrom,
    fmt,
    marker::PhantomData,
    ops::Range,
    result::Result,
    slice,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::Duration,
};

pub use bytes::Bytes;

pub use prost;
pub use prost_types;

pub type Instant = Duration;
