// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, PhantomData, Rc},
    BlockError, Message, Port, PortID, PortState, System,
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutputPort<T: Message> {
    _phantom: PhantomData<T>,
    id: PortID,
}

impl<T: Message> OutputPort<T> {
    pub fn new(system: &Rc<System>) -> Self {
        Self {
            _phantom: PhantomData,
            id: system.source_id.replace_with(|&mut id| id - 1),
        }
    }

    pub fn close(&mut self) -> Result<(), BlockError> {
        Ok(()) // TODO
    }

    pub fn send(&self, _message: &T) -> Result<(), BlockError> {
        Ok(()) // TODO
    }
}

impl<T: Message> Port for OutputPort<T> {
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

impl<T: Message> fmt::Display for OutputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}→", self.id)
    }
}
