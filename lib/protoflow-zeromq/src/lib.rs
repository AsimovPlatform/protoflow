// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

extern crate std;

use protoflow_core::{
    prelude::{BTreeMap, Bytes, ToString, Vec},
    InputPortID, OutputPortID, PortError, PortResult, PortState, Transport,
};

use parking_lot::{Mutex, RwLock};
use std::{
    fmt::{self, Write},
    format,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    write,
};
use zeromq::{util::PeerIdentity, Socket, SocketOptions, ZmqError};

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
        SyncSender<ZmqTransportEvent>,
        Mutex<Receiver<ZmqOutputPortEvent>>,
        InputPortID,
    ),
    Closed,
}

impl ZmqOutputPortState {
    fn state(&self) -> PortState {
        use ZmqOutputPortState::*;
        match self {
            Open => PortState::Open,
            Connected(_, _, _) => PortState::Connected,
            Closed => PortState::Closed,
        }
    }
}

#[derive(Debug)]
enum ZmqInputPortState {
    Open(
        SyncSender<ZmqTransportEvent>,
        Mutex<Receiver<ZmqInputPortEvent>>,
    ),
    Connected(
        SyncSender<ZmqTransportEvent>,
        Mutex<Receiver<ZmqInputPortEvent>>,
        Vec<OutputPortID>,
    ),
    Closed,
}

impl ZmqInputPortState {
    fn state(&self) -> PortState {
        use ZmqInputPortState::*;
        match self {
            Open(_, _) => PortState::Open,
            Connected(_, _, _) => PortState::Connected,
            Closed => PortState::Closed,
        }
    }
}

type SequenceID = u64;

/// ZmqTransportEvent represents the data that goes over the wire, sent from an output port over
/// the network to an input port.
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

/// ZmqOutputPortEvent represents events that we receive from the background worker of the port.
#[derive(Clone, Debug)]
enum ZmqOutputPortEvent {
    Opened,
    Connected(InputPortID),
    Message(Bytes),
    Closed,
}

/// ZmqInputPortEvent represents events that we receive from the background worker of the port.
#[derive(Clone, Debug)]
enum ZmqInputPortEvent {
    Opened,
    Connected(OutputPortID),
    Message(Bytes),
    Closed,
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

    fn subscribe_for_input_port(
        &self,
        input: InputPortID,
    ) -> Result<(SyncSender<ZmqTransportEvent>, Receiver<ZmqInputPortEvent>), ZmqError> {
        // TODO: only sub to relevant events
        let topic = format!("{}:", input);
        self.tokio.block_on(self.ssock.lock().subscribe(&topic))?;
        let (from_worker_send, from_worker_recv) = sync_channel(1);
        let (to_worker_send, to_worker_recv) = sync_channel(1);

        // Input worker loop:
        //   1. Receive connection attempts and respond
        //   2. Receive messages and forward to channel
        //   3. Receive and handle disconnects
        tokio::task::spawn(async {
            let (output, input) = (from_worker_send, to_worker_recv);
            loop {
                todo!();
            }
        });

        Ok((to_worker_send, from_worker_recv))
    }
}

impl Transport for ZmqTransport {
    fn input_state(&self, input: InputPortID) -> PortResult<PortState> {
        self.inputs
            .read()
            .get(&input)
            .map(|port| port.read().state())
            .ok_or(PortError::Invalid(input.into()))
    }

    fn output_state(&self, output: OutputPortID) -> PortResult<PortState> {
        self.outputs
            .read()
            .get(&output)
            .map(|port| port.read().state())
            .ok_or(PortError::Invalid(output.into()))
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        let mut inputs = self.inputs.write();

        let new_id = InputPortID::try_from(-(inputs.len() as isize + 1))
            .map_err(|e| PortError::Other(e.to_string()))?;

        let (sender, receiver) = self
            .subscribe_for_input_port(new_id)
            .map_err(|e| PortError::Other(e.to_string()))?;

        let state = RwLock::new(ZmqInputPortState::Open(sender, Mutex::new(receiver)));
        inputs.insert(new_id, state);

        Ok(new_id)
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let mut outputs = self.outputs.write();

        let new_id = OutputPortID::try_from(outputs.len() as isize + 1)
            .map_err(|e| PortError::Other(e.to_string()))?;

        let state = RwLock::new(ZmqOutputPortState::Open);
        outputs.insert(new_id, state);

        Ok(new_id)
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        let inputs = self.inputs.read();

        let Some(state) = inputs.get(&input) else {
            return Err(PortError::Invalid(input.into()));
        };

        let state = state.read();

        let ZmqInputPortState::Connected(sender, receiver, _) = &*state else {
            return Err(PortError::Disconnected);
        };

        sender
            .send(ZmqTransportEvent::CloseInput(input))
            .map_err(|e| PortError::Other(e.to_string()))?;

        loop {
            let msg = receiver
                .lock()
                .recv()
                .map_err(|e| PortError::Other(e.to_string()))?;
            use ZmqInputPortEvent::*;
            match msg {
                Closed => break Ok(true),
                _ => continue, // TODO
            };
        }
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        let outputs = self.outputs.read();

        let Some(state) = outputs.get(&output) else {
            return Err(PortError::Invalid(output.into()));
        };

        let state = state.write();

        let ZmqOutputPortState::Connected(sender, receiver, input) = &*state else {
            return Err(PortError::Disconnected);
        };

        sender
            .send(ZmqTransportEvent::CloseOutput(output, *input))
            .map_err(|e| PortError::Other(e.to_string()))?;

        loop {
            let msg = receiver
                .lock()
                .recv()
                .map_err(|e| PortError::Other(e.to_string()))?;
            use ZmqOutputPortEvent::*;
            match msg {
                Closed => break Ok(true),
                _ => continue, // TODO
            }
        }
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let outputs = self.outputs.read();
        let Some(output) = outputs.get(&source) else {
            return Err(PortError::Invalid(source.into()));
        };

        let inputs = self.inputs.read();
        let Some(input) = inputs.get(&target) else {
            return Err(PortError::Invalid(target.into()));
        };

        //let mut output = output.write();
        //if !output.state().is_open() {
        //    return Err(PortError::Invalid(source.into()));
        //}
        //
        //let mut input = input.write();
        //if !input.state().is_open() {
        //    return Err(PortError::Invalid(source.into()));
        //}

        // TODO: send from output, receive and respond from input

        //let (out_recv, out_send) = {
        //    let (from_worker_send, from_worker_recv) = sync_channel::<ZmqOutputPortEvent>(1);
        //    let (to_worker_send, to_worker_recv) = sync_channel::<ZmqTransportEvent>(1);
        //
        //    tokio::task::spawn(async {
        //        let (output, input) = (from_worker_send, to_worker_recv);
        //        loop {
        //            tokio::time::sleep(Duration::from_secs(1)).await;
        //        }
        //    });
        //
        //    (from_worker_recv, to_worker_send)
        //};

        let (from_worker_send, from_worker_recv) = sync_channel::<ZmqOutputPortEvent>(1);
        let (to_worker_send, to_worker_recv) = sync_channel::<ZmqTransportEvent>(1);

        // Output worker loop:
        //   1. Send connection attempts
        //   2. Send messages
        //     2.1 Wait for ACK
        //     2.2. Resend on timeout
        //   3. Send disconnect events
        tokio::task::spawn(async {
            let (output, input) = (from_worker_send, to_worker_recv);
            loop {
                todo!();
            }
        });

        loop {
            let msg = from_worker_recv
                .recv()
                .map_err(|e| PortError::Other(e.to_string()))?;
            use ZmqOutputPortEvent::*;
            match msg {
                Connected(_) => break Ok(true),
                _ => continue, // TODO
            }
        }
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
