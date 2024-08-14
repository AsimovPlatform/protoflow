// This is free and unencumbered software released into the public domain.

use crate::prelude::{Bytes, VecDeque};

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct MessageBuffer {
    pub messages: VecDeque<Bytes>,
}

impl MessageBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn push(&mut self, message: Bytes) {
        self.messages.push_back(message);
    }

    pub fn pop(&mut self) -> Option<Bytes> {
        self.messages.pop_front()
    }
}
