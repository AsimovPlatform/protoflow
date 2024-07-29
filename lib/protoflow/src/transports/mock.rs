// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, ToString, Vec},
    transport::Transport,
    InputPortID, Message, MessageBuffer, OutputPortID, PortError, PortID, PortResult, PortState,
};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::sync::RwLock; // FIXME

#[derive(Debug, Default)]
pub struct MockTransport {
    pub state: RwLock<MockTransportState>,
}

#[derive(Debug, Default)]
pub struct MockTransportState {
    outputs: Vec<PortState>,
    inputs: Vec<PortState>,
    inboxes: Vec<MessageBuffer>,
}

impl MockTransport {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            state: RwLock::new(MockTransportState::default()),
        })
    }
}

impl Transport for MockTransport {
    fn state(&self, port: PortID) -> PortResult<PortState> {
        let state = self.state.read().unwrap();
        match port {
            PortID::Input(port) => match state.inputs.get(port.index()) {
                None => Err(PortError::Invalid),
                Some(state) => Ok(*state),
            },
            PortID::Output(port) => match state.outputs.get(port.index()) {
                None => Err(PortError::Invalid),
                Some(state) => Ok(*state),
            },
        }
    }

    fn close(&self, port: PortID) -> PortResult<bool> {
        let mut state = self.state.write().unwrap();
        match port {
            PortID::Input(port) => match state.inputs.get_mut(port.index()) {
                None => Err(PortError::Invalid),
                Some(port_state) => match port_state {
                    PortState::Closed => Ok(false), // already closed
                    PortState::Open => {
                        *port_state = PortState::Closed;
                        Ok(true)
                    }
                    PortState::Connected(_) => {
                        // TODO: close the connected output port as well
                        *port_state = PortState::Closed;
                        state.inboxes[port.index()].clear();
                        Ok(true)
                    }
                },
            },
            PortID::Output(port) => match state.outputs.get_mut(port.index()) {
                None => Err(PortError::Invalid),
                Some(port_state) => match port_state {
                    PortState::Closed => Ok(false), // already closed
                    PortState::Open => {
                        *port_state = PortState::Closed;
                        Ok(true)
                    }
                    PortState::Connected(_) => {
                        // TODO: close the connected input port as well
                        *port_state = PortState::Closed;
                        Ok(true)
                    }
                },
            },
        }
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        let mut state = self.state.write().unwrap();
        state.inputs.push(PortState::Open);
        state.inboxes.push(MessageBuffer::new());
        InputPortID::try_from(state.inputs.len() as isize)
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let mut state = self.state.write().unwrap();
        state.outputs.push(PortState::Open);
        OutputPortID::try_from(state.outputs.len() as isize)
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let mut state = self.state.write().unwrap();
        match (
            state.outputs.get(source.index()),
            state.inputs.get(target.index()),
        ) {
            (Some(PortState::Open), Some(PortState::Open)) => {
                state.outputs[source.index()] = PortState::Connected(PortID::Input(target));
                state.inputs[target.index()] = PortState::Connected(PortID::Output(source));
                Ok(true)
            }
            _ => Err(PortError::Invalid), // TODO: better errors
        }
    }

    fn send(&self, output: OutputPortID, message: Box<dyn Message>) -> PortResult<()> {
        let state = self.state.read().unwrap();
        let input = match state.outputs.get(output.index()) {
            None => return Err(PortError::Invalid),
            Some(PortState::Closed) => return Err(PortError::Closed),
            Some(PortState::Open) => return Err(PortError::Disconnected),
            Some(PortState::Connected(PortID::Output(_))) => unreachable!(),
            Some(PortState::Connected(PortID::Input(input))) => *input,
        };

        let mut state = self.state.write().unwrap();
        state.inboxes[input.index()].push(message);
        Ok(())
    }

    fn recv(&self, input: InputPortID) -> PortResult<Box<dyn Message>> {
        let state = self.state.read().unwrap();
        match state.inputs.get(input.index()) {
            None => return Err(PortError::Invalid),
            Some(PortState::Closed) => return Err(PortError::Closed),
            Some(PortState::Open) => return Err(PortError::Disconnected), // TODO?
            Some(PortState::Connected(PortID::Input(_))) => unreachable!(),
            Some(PortState::Connected(PortID::Output(_))) => (),
        };

        let mut state = self.state.write().unwrap();
        state.inboxes[input.index()].pop().ok_or(PortError::Closed) // FIXME
    }
}
