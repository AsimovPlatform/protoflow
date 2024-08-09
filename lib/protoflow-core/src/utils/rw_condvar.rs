// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use crate::prelude::fmt;
use parking_lot::{Condvar, Mutex, RwLockReadGuard};

// See: https://github.com/Amanieu/parking_lot/issues/165
#[derive(Default)]
pub struct RwCondvar {
    mutex: Mutex<()>,
    condvar: Condvar,
}

impl RwCondvar {
    /// Creates a new condition variable which is ready to be waited on and
    /// notified.
    pub const fn new() -> Self {
        Self {
            mutex: Mutex::new(()),
            condvar: Condvar::new(),
        }
    }

    /// Wakes up one blocked thread on this condvar.
    #[inline]
    pub fn notify_one(&self) -> bool {
        self.condvar.notify_one()
    }

    /// Wakes up all blocked threads on this condvar.
    #[inline]
    pub fn notify_all(&self) -> usize {
        self.condvar.notify_all()
    }

    /// Blocks the current thread until this condition variable receives a
    /// notification.
    pub fn wait<T: ?Sized>(&self, rwlock_read_guard: &mut RwLockReadGuard<'_, T>) {
        let mutex_guard = self.mutex.lock();
        RwLockReadGuard::unlocked(rwlock_read_guard, || {
            // Move the guard in to unlock it before we re-lock `rwlock_read_guard`:
            let mut mutex_guard = mutex_guard;
            self.condvar.wait(&mut mutex_guard);
        });
    }
}

impl fmt::Debug for RwCondvar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("RwCondvar { .. }")
    }
}
