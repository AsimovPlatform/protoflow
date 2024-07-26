// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, PhantomData, Rc},
    BlockError, Message, Port, PortID, PortState, System,
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InputPort<T: Message> {
    _phantom: PhantomData<T>,
    id: PortID,
}

impl<T: Message> InputPort<T> {
    pub fn new(system: &Rc<System>) -> Self {
        Self {
            _phantom: PhantomData,
            id: system.target_id.replace_with(|&mut id| id + 1),
        }
    }

    pub fn close(&mut self) -> Result<(), BlockError> {
        Ok(()) // TODO
    }

    pub fn receive(&self) -> Result<Option<T>, BlockError> {
        Ok(None) // TODO
    }
}

impl<T: Message> Port for InputPort<T> {
    fn id(&self) -> Option<PortID> {
        Some(self.id)
    }

    fn state(&self) -> PortState {
        PortState::Closed // TODO
    }

    fn name(&self) -> &str {
        "" // TODO
    }

    fn label(&self) -> Option<&str> {
        None // TODO
    }
}

impl<T: Message> fmt::Display for InputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "→{}", self.id)
    }
}
