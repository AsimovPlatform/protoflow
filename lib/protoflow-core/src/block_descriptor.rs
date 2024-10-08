// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, MaybeLabeled, MaybeNamed, Vec},
    ParameterDescriptor, PortDescriptor,
};

/// A block is an autonomous unit of computation in a system.
pub trait BlockDescriptor: AsBlockDescriptor + MaybeNamed + MaybeLabeled {
    /// A description of this block's I/O ports.
    fn ports(&self) -> Vec<PortDescriptor> {
        let mut result = self.inputs();
        result.append(&mut self.outputs());
        result
    }

    /// A description of this block's input ports.
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![]
    }

    /// A description of this block's output ports.
    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![]
    }

    /// A description of this block's parameters.
    fn parameters(&self) -> Vec<ParameterDescriptor> {
        vec![]
    }
}

pub trait AsBlockDescriptor {
    fn as_block_descriptor(&self) -> &dyn BlockDescriptor;
}

impl<T: BlockDescriptor + Sized> AsBlockDescriptor for T {
    fn as_block_descriptor(&self) -> &dyn BlockDescriptor {
        self
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for &dyn BlockDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("BlockDescriptor", 1)?;
        state.serialize_field("name", &self.name())?;
        // TODO: add more fields
        state.end()
    }
}
