// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{type_name, String, ToString},
    InputPort, Message, OutputPort, Port, PortID, PortState,
};

/// The dataflow direction of a port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortDirection {
    Input,
    Output,
}

/// A descriptor for a block port.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PortDescriptor {
    /// The dataflow direction of this port.
    pub direction: PortDirection,

    /// The machine-readable name of this port.
    pub name: Option<String>,

    /// A human-readable label for this port.
    pub label: Option<String>,

    /// The data type for messages on this port.
    pub r#type: Option<String>,

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
            name: port.name().map(|s| s.to_string()),
            label: port.label().map(|s| s.to_string()),
            r#type: Some(type_name::<T>().to_string()),
            state: port.state(),
        }
    }
}

impl<T: Message> From<&OutputPort<T>> for PortDescriptor {
    fn from(port: &OutputPort<T>) -> Self {
        Self {
            direction: PortDirection::Output,
            name: port.name().map(|s| s.to_string()),
            label: port.label().map(|s| s.to_string()),
            r#type: Some(type_name::<T>().to_string()),
            state: port.state(),
        }
    }
}
