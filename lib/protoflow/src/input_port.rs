// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, PhantomData, String},
    BlockError, Message, Port, PortState,
};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub fn close(&mut self) -> Result<(), BlockError> {
        self.state = PortState::Closed;
        Ok(())
    }

    pub fn receive(&self) -> Result<Option<T>, BlockError> {
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

impl<T: Message> fmt::Display for InputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
