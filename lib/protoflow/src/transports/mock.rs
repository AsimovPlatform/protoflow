// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Bytes, ToString, Vec},
    transport::Transport,
    InputPortID, MessageBuffer, OutputPortID, PortError, PortID, PortResult, PortState,
};
use parking_lot::RwLock;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::sync::{Condvar, Mutex};

#[derive(Debug, Default)]
pub struct MockTransport {
    pub state: RwLock<MockTransportState>,
    pub(crate) wakeup: (Mutex<bool>, Condvar),
}

#[derive(Debug, Default)]
pub struct MockTransportState {
    outputs: Vec<PortState>,
    inputs: Vec<PortState>,
    inboxes: Vec<MessageBuffer>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(MockTransportState::default()),
            wakeup: (Mutex::new(false), Condvar::new()),
        }
    }

    pub fn with_ports(input: usize, output: usize) -> Self {
        let mut inboxes = Vec::with_capacity(input);
        inboxes.resize_with(input, MessageBuffer::new);
        Self {
            state: RwLock::new(MockTransportState {
                outputs: vec![PortState::Open; output],
                inputs: vec![PortState::Open; input],
                inboxes,
            }),
            wakeup: (Mutex::new(false), Condvar::new()),
        }
    }
}

impl Transport for MockTransport {
    fn state(&self, port: PortID) -> PortResult<PortState> {
        let state = self.state.read();
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

    fn open_input(&self) -> PortResult<InputPortID> {
        let mut state = self.state.write();
        state.inputs.push(PortState::Open);
        state.inboxes.push(MessageBuffer::new());

        InputPortID::try_from(-(state.inputs.len() as isize))
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let mut state = self.state.write();
        state.outputs.push(PortState::Open);

        OutputPortID::try_from(state.outputs.len() as isize)
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        let input_index = input.index();
        let mut state = self.state.upgradable_read();
        Ok(match state.inputs.get(input_index) {
            None => return Err(PortError::Invalid(input.into())),
            Some(input_state) => match input_state {
                PortState::Closed => false, // already closed
                PortState::Open => {
                    state.with_upgraded(|state| {
                        state.inputs[input_index] = PortState::Closed;
                    });
                    true
                }
                PortState::Connected(PortID::Output(output)) => {
                    let output_index = output.index();
                    debug_assert!(matches!(
                        state.outputs[output_index],
                        PortState::Connected(PortID::Input(_))
                    ));
                    state.with_upgraded(|state| {
                        state.outputs[output_index] = PortState::Open;
                        state.inputs[input_index] = PortState::Closed;
                        state.inboxes[input_index].clear();
                    });
                    true
                }
                PortState::Connected(PortID::Input(_)) => unreachable!(),
            },
        })
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        let output_index = output.index();
        let mut state = self.state.upgradable_read();
        Ok(match state.outputs.get(output_index) {
            None => return Err(PortError::Invalid(output.into())),
            Some(output_state) => match output_state {
                PortState::Closed => false, // already closed
                PortState::Open => {
                    state.with_upgraded(|state| {
                        state.outputs[output_index] = PortState::Closed;
                    });
                    true
                }
                PortState::Connected(PortID::Input(input)) => {
                    let input = input.clone();
                    let input_index = input.index();
                    debug_assert!(matches!(
                        state.inputs[input_index],
                        PortState::Connected(PortID::Output(_))
                    ));
                    state.with_upgraded(|state| {
                        state.outputs[output_index] = PortState::Closed;
                        state.inputs[input_index] = PortState::Open;
                    });
                    drop(state);
                    self.recv_notify(input); // wake up the receiving thread
                    true
                }
                PortState::Connected(PortID::Output(_)) => unreachable!(),
            },
        })
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let mut state = self.state.write();
        match (
            state.outputs.get(source.index()),
            state.inputs.get(target.index()),
        ) {
            (Some(PortState::Open), Some(PortState::Open)) => {
                state.outputs[source.index()] = PortState::Connected(PortID::Input(target));
                state.inputs[target.index()] = PortState::Connected(PortID::Output(source));
            }
            _ => return Err(PortError::Invalid(PortID::Output(source))), // TODO: better errors
        };
        Ok(true)
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        let input = {
            let state = self.state.read();
            match state.outputs.get(output.index()) {
                None => return Err(PortError::Invalid(PortID::Output(output))),
                Some(PortState::Closed) => return Err(PortError::Closed),
                Some(PortState::Open) => return Err(PortError::Disconnected),
                Some(PortState::Connected(PortID::Output(_))) => unreachable!(),
                Some(PortState::Connected(PortID::Input(input))) => *input,
            }
        };
        {
            let mut state = self.state.write();
            state.inboxes[input.index()].push(message);
        }
        self.recv_notify(input); // wake up the receiving thread
        Ok(())
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        let state = self.state.read();
        if state.inputs.get(input.index()).is_none() {
            return Err(PortError::Invalid(PortID::Input(input)));
        }
        drop(state);

        loop {
            let mut state = self.state.upgradable_read();
            if !state.inboxes[input.index()].is_empty() {
                return Ok(state.with_upgraded(|state| state.inboxes[input.index()].pop()));
            }
            if state.inputs[input.index()].is_closed() {
                return Ok(None);
            }
            drop(state);
            self.recv_wait(input); // sleep until something happens
        }
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!() // TODO: implement try_recv()
    }
}

impl MockTransport {
    fn recv_wait(&self, _input: InputPortID) {
        let (ref recv_lock, ref recv_cvar) = self.wakeup;
        let mut recv_guard = recv_lock.lock().unwrap();
        while !*recv_guard {
            recv_guard = recv_cvar.wait(recv_guard).unwrap(); // blocks the current thread
        }
        *recv_guard = false;
    }

    fn recv_notify(&self, _input: InputPortID) {
        let (ref recv_lock, ref recv_cvar) = self.wakeup;
        let mut recv_guard = recv_lock.lock().unwrap();
        *recv_guard = true;
        recv_cvar.notify_all();
    }
}

impl MockTransportState {}
