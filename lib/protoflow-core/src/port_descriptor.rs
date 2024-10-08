// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{type_name, Cow, MaybeLabeled, MaybeNamed, String, ToString},
    InputPort, Message, OutputPort, Port, PortID, PortState,
};

/// The dataflow direction of a port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum PortDirection {
    Input,
    Output,
}

/// A descriptor for a block port.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PortDescriptor {
    /// The dataflow direction of this port.
    pub direction: PortDirection,

    /// The machine-readable name of this port, if any.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// A human-readable label for this port, if any.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,

    /// The data type for messages on this port.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub r#type: Option<String>,

    /// The unique identifier for this port.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub id: PortID,

    /// The current state of this port.
    #[cfg_attr(feature = "serde", serde(skip))]
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

impl MaybeNamed for PortDescriptor {
    fn name(&self) -> Option<Cow<str>> {
        self.name.as_deref().map(Cow::Borrowed)
    }
}

impl MaybeLabeled for PortDescriptor {
    fn label(&self) -> Option<Cow<str>> {
        self.label.as_deref().map(Cow::Borrowed)
    }
}

impl Port for PortDescriptor {
    fn id(&self) -> PortID {
        self.id
    }

    fn state(&self) -> PortState {
        self.state
    }
}

impl<T: Message> From<&InputPort<T>> for PortDescriptor {
    fn from(port: &InputPort<T>) -> Self {
        Self {
            direction: PortDirection::Input,
            name: port.name().map(|s| s.to_string()),
            label: port.label().map(|s| s.to_string()),
            r#type: Some(type_name::<T>().to_string()),
            id: port.id(),
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
            id: port.id(),
            state: port.state(),
        }
    }
}
