// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{type_name, String, ToString},
    InputPort, Message, OutputPort, Port, PortID, PortState,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortDirection {
    Input,
    Output,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PortDescriptor {
    /// The direction of this port.
    pub direction: PortDirection,

    /// The type of data that this port carries.
    pub r#type: Option<String>,

    /// The machine-readable name of this port.
    pub name: Option<String>,

    /// A human-readable label for this port.
    pub label: Option<String>,

    /// The current state of this port.
    pub state: PortState,
}

impl PortDescriptor {
    pub fn is_input(&self) -> bool {
        self.direction == PortDirection::Input
    }

    pub fn is_output(&self) -> bool {
        self.direction == PortDirection::Output
    }
}

impl Port for PortDescriptor {
    fn id(&self) -> Option<PortID> {
        None
    }

    fn state(&self) -> PortState {
        self.state
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl<T: Message> From<&InputPort<T>> for PortDescriptor {
    fn from(port: &InputPort<T>) -> Self {
        Self {
            direction: PortDirection::Input,
            r#type: Some(type_name::<T>().to_string()),
            name: port.name().map(|s| s.to_string()),
            label: port.label().map(|s| s.to_string()),
            state: port.state(),
        }
    }
}

impl<T: Message> From<&OutputPort<T>> for PortDescriptor {
    fn from(port: &OutputPort<T>) -> Self {
        Self {
            direction: PortDirection::Output,
            r#type: Some(type_name::<T>().to_string()),
            name: port.name().map(|s| s.to_string()),
            label: port.label().map(|s| s.to_string()),
            state: port.state(),
        }
    }
}
