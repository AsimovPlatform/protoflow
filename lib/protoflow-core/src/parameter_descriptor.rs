// This is free and unencumbered software released into the public domain.

use crate::prelude::{Cow, MaybeLabeled, Named, String};

/// A descriptor for a block parameter.
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParameterDescriptor {
    /// The machine-readable name of this parameter.
    pub name: String,

    /// A human-readable label, if any, for this parameter.
    pub label: Option<String>,

    /// The data type, if known, of this parameter.
    pub r#type: Option<String>,

    /// A default value, if any, for this parameter.
    pub default_value: Option<String>,
}

impl Named for ParameterDescriptor {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}

impl MaybeLabeled for ParameterDescriptor {
    fn label(&self) -> Option<Cow<str>> {
        self.label.as_deref().map(Cow::Borrowed)
    }
}
