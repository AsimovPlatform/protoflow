// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{Bytes, ToString, Vec},
    transport::Transport,
    InputPortID, OutputPortID, PortError, PortID, PortResult, PortState,
};
use parking_lot::RwLock;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub(crate) const DEFAULT_INPUT_PORT_COUNT: usize = 16;
pub(crate) const DEFAULT_OUTPUT_PORT_COUNT: usize = 16;
pub(crate) const DEFAULT_CONNECTION_CAPACITY: usize = 1;

#[derive(Debug, Default)]
pub struct MpscTransport {
    pub state: RwLock<MpscTransportState>,
}

#[derive(Debug, Default)]
pub struct MpscTransportState {
    outputs: Vec<PortState>,
    inputs: Vec<PortState>,
    channels: Vec<(SyncSender<MpscTransportEvent>, Receiver<MpscTransportEvent>)>,
}

#[derive(Debug)]
pub enum MpscTransportEvent {
    Connect,
    Message(Bytes),
    Disconnect,
}

unsafe impl Sync for MpscTransportState {}

impl MpscTransport {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(MpscTransportState::new()),
        }
    }
}

impl MpscTransportState {
    pub fn new() -> Self {
        // Avoid reallocations by pre-allocating an ample default capacity.
        Self {
            outputs: Vec::with_capacity(DEFAULT_OUTPUT_PORT_COUNT),
            inputs: Vec::with_capacity(DEFAULT_INPUT_PORT_COUNT),
            channels: Vec::with_capacity(DEFAULT_INPUT_PORT_COUNT),
        }
    }
}

impl Transport for MpscTransport {
    fn state(&self, port: PortID) -> PortResult<PortState> {
        let state = self.state.read();
        match port {
            PortID::Input(input) => match state.inputs.get(input.index()) {
                None => Err(PortError::Invalid(port)),
                Some(state) => Ok(*state),
            },
            PortID::Output(output) => match state.outputs.get(output.index()) {
                None => Err(PortError::Invalid(port)),
                Some(state) => Ok(*state),
            },
        }
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        let mut state = self.state.write();
        state.inputs.push(PortState::Open);
        state
            .channels
            .push(sync_channel(DEFAULT_CONNECTION_CAPACITY));

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
                    let sender = state.channels[input_index].0.clone();
                    state.with_upgraded(|state| {
                        state.outputs[output_index] = PortState::Open;
                        state.inputs[input_index] = PortState::Closed;
                    });
                    drop(state);
                    sender.send(MpscTransportEvent::Disconnect).unwrap(); // blocking
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
                    let sender = state.channels[input_index].0.clone();
                    sender.send(MpscTransportEvent::Disconnect).unwrap(); // blocking
                    state.with_upgraded(|state| {
                        state.outputs[output_index] = PortState::Closed;
                        state.inputs[input_index] = PortState::Open;
                    });
                    drop(state);
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
        let state = self.state.read();
        let input = {
            match state.outputs.get(output.index()) {
                None => return Err(PortError::Invalid(PortID::Output(output))),
                Some(PortState::Closed) => return Err(PortError::Closed),
                Some(PortState::Open) => return Err(PortError::Disconnected),
                Some(PortState::Connected(PortID::Output(_))) => unreachable!(),
                Some(PortState::Connected(PortID::Input(input))) => *input,
            }
        };
        let sender = state.channels[input.index()].0.clone();
        drop(state);
        Ok(sender.send(MpscTransportEvent::Message(message)).unwrap()) // blocking (TODO: error handling)
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        let state = self.state.read();
        if state.inputs.get(input.index()).is_none() {
            return Err(PortError::Invalid(PortID::Input(input)));
        }
        if state.inputs[input.index()].is_closed() {
            return Ok(None); // EOS
        }
        let receiver = &state.channels[input.index()].1;
        let event = receiver
            .recv() // blocking
            .map_err(|_| PortError::Disconnected)?;
        use MpscTransportEvent::*;
        match event {
            Connect => unreachable!(),
            Message(bytes) => Ok(Some(bytes)),
            Disconnect => Ok(None), // EOS
        }
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!() // TODO: implement try_recv()
    }
}
