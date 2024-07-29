// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, Vec},
    Message,
};

#[derive(Debug, Default)]
pub struct MessageBuffer {
    pub(crate) messages: Vec<Box<dyn Message>>,
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

    pub fn push(&mut self, message: Box<dyn Message>) {
        self.messages.push(message);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Message>> {
        self.messages.pop()
    }
}
