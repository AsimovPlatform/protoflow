// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OutputPort(String);

impl OutputPort {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OutputPort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}→", self.0)
    }
}

impl From<&str> for OutputPort {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for OutputPort {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl AsRef<str> for OutputPort {
    fn as_ref(&self) -> &str {
        self.name()
    }
}
