// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{Bytes, ToString},
    transport::Transport,
    InputPortID, OutputPortID, PortError, PortID, PortResult, PortState,
};
use parking_lot::{Mutex, RwLock};
use sharded_slab::Slab;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

//pub(crate) const DEFAULT_INPUT_PORT_COUNT: usize = 16;
//pub(crate) const DEFAULT_OUTPUT_PORT_COUNT: usize = 16;
pub(crate) const DEFAULT_CONNECTION_CAPACITY: usize = 1;

#[derive(Debug, Default)]
pub struct MpscTransport {
    outputs: Slab<MpscTransportOutputPort>,
    inputs: Slab<MpscTransportInputPort>,
}

#[derive(Debug, Default)]
pub struct MpscTransportOutputPort {
    state: RwLock<PortState>,
}

#[derive(Debug)]
pub struct MpscTransportInputPort {
    state: RwLock<PortState>,
    receiver: Mutex<Receiver<MpscTransportEvent>>,
    sender: SyncSender<MpscTransportEvent>,
}

#[derive(Debug)]
pub enum MpscTransportEvent {
    Connect,
    Message(Bytes),
    Disconnect,
}

impl MpscTransport {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Transport for MpscTransport {
    fn state(&self, port: PortID) -> PortResult<PortState> {
        match port {
            PortID::Input(input) => match self.inputs.get(input.index()) {
                None => Err(PortError::Invalid(port)),
                Some(entry) => Ok(*entry.state.read()),
            },
            PortID::Output(output) => match self.outputs.get(output.index()) {
                None => Err(PortError::Invalid(port)),
                Some(entry) => Ok(*entry.state.read()),
            },
        }
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        let (sender, receiver) = sync_channel(DEFAULT_CONNECTION_CAPACITY);

        let input_index = self
            .inputs
            .insert(MpscTransportInputPort {
                state: RwLock::new(PortState::Open),
                receiver: Mutex::new(receiver),
                sender,
            })
            .unwrap();

        InputPortID::try_from(-(input_index as isize + 1))
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let output_index = self
            .outputs
            .insert(MpscTransportOutputPort {
                state: RwLock::new(PortState::Open),
            })
            .unwrap();

        OutputPortID::try_from(output_index as isize + 1)
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        let input_index = input.index();
        let Some(input_entry) = self.inputs.get(input_index) else {
            return Err(PortError::Invalid(input.into()));
        };

        let input_state = *input_entry.state.read();
        Ok(match input_state {
            PortState::Closed => false, // already closed
            PortState::Open => {
                *input_entry.state.write() = PortState::Closed;
                true
            }
            PortState::Connected(PortID::Output(output)) => {
                let output_index = output.index();
                let Some(output_entry) = self.outputs.get(output_index) else {
                    return Err(PortError::Invalid(output.into()));
                };
                debug_assert!(matches!(
                    *output_entry.state.read(),
                    PortState::Connected(PortID::Input(_))
                ));
                let sender = input_entry.sender.clone();
                *output_entry.state.write() = PortState::Open;
                *input_entry.state.write() = PortState::Closed;
                sender.send(MpscTransportEvent::Disconnect).unwrap(); // blocking
                true
            }
            PortState::Connected(PortID::Input(_)) => unreachable!(),
        })
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        let output_index = output.index();
        let Some(output_entry) = self.outputs.get(output_index) else {
            return Err(PortError::Invalid(output.into()));
        };

        let output_state = *output_entry.state.read();
        Ok(match output_state {
            PortState::Closed => false, // already closed
            PortState::Open => {
                *output_entry.state.write() = PortState::Closed;
                true
            }
            PortState::Connected(PortID::Input(input)) => {
                let input = input.clone();
                let input_index = input.index();
                let Some(input_entry) = self.inputs.get(input_index) else {
                    return Err(PortError::Invalid(input.into()));
                };
                debug_assert!(matches!(
                    *input_entry.state.read(),
                    PortState::Connected(PortID::Output(_))
                ));
                let sender = input_entry.sender.clone();
                sender.send(MpscTransportEvent::Disconnect).unwrap(); // blocking
                *output_entry.state.write() = PortState::Closed;
                *input_entry.state.write() = PortState::Open;
                true
            }
            PortState::Connected(PortID::Output(_)) => unreachable!(),
        })
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let Some(output_entry) = self.outputs.get(source.index()) else {
            return Err(PortError::Invalid(source.into()));
        };

        let Some(input_entry) = self.inputs.get(target.index()) else {
            return Err(PortError::Invalid(target.into()));
        };

        let output_state = *output_entry.state.read();
        let input_state = *input_entry.state.read();
        match (output_state, input_state) {
            (PortState::Open, PortState::Open) => {
                // FIXME: this has multiple race conditions right now:
                *output_entry.state.write() = PortState::Connected(PortID::Input(target));
                *input_entry.state.write() = PortState::Connected(PortID::Output(source));
            }
            _ => return Err(PortError::Invalid(source.into())), // TODO: better errors
        };

        Ok(true)
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        let Some(output_entry) = self.outputs.get(output.index()) else {
            return Err(PortError::Invalid(output.into()));
        };

        let input = match *output_entry.state.read() {
            PortState::Closed => return Err(PortError::Closed),
            PortState::Open => return Err(PortError::Disconnected),
            PortState::Connected(PortID::Output(_)) => unreachable!(),
            PortState::Connected(PortID::Input(input)) => input,
        };
        let Some(input_entry) = self.inputs.get(input.index()) else {
            return Err(PortError::Invalid(input.into()));
        };

        let sender = input_entry.sender.clone();
        Ok(sender.send(MpscTransportEvent::Message(message)).unwrap()) // blocking (TODO: error handling)
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        let Some(input_entry) = self.inputs.get(input.index()) else {
            return Err(PortError::Invalid(input.into()));
        };

        if input_entry.state.read().is_closed() {
            return Ok(None); // EOS
        }

        let receiver = input_entry.receiver.lock();

        use MpscTransportEvent::*;
        match receiver
            .recv() // blocking
            .map_err(|_| PortError::Disconnected)?
        {
            Connect => unreachable!(),
            Message(bytes) => Ok(Some(bytes)),
            Disconnect => Ok(None), // EOS
        }
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!() // TODO: implement try_recv()
    }
}
