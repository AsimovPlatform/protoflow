// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

extern crate std;

use protoflow_core::{
    prelude::{BTreeMap, Bytes, ToString},
    InputPortID, OutputPortID, PortError, PortResult, PortState, Transport,
};

use parking_lot::{Mutex, RwLock};
use std::{
    fmt::{self, Write},
    sync::mpsc::{Receiver, SyncSender},
    write,
};
use zeromq::{util::PeerIdentity, Socket, SocketOptions};

const DEFAULT_PUB_SOCKET: &str = "tcp://127.0.0.1:10000";
const DEFAULT_SUB_SOCKET: &str = "tcp://127.0.0.1:10001";

pub struct ZmqTransport {
    tokio: tokio::runtime::Handle,

    psock: Mutex<zeromq::PubSocket>,
    ssock: Mutex<zeromq::SubSocket>,

    outputs: RwLock<BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>>,
    inputs: RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
}

#[derive(Debug, Default)]
enum ZmqOutputPortState {
    #[default]
    Open,
    Connected(
        Mutex<Receiver<ZmqTransportEvent>>,
        SyncSender<ZmqTransportEvent>,
    ),
    Closed,
}

#[derive(Debug, Default)]
enum ZmqInputPortState {
    #[default]
    Open,
    Connected(
        Mutex<Receiver<ZmqTransportEvent>>,
        SyncSender<ZmqTransportEvent>,
    ),
    Closed,
}

type SequenceID = u64;

#[derive(Clone, Debug)]
enum ZmqTransportEvent {
    Connect(OutputPortID, InputPortID),
    AckConnection(OutputPortID, InputPortID),
    Message(OutputPortID, InputPortID, SequenceID, Bytes),
    AckMessage(OutputPortID, InputPortID, SequenceID),
    CloseOutput(OutputPortID, InputPortID),
    CloseInput(InputPortID),
}

impl ZmqTransportEvent {
    fn write_topic<W: Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        use ZmqTransportEvent::*;
        match self {
            Connect(o, i) => write!(f, "{}:conn:{}", i, o),
            AckConnection(o, i) => write!(f, "{}:ackConn:{}", i, o),
            Message(o, i, seq, _payload) => write!(f, "{}:msg:{}:{}", i, o, seq),
            AckMessage(o, i, seq) => write!(f, "{}:ackMsg:{}:{}", i, o, seq),
            CloseOutput(o, i) => write!(f, "{}:closeOut:{}", i, o),
            CloseInput(i) => write!(f, "{}:closeIn", i),
        }
    }
}

impl Default for ZmqTransport {
    fn default() -> Self {
        Self::new(DEFAULT_PUB_SOCKET, DEFAULT_SUB_SOCKET)
    }
}

impl ZmqTransport {
    pub fn new(pub_url: &str, sub_url: &str) -> Self {
        let tokio = tokio::runtime::Handle::current();

        let peer_id = PeerIdentity::new();

        let psock = {
            let peer_id = peer_id.clone();
            let mut sock_opts = SocketOptions::default();
            sock_opts.peer_identity(peer_id);

            let mut psock = zeromq::PubSocket::with_options(sock_opts);
            tokio
                .block_on(psock.connect(pub_url))
                .expect("failed to connect PUB");
            Mutex::new(psock)
        };

        let ssock = {
            let mut sock_opts = SocketOptions::default();
            sock_opts.peer_identity(peer_id);

            let mut ssock = zeromq::SubSocket::with_options(sock_opts);
            tokio
                .block_on(ssock.connect(sub_url))
                .expect("failed to connect SUB");
            Mutex::new(ssock)
        };

        let outputs = RwLock::new(BTreeMap::default());
        let inputs = RwLock::new(BTreeMap::default());

        Self {
            psock,
            ssock,
            tokio,
            outputs,
            inputs,
        }
    }
}

impl Transport for ZmqTransport {
    fn input_state(&self, input: InputPortID) -> PortResult<PortState> {
        use ZmqInputPortState::*;
        match self.inputs.read().get(&input) {
            Some(input) => match *input.read() {
                Open => Ok(PortState::Open),
                Connected(_, _) => Ok(PortState::Connected),
                Closed => Ok(PortState::Closed),
            },
            None => Err(PortError::Invalid(input.into())),
        }
    }

    fn output_state(&self, output: OutputPortID) -> PortResult<PortState> {
        use ZmqOutputPortState::*;
        match self.outputs.read().get(&output) {
            Some(output) => match *output.read() {
                Open => Ok(PortState::Open),
                Connected(_, _) => Ok(PortState::Connected),
                Closed => Ok(PortState::Closed),
            },
            None => Err(PortError::Invalid(output.into())),
        }
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        let mut inputs = self.inputs.write();

        let new_id = InputPortID::try_from(-(inputs.len() as isize + 1))
            .map_err(|e| PortError::Other(e.to_string()))?;

        let state = RwLock::new(ZmqInputPortState::Open);
        inputs.insert(new_id, state);

        // TODO: start worker

        Ok(new_id)
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let mut outputs = self.outputs.write();

        let new_id = OutputPortID::try_from(outputs.len() as isize + 1)
            .map_err(|e| PortError::Other(e.to_string()))?;

        let state = RwLock::new(ZmqOutputPortState::Open);
        outputs.insert(new_id, state);

        // TODO: start worker

        Ok(new_id)
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        let inputs = self.inputs.read();

        let Some(state) = inputs.get(&input) else {
            return Err(PortError::Invalid(input.into()));
        };

        let mut state = state.write();

        let ZmqInputPortState::Connected(_, _) = *state else {
            return Err(PortError::Disconnected);
        };

        // TODO: send message to worker

        *state = ZmqInputPortState::Closed;

        Ok(true)
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        let outputs = self.outputs.read();

        let Some(state) = outputs.get(&output) else {
            return Err(PortError::Invalid(output.into()));
        };

        let mut state = state.write();

        let ZmqOutputPortState::Connected(_, _) = *state else {
            return Err(PortError::Disconnected);
        };

        // TODO: send message to worker

        *state = ZmqOutputPortState::Closed;

        Ok(true)
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let Some(output) = self.outputs.read().get(&source) else {
            return Err(PortError::Invalid(source.into()));
        };

        let Some(input) = self.inputs.read().get(&target) else {
            return Err(PortError::Invalid(target.into()));
        };

        Ok(true)
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        todo!();
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!();
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use protoflow_core::System;

    use futures_util::future::TryFutureExt;
    use zeromq::{PubSocket, SocketRecv, SocketSend, SubSocket};

    fn start_zmqtransport_server(rt: &tokio::runtime::Runtime) {
        // bind a *SUB* socket to the *PUB* address so that the transport can *PUB* to it
        let mut pub_srv = SubSocket::new();
        rt.block_on(pub_srv.bind(DEFAULT_PUB_SOCKET)).unwrap();

        // bind a *PUB* socket to the *SUB* address so that the transport can *SUB* to it
        let mut sub_srv = PubSocket::new();
        rt.block_on(sub_srv.bind(DEFAULT_SUB_SOCKET)).unwrap();

        // subscribe to all messages
        rt.block_on(pub_srv.subscribe("")).unwrap();

        // resend anything received on the *SUB* socket to the *PUB* socket
        tokio::task::spawn(async move {
            let mut pub_srv = pub_srv;
            loop {
                pub_srv
                    .recv()
                    .and_then(|msg| sub_srv.send(msg))
                    .await
                    .unwrap();
            }
        });
    }

    #[test]
    fn implementation_matches() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _guard = rt.enter();

        //zeromq::proxy(frontend, backend, capture)
        start_zmqtransport_server(&rt);

        let _ = System::<ZmqTransport>::build(|_s| { /* do nothing */ });
    }
}
