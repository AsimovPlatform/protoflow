// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Box, ToString, Vec},
    transport::Transport,
    InputPortID, Message, MessageBuffer, OutputPortID, PortError, PortID, PortResult, PortState,
};
use parking_lot::{Condvar, Mutex};

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
    inwaits: Vec<(Mutex<bool>, Condvar)>,
    inboxes: Vec<MessageBuffer>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(MockTransportState::default()),
        }
    }

    pub fn with_ports(input: usize, output: usize) -> Self {
        let mut invars = Vec::with_capacity(input);
        invars.resize_with(input, || (Mutex::new(false), Condvar::new()));

        let mut inboxes = Vec::with_capacity(input);
        inboxes.resize_with(input, MessageBuffer::new);

        Self {
            state: RwLock::new(MockTransportState {
                outputs: vec![PortState::Open; output],
                inputs: vec![PortState::Open; input],
                inwaits: invars,
                inboxes,
            }),
        }
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

        let mut state = self.state.write().unwrap(); // upgrade lock
        state.inboxes[input.index()].push(message);
        {
            let (ref recv_lock, ref recv_cvar) = state.inwaits[input.index()];
            let mut _recv_guard = recv_lock.lock();
            recv_cvar.notify_one();
        }
        Ok(())
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Box<dyn Message>>> {
        let state = self.state.read().unwrap();
        if state.inputs.get(input.index()).is_none() {
            return Err(PortError::Invalid(PortID::Input(input)));
        }
        if state.inboxes[input.index()].is_empty()
            && state.inputs[input.index()] == PortState::Closed
        {
            return Ok(None); // EOS
        }

        let mut state = self.state.write().unwrap(); // upgrade lock
        if state.inputs.get(input.index()).is_none() {
            return Err(PortError::Invalid(PortID::Input(input)));
        }
        if state.inboxes[input.index()].is_empty() {
            match &state.inputs[input.index()] {
                PortState::Closed => return Ok(None), // EOS
                PortState::Open => (),
                PortState::Connected(PortID::Input(_)) => unreachable!(),
                PortState::Connected(PortID::Output(_)) => (),
            };

            let (ref recv_lock, ref recv_cvar) = state.inwaits[input.index()];
            let mut recv_guard = recv_lock.lock();
            if !*recv_guard {
                recv_cvar.wait(&mut recv_guard);
            }
        }
        Ok(state.inboxes[input.index()].pop())
    }

    fn try_recv(&self, input: InputPortID) -> PortResult<Option<Box<dyn Message>>> {
        let state = self.state.read().unwrap();
        if state.inputs.get(input.index()).is_none() {
            return Err(PortError::Invalid(PortID::Input(input)));
        }
        if state.inboxes[input.index()].is_empty() {
            return Ok(None); // no message immediately available
        }

        let mut state = self.state.write().unwrap(); // upgrade lock
        if state.inputs.get(input.index()).is_none() {
            return Err(PortError::Invalid(PortID::Input(input)));
        }
        Ok(state.inboxes[input.index()].pop())
    }
}
