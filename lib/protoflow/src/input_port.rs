// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, Arc, Box, PhantomData},
    InputPortID, Message, Port, PortID, PortResult, PortState, System, Transport,
};

#[derive(Clone)] //, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InputPort<T: Message> {
    _phantom: PhantomData<T>,
    pub(crate) id: InputPortID,
    pub(crate) transport: Arc<dyn Transport>,
}

impl<T: Message> InputPort<T> {
    pub fn new<X: Transport>(system: &System<X>) -> Self {
        let runtime = system.runtime.as_ref();
        let transport = runtime.transport.clone();
        Self {
            _phantom: PhantomData,
            id: transport.open_input().unwrap(),
            transport,
        }
    }

    pub fn close(&mut self) -> PortResult<bool> {
        self.transport.close(PortID::Input(self.id))
    }

    pub fn recv(&self) -> PortResult<Option<Box<dyn Message>>> {
        self.transport.recv(self.id)
    }

    pub fn try_recv(&self) -> PortResult<Option<Box<dyn Message>>> {
        self.transport.try_recv(self.id)
    }
}

impl<T: Message> Port for InputPort<T> {
    fn id(&self) -> Option<PortID> {
        Some(PortID::Input(self.id))
    }

    fn state(&self) -> PortState {
        self.transport
            .state(PortID::Input(self.id))
            .unwrap_or(PortState::Closed)
    }
}

impl<T: Message> fmt::Display for InputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "→{}", self.id)
    }
}
