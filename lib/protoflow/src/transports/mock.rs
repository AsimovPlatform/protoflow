// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Box, ToString, Vec},
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

    pub fn with_ports(input: usize, output: usize) -> Box<Self> {
        let mut inboxes = Vec::with_capacity(input);
        inboxes.resize_with(input, || MessageBuffer::new());
        let state = MockTransportState {
            outputs: vec![PortState::Open; output],
            inputs: vec![PortState::Open; input],
            inboxes,
        };
        Box::new(Self {
            state: RwLock::new(state),
        })
    }
}

impl Transport for MockTransport {
    fn state(&self, port: PortID) -> PortResult<PortState> {
        let state = self.state.read().unwrap();
        match port {
            PortID::Input(inport) => match state.inputs.get(inport.index()) {
                None => Err(PortError::Invalid(port)),
                Some(state) => Ok(*state),
            },
            PortID::Output(outport) => match state.outputs.get(outport.index()) {
                None => Err(PortError::Invalid(port)),
                Some(state) => Ok(*state),
            },
        }
    }

    fn close(&self, port: PortID) -> PortResult<bool> {
        let mut state = self.state.write().unwrap();
        match port {
            PortID::Input(inport) => match state.inputs.get_mut(inport.index()) {
                None => Err(PortError::Invalid(port)),
                Some(port_state) => match port_state {
                    PortState::Closed => Ok(false), // already closed
                    PortState::Open => {
                        *port_state = PortState::Closed;
                        Ok(true)
                    }
                    PortState::Connected(_) => {
                        // TODO: close the connected output port as well
                        *port_state = PortState::Closed;
                        state.inboxes[inport.index()].clear();
                        Ok(true)
                    }
                },
            },
            PortID::Output(outport) => match state.outputs.get_mut(outport.index()) {
                None => Err(PortError::Invalid(port)),
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
            _ => Err(PortError::Invalid(PortID::Output(source))), // TODO: better errors
        }
    }

    fn send(&self, output: OutputPortID, message: Box<dyn Message>) -> PortResult<()> {
        let state = self.state.read().unwrap();
        let input = match state.outputs.get(output.index()) {
            None => return Err(PortError::Invalid(PortID::Output(output))),
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
            None => return Err(PortError::Invalid(PortID::Input(input))),
            Some(PortState::Closed) => return Err(PortError::Closed),
            Some(PortState::Open) => return Err(PortError::Disconnected), // TODO?
            Some(PortState::Connected(PortID::Input(_))) => unreachable!(),
            Some(PortState::Connected(PortID::Output(_))) => (),
        };

        let mut state = self.state.write().unwrap();
        state.inboxes[input.index()].pop().ok_or(PortError::Closed) // FIXME
    }
}
