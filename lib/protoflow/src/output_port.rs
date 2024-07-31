// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, Arc, PhantomData},
    IntoMessage, Message, OutputPortID, Port, PortID, PortResult, PortState, System, Transport,
};

#[derive(Clone)] //, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutputPort<T: Message> {
    _phantom: PhantomData<T>,
    pub(crate) id: OutputPortID,
    pub(crate) transport: Arc<dyn Transport>,
}

impl<T: Message + Clone + 'static> OutputPort<T> {
    pub fn new<X: Transport + Default>(system: &System<X>) -> Self {
        let runtime = system.runtime.as_ref();
        let transport = runtime.transport.clone();
        Self {
            _phantom: PhantomData,
            id: transport.open_output().unwrap(),
            transport,
        }
    }

    pub fn close(&mut self) -> PortResult<bool> {
        self.transport.close(PortID::Output(self.id))
    }

    pub fn send(&self, message: impl IntoMessage) -> PortResult<()> {
        self.transport.send(self.id, message.into_message())
    }
}

impl<T: Message> Port for OutputPort<T> {
    fn id(&self) -> Option<PortID> {
        Some(PortID::Output(self.id))
    }

    fn state(&self) -> PortState {
        self.transport
            .state(PortID::Output(self.id))
            .unwrap_or(PortState::Closed)
    }
}

impl<T: Message> fmt::Display for OutputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}→", self.id)
    }
}
