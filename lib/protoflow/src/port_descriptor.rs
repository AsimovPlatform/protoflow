// This is free and unencumbered software released into the public domain.

use crate::{InputPort, Message, OutputPort, Port, PortState};

pub struct PortDescriptor {
    state: PortState,
    name: String,
    label: Option<String>,
}

impl Port for PortDescriptor {
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

impl<T: Message> From<&InputPort<T>> for PortDescriptor {
    fn from(port: &InputPort<T>) -> Self {
        Self {
            state: port.state(),
            name: port.name().to_string(),
            label: port.label().map(|s| s.to_string()),
        }
    }
}

impl<T: Message> From<&OutputPort<T>> for PortDescriptor {
    fn from(port: &OutputPort<T>) -> Self {
        Self {
            state: port.state(),
            name: port.name().to_string(),
            label: port.label().map(|s| s.to_string()),
        }
    }
}
