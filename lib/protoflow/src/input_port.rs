// This is free and unencumbered software released into the public domain.

use crate::{Message, Port, PortState};
use std::marker::PhantomData;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InputPort<T: Message> {
    _phantom: PhantomData<T>,
    state: PortState,
    name: String,
    label: Option<String>,
}

impl<T: Message> InputPort<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            _phantom: PhantomData,
            state: PortState::default(),
            name: name.into(),
            label: None,
        }
    }

    pub fn new_with_label(name: impl Into<String>, label: Option<impl Into<String>>) -> Self {
        Self {
            _phantom: PhantomData,
            state: PortState::default(),
            name: name.into(),
            label: label.map(|s| s.into()),
        }
    }

    pub fn close(&mut self) {
        self.state = PortState::Closed;
    }

    pub fn receive(&self) -> Result<Option<T>, ()> {
        Ok(None) // TODO
    }
}

impl<T: Message> Port for InputPort<T> {
    fn state(&self) -> PortState {
        self.state
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl<T: Message> std::fmt::Display for InputPort<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "→{}", self.name)
    }
}

impl<T: Message> From<&str> for InputPort<T> {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl<T: Message> From<String> for InputPort<T> {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl<T: Message> AsRef<str> for InputPort<T> {
    fn as_ref(&self) -> &str {
        self.name()
    }
}
