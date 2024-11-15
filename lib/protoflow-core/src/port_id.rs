// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, TryFrom};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum PortID {
    Input(InputPortID),
    Output(OutputPortID),
}

impl PortID {
    pub fn as_isize(&self) -> isize {
        match self {
            PortID::Input(id) => id.0,
            PortID::Output(id) => id.0,
        }
    }

    pub fn as_usize(&self) -> usize {
        self.as_isize() as _
    }
}

impl TryFrom<isize> for PortID {
    type Error = &'static str;

    fn try_from(id: isize) -> Result<PortID, &'static str> {
        if id < 0 {
            Ok(Self::Input(InputPortID(id)))
        } else if id > 0 {
            Ok(Self::Output(OutputPortID(id)))
        } else {
            Err("Port ID cannot be zero")
        }
    }
}

impl From<InputPortID> for PortID {
    fn from(port_id: InputPortID) -> PortID {
        PortID::Input(port_id)
    }
}

impl From<OutputPortID> for PortID {
    fn from(port_id: OutputPortID) -> PortID {
        PortID::Output(port_id)
    }
}

impl From<PortID> for isize {
    fn from(port_id: PortID) -> isize {
        port_id.as_isize()
    }
}

impl From<PortID> for usize {
    fn from(port_id: PortID) -> usize {
        port_id.as_usize()
    }
}

impl fmt::Display for PortID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortID::Input(id) => write!(f, "{}", id),
            PortID::Output(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputPortID(pub(crate) isize);

impl InputPortID {
    #[doc(hidden)]
    pub fn index(&self) -> usize {
        self.0.unsigned_abs() - 1
    }
}

impl TryFrom<isize> for InputPortID {
    type Error = &'static str;

    fn try_from(id: isize) -> Result<InputPortID, &'static str> {
        if id < 0 {
            Ok(InputPortID(id))
        } else {
            Err("Input port IDs must be negative integers")
        }
    }
}

impl From<InputPortID> for isize {
    fn from(id: InputPortID) -> isize {
        id.0
    }
}

impl From<InputPortID> for usize {
    fn from(id: InputPortID) -> usize {
        id.0.unsigned_abs()
    }
}

impl fmt::Display for InputPortID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputPortID(pub(crate) isize);

impl OutputPortID {
    #[doc(hidden)]
    pub fn index(&self) -> usize {
        (self.0 as usize) - 1
    }
}

impl TryFrom<isize> for OutputPortID {
    type Error = &'static str;

    fn try_from(id: isize) -> Result<OutputPortID, &'static str> {
        if id > 0 {
            Ok(OutputPortID(id))
        } else {
            Err("Output port IDs must be positive integers")
        }
    }
}

impl From<OutputPortID> for isize {
    fn from(id: OutputPortID) -> isize {
        id.0
    }
}

impl From<OutputPortID> for usize {
    fn from(id: OutputPortID) -> usize {
        id.0 as usize
    }
}

impl fmt::Display for OutputPortID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
