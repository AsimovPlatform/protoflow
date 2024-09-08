// This is free and unencumbered software released into the public domain.

use crate::prelude::String;

/// A descriptor for a block parameter.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParameterDescriptor {
    /// The machine-readable name of this parameter.
    pub name: String,

    /// A human-readable label for this parameter.
    pub label: Option<String>,

    /// The data type of this parameter.
    pub r#type: Option<String>,

    /// A default value for this parameter.
    pub default_value: Option<String>,
}
