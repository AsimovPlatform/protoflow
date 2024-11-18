// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

extern crate std;

use protoflow_core::{
    prelude::{BTreeMap, Bytes},
    InputPortID, OutputPortID, PortError, PortResult, PortState, Transport,
};

use parking_lot::{Mutex, RwLock};
use std::sync::mpsc::{Receiver, SyncSender};
use zeromq::{util::PeerIdentity, Socket, SocketOptions, SocketRecv, SocketSend};

pub struct ZMQTransport {
    sock: Mutex<zeromq::RouterSocket>,
    tokio: tokio::runtime::Handle,
    outputs: BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>,
    inputs: BTreeMap<InputPortID, RwLock<ZmqInputPortState>>,
}

#[derive(Debug, Clone, Default)]
enum ZmqOutputPortState {
    #[default]
    Open,
    Connected(SyncSender<ZmqTransportEvent>),
    Closed,
}

#[derive(Debug, Default)]
enum ZmqInputPortState {
    #[default]
    Open,
    Connected(Mutex<Receiver<ZmqTransportEvent>>),
    Closed,
}

#[derive(Clone, Debug)]
enum ZmqTransportEvent {
    Connect,
    Message(Bytes),
    Disconnect,
}

impl ZMQTransport {
    pub fn new(url: &str) -> Self {
        let tokio = tokio::runtime::Handle::current();

        let peer_id = PeerIdentity::new();
        let mut sock_opts = SocketOptions::default();
        sock_opts.peer_identity(peer_id);

        let mut sock = zeromq::RouterSocket::with_options(sock_opts);
        tokio
            .block_on(sock.connect(url))
            .expect("failed to connect");
        let sock = Mutex::new(sock);

        let outputs = BTreeMap::default();
        let inputs = BTreeMap::default();

        Self {
            sock,
            tokio,
            outputs,
            inputs,
        }
    }
}

impl Transport for ZMQTransport {
    fn input_state(&self, input: InputPortID) -> PortResult<PortState> {
        todo!();
    }

    fn output_state(&self, output: OutputPortID) -> PortResult<PortState> {
        todo!();
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        todo!();
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        todo!();
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        todo!();
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        todo!();
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        todo!();
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        let Some(output_state) = self.outputs.get(&output) else {
            todo!();
        };

        use ZmqOutputPortState::*;
        match *output_state.read() {
            Open => todo!(),
            Closed => todo!(),
            Connected(ref sender) => {
                let msg = ZmqTransportEvent::Message(message);
                Ok(sender.send(msg).unwrap())
            }
        }
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        let Some(input_state) = self.inputs.get(&input) else {
            todo!();
        };

        use ZmqInputPortState::*;
        match *input_state.read() {
            Open => todo!(),
            Closed => todo!(),
            Connected(ref receiver) => {
                use ZmqTransportEvent::*;
                let receiver = receiver.lock();
                match receiver.recv().map_err(|_| PortError::Disconnected)? {
                    Connect => todo!(),
                    Disconnect => todo!(),
                    Message(bytes) => Ok(Some(bytes)),
                }
            }
        }
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!();
    }
}
