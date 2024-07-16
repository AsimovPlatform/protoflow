// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Port(String);

impl Port {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Port {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for Port {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl AsRef<str> for Port {
    fn as_ref(&self) -> &str {
        self.name()
    }
}
