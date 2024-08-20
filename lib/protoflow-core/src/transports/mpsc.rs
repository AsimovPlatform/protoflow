// This is free and unencumbered software released into the public domain.

mod event;
use event::*;

mod input;
use input::*;

mod output;
use output::*;

extern crate std;

use crate::{
    prelude::{Bytes, ToString},
    transport::Transport,
    InputPortID, OutputPortID, PortError, PortResult, PortState,
};
use parking_lot::{Mutex, RwLock};
use sharded_slab::Slab;
use std::sync::mpsc::sync_channel;

pub(crate) const DEFAULT_CONNECTION_CAPACITY: usize = 1;

#[derive(Debug, Default)]
pub struct MpscTransport {
    outputs: Slab<RwLock<MpscTransportOutputPortState>>,
    inputs: Slab<RwLock<MpscTransportInputPortState>>,
}

impl MpscTransport {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Transport for MpscTransport {
    fn input_state(&self, input: InputPortID) -> PortResult<PortState> {
        match self.inputs.get(input.index()) {
            None => Err(PortError::Invalid(input.into())),
            Some(entry) => Ok(entry.read().state()),
        }
    }

    fn output_state(&self, output: OutputPortID) -> PortResult<PortState> {
        match self.outputs.get(output.index()) {
            None => Err(PortError::Invalid(output.into())),
            Some(entry) => Ok(entry.read().state()),
        }
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        let input_index = self
            .inputs
            .insert(RwLock::new(MpscTransportInputPortState::Open))
            .unwrap();

        InputPortID::try_from(-(input_index as isize + 1))
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let output_index = self
            .outputs
            .insert(RwLock::new(MpscTransportOutputPortState::Open))
            .unwrap();

        OutputPortID::try_from(output_index as isize + 1)
            .map_err(|s| PortError::Other(s.to_string()))
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        let Some(input_entry) = self.inputs.get(input.index()) else {
            return Err(PortError::Invalid(input.into()));
        };
        let mut input_state = input_entry.write();

        use MpscTransportInputPortState::*;
        Ok(match *input_state {
            Closed => false, // already closed
            Open | Connected(_) => {
                *input_state = MpscTransportInputPortState::Closed;
                true
            }
        })
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        let Some(output_entry) = self.outputs.get(output.index()) else {
            return Err(PortError::Invalid(output.into()));
        };
        let mut output_state = output_entry.write();

        use MpscTransportOutputPortState::*;
        Ok(match *output_state {
            Closed => false, // already closed
            Open => {
                *output_state = MpscTransportOutputPortState::Closed;
                true
            }
            Connected(ref sender) => {
                let sender = sender.clone();
                *output_state = MpscTransportOutputPortState::Closed;
                drop(output_state);
                sender.send(MpscTransportEvent::Disconnect).unwrap(); // blocking
                true
            }
        })
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let Some(output_entry) = self.outputs.get(source.index()) else {
            return Err(PortError::Invalid(source.into()));
        };

        let Some(input_entry) = self.inputs.get(target.index()) else {
            return Err(PortError::Invalid(target.into()));
        };

        let mut output_state = output_entry.write();
        let mut input_state = input_entry.write();
        if !output_state.state().is_open() && !input_state.state().is_open() {
            return Err(PortError::Other("connect".to_string())); // TODO: better errors
        }

        let (sender, receiver) = sync_channel(DEFAULT_CONNECTION_CAPACITY);
        *output_state = MpscTransportOutputPortState::Connected(sender);
        *input_state = MpscTransportInputPortState::Connected(Mutex::new(receiver));
        Ok(true)
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        let Some(output_entry) = self.outputs.get(output.index()) else {
            return Err(PortError::Invalid(output.into()));
        };
        let output_state = output_entry.read();

        use MpscTransportOutputPortState::*;
        match *output_state {
            Closed => return Err(PortError::Closed),
            Open => return Err(PortError::Disconnected),
            Connected(ref sender) => {
                let sender = sender.clone();
                Ok(sender.send(MpscTransportEvent::Message(message)).unwrap()) // blocking (TODO: error handling)
            }
        }
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        let Some(input_entry) = self.inputs.get(input.index()) else {
            return Err(PortError::Invalid(input.into()));
        };
        let input_state = input_entry.read();

        use MpscTransportInputPortState::*;
        match *input_state {
            Closed => return Ok(None), // EOS
            Open => return Ok(None),   // FIXME
            Connected(ref receiver) => {
                use MpscTransportEvent::*;
                let receiver = receiver.lock();
                match receiver
                    .recv() // blocking
                    .map_err(|_| PortError::Disconnected)?
                {
                    Connect => unreachable!(),
                    Message(bytes) => Ok(Some(bytes)),
                    Disconnect => {
                        drop(receiver);
                        drop(input_state);
                        let mut input_state = input_entry.write();
                        *input_state = Closed;
                        Ok(None) // EOS
                    }
                }
            }
        }
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!() // TODO: implement try_recv()
    }
}
