// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, MaybeLabeled, MaybeNamed, Vec},
    ParameterDescriptor, PortDescriptor,
};

/// A block is an autonomous unit of computation in a system.
pub trait BlockDescriptor: MaybeNamed + MaybeLabeled {
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
