// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

extern crate std;

use protoflow_core::{
    prelude::{vec, Arc, BTreeMap, Bytes, String, ToString, Vec},
    InputPortID, OutputPortID, PortError, PortResult, PortState, Transport,
};

use core::fmt::Error;
use parking_lot::{Mutex, RwLock};
use std::{format, write};
use tokio::sync::mpsc::{channel as sync_channel, Receiver, Sender};
use zeromq::{util::PeerIdentity, Socket, SocketOptions, SocketRecv, SocketSend, ZmqMessage};

const DEFAULT_PUB_SOCKET: &str = "tcp://127.0.0.1:10000";
const DEFAULT_SUB_SOCKET: &str = "tcp://127.0.0.1:10001";

pub struct ZmqTransport {
    tokio: tokio::runtime::Handle,

    pub_queue: Sender<ZmqTransportEvent>,
    sub_queue: Sender<ZmqSubscriptionRequest>,

    outputs: Arc<RwLock<BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>>>,
    inputs: Arc<RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>>,
}

#[derive(Debug, Clone)]
enum ZmqOutputPortState {
    Open(
        Sender<(InputPortID, Sender<Result<(), PortError>>)>,
        Sender<ZmqTransportEvent>,
    ),
    Connected(
        // channel for public send, contained channel is for the ack back
        Sender<(ZmqOutputPortRequest, Sender<Result<(), PortError>>)>,
        // internal channel for events
        Sender<ZmqTransportEvent>,
        // id of the connected input port
        InputPortID,
    ),
    Closed,
}

#[derive(Debug, Clone)]
enum ZmqOutputPortRequest {
    Close,
    Send(Bytes),
}

impl ZmqOutputPortState {
    fn state(&self) -> PortState {
        use ZmqOutputPortState::*;
        match self {
            Open(_, _) => PortState::Open,
            Connected(_, _, _) => PortState::Connected,
            Closed => PortState::Closed,
        }
    }
}

#[derive(Debug, Clone)]
enum ZmqInputPortState {
    Open(
        // TODO: hide these
        Sender<ZmqTransportEvent>,
    ),
    Connected(
        // channel for requests from public close
        Sender<(ZmqInputPortRequest, Sender<Result<(), PortError>>)>,
        // channel for the public recv
        Sender<ZmqInputPortEvent>,
        Arc<Mutex<Receiver<ZmqInputPortEvent>>>,
        // internal  channel for events
        Sender<ZmqTransportEvent>,
        // vec of the connected port ids
        Vec<OutputPortID>,
    ),
    Closed,
}

#[derive(Debug, Clone)]
enum ZmqInputPortRequest {
    Close,
}

impl ZmqInputPortState {
    fn state(&self) -> PortState {
        use ZmqInputPortState::*;
        match self {
            Open(_) => PortState::Open,
            Connected(_, _, _, _, _) => PortState::Connected,
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
    fn write_topic<W: std::io::Write + ?Sized>(&self, f: &mut W) -> Result<(), std::io::Error> {
        use ZmqTransportEvent::*;
        match self {
            Connect(o, i) => write!(f, "{}:conn:{}", i, o),
            AckConnection(o, i) => write!(f, "{}:ackConn:{}", i, o),
            Message(o, i, seq, _) => write!(f, "{}:msg:{}:{}", i, o, seq),
            AckMessage(o, i, seq) => write!(f, "{}:ackMsg:{}:{}", i, o, seq),
            CloseOutput(o, i) => write!(f, "{}:closeOut:{}", i, o),
            CloseInput(i) => write!(f, "{}:closeIn", i),
        }
    }
}

impl From<ZmqTransportEvent> for ZmqMessage {
    fn from(value: ZmqTransportEvent) -> Self {
        let mut topic = Vec::new();
        value.write_topic(&mut topic).unwrap();

        // first frame of the message is the topic
        let mut msg = ZmqMessage::from(topic.clone());

        // second frame of the message is the payload
        use ZmqTransportEvent::*;
        match value {
            Connect(_, _) => todo!(),
            AckConnection(_, _) => todo!(),
            Message(_, _, _, bytes) => msg.push_back(bytes),
            AckMessage(_, _, _) => todo!(),
            CloseOutput(_, _) => todo!(),
            CloseInput(_) => todo!(),
        };

        msg
    }
}

impl TryFrom<ZmqMessage> for ZmqTransportEvent {
    type Error = protoflow_core::DecodeError;

    fn try_from(_value: ZmqMessage) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// ZmqInputPortEvent represents events that we receive from the background worker of the port.
#[derive(Clone, Debug)]
enum ZmqInputPortEvent {
    Message(Bytes),
    Closed,
}

impl Default for ZmqTransport {
    fn default() -> Self {
        Self::new(DEFAULT_PUB_SOCKET, DEFAULT_SUB_SOCKET)
    }
}

#[derive(Clone)]
enum ZmqSubscriptionRequest {
    Subscribe(String),
    Unsubscribe(String),
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
            psock
        };

        let ssock = {
            let mut sock_opts = SocketOptions::default();
            sock_opts.peer_identity(peer_id);

            let mut ssock = zeromq::SubSocket::with_options(sock_opts);
            tokio
                .block_on(ssock.connect(sub_url))
                .expect("failed to connect SUB");
            ssock
        };

        let outputs = Arc::new(RwLock::new(BTreeMap::default()));
        let inputs = Arc::new(RwLock::new(BTreeMap::default()));

        let (pub_queue, pub_queue_recv) = sync_channel(1);

        let (sub_queue, sub_queue_recv) = tokio::sync::mpsc::channel(1);

        let transport = Self {
            pub_queue,
            sub_queue,
            tokio,
            outputs,
            inputs,
        };

        transport.start_pub_socket_worker(psock, pub_queue_recv);
        transport.start_sub_socket_worker(ssock, sub_queue_recv);

        transport
    }

    fn start_pub_socket_worker(
        &self,
        psock: zeromq::PubSocket,
        pub_queue: Receiver<ZmqTransportEvent>,
    ) {
        let tokio = self.tokio.clone();
        let mut psock = psock;
        let mut pub_queue = pub_queue;
        tokio::task::spawn(async move {
            while let Some(event) = pub_queue.recv().await {
                tokio
                    .block_on(psock.send(event.into()))
                    .expect("zmq pub-socket worker")
            }
        });
    }

    fn start_sub_socket_worker(
        &self,
        ssock: zeromq::SubSocket,
        sub_queue: Receiver<ZmqSubscriptionRequest>,
    ) {
        let outputs = self.outputs.clone();
        let inputs = self.inputs.clone();
        let mut ssock = ssock;
        let mut sub_queue = sub_queue;
        tokio::task::spawn_local(async move {
            loop {
                tokio::select! {
                    Ok(msg) = ssock.recv() => handle_zmq_msg(msg, &outputs, &inputs).await.unwrap(),
                    Some(req) = sub_queue.recv() => {
                        use ZmqSubscriptionRequest::*;
                        match req {
                            Subscribe(topic) => ssock.subscribe(&topic).await.expect("zmq recv worker subscribe"),
                            Unsubscribe(topic) => ssock.unsubscribe(&topic).await.expect("zmq recv worker unsubscribe"),
                        };
                    }
                };
            }
        });
    }

    fn start_input_worker(&self, input_port_id: InputPortID) -> Result<(), PortError> {
        let (to_worker_send, mut to_worker_recv) = sync_channel(1);

        let (req_send, mut req_recv) = sync_channel(1);

        {
            let mut inputs = self.inputs.write();
            if inputs.contains_key(&input_port_id) {
                return Ok(()); // TODO
            }
            let state = ZmqInputPortState::Open(to_worker_send.clone());
            let state = RwLock::new(state);
            inputs.insert(input_port_id, state);
        }

        //let sub_queue = self.sub_queue.clone();
        let pub_queue = self.pub_queue.clone();
        let inputs = self.inputs.clone();

        async fn handle_socket_event(
            event: ZmqTransportEvent,
            inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
            req_send: &Sender<(ZmqInputPortRequest, Sender<Result<(), PortError>>)>,
            to_worker_send: &Sender<ZmqTransportEvent>,
            pub_queue: &Sender<ZmqTransportEvent>,
            input_port_id: InputPortID,
        ) {
            use ZmqTransportEvent::*;
            match event {
                Connect(output_port_id, input_port_id) => {
                    let inputs = inputs.read();
                    let Some(input_state) = inputs.get(&input_port_id) else {
                        todo!();
                    };
                    let mut input_state = input_state.upgradable_read();

                    use ZmqInputPortState::*;
                    match &*input_state {
                        Open(_) => (),
                        Connected(_, _, _, _, connected_ids) => {
                            if !connected_ids.iter().any(|&id| id == output_port_id) {
                                return;
                            }
                        }
                        Closed => return,
                    };

                    let add_connection = |input_state: &mut ZmqInputPortState| match input_state {
                        Open(_) => {
                            let (msgs_send, msgs_recv) = sync_channel(1);
                            let msgs_recv = Arc::new(Mutex::new(msgs_recv));
                            *input_state = Connected(
                                req_send.clone(),
                                msgs_send,
                                msgs_recv,
                                to_worker_send.clone(),
                                vec![output_port_id],
                            );
                        }
                        Connected(_, _, _, _, ids) => {
                            ids.push(output_port_id);
                        }
                        Closed => unreachable!(),
                    };

                    pub_queue
                        .send(ZmqTransportEvent::AckConnection(
                            output_port_id,
                            input_port_id,
                        ))
                        .await
                        .expect("input worker send ack-conn event");

                    input_state.with_upgraded(add_connection);
                }
                Message(output_port_id, _, seq_id, bytes) => {
                    let inputs = inputs.read();
                    let Some(input_state) = inputs.get(&input_port_id) else {
                        todo!();
                    };
                    let input_state = input_state.read();

                    use ZmqInputPortState::*;
                    match &*input_state {
                        Connected(_, sender, _, _, connected_ids) => {
                            if !connected_ids.iter().any(|id| *id == output_port_id) {
                                return;
                            }

                            sender
                                .send(ZmqInputPortEvent::Message(bytes))
                                .await
                                .expect("input worker send message");

                            pub_queue
                                .send(ZmqTransportEvent::AckMessage(
                                    output_port_id,
                                    input_port_id,
                                    seq_id,
                                ))
                                .await
                                .expect("input worker send message ack");
                        }

                        Open(_) | Closed => todo!(),
                    }
                }
                CloseOutput(output_port_id, input_port_id) => {
                    let inputs = inputs.read();
                    let Some(input_state) = inputs.get(&input_port_id) else {
                        todo!();
                    };
                    let mut input_state = input_state.upgradable_read();

                    use ZmqInputPortState::*;
                    let Connected(_, _, _, _, ref connected_ids) = *input_state else {
                        return;
                    };

                    if !connected_ids.iter().any(|id| *id == output_port_id) {
                        return;
                    }

                    // TODO: send unsubscription for relevant topics
                    //sub_queue
                    //    .send(ZmqSubscriptionRequest::Unsubscribe("".to_string()))
                    //    .await
                    //    .expect("input worker closeoutput unsub");

                    input_state.with_upgraded(|state| match state {
                        Open(_) | Closed => (),
                        Connected(_, _, _, _, connected_ids) => {
                            connected_ids.retain(|&id| id != output_port_id)
                        }
                    });
                }

                // ignore, ideally we never receive these here:
                AckConnection(_, _) | AckMessage(_, _, _) | CloseInput(_) => (),
            }
        }

        async fn handle_input_request(
            request: ZmqInputPortRequest,
            response_chan: Sender<Result<(), PortError>>,
            inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
            pub_queue: &Sender<ZmqTransportEvent>,
            input_port_id: InputPortID,
        ) {
            use ZmqInputPortRequest::*;
            match request {
                Close => {
                    let inputs = inputs.read();
                    let Some(input_state) = inputs.get(&input_port_id) else {
                        todo!();
                    };
                    let mut input_state = input_state.upgradable_read();

                    use ZmqInputPortState::*;
                    let Connected(_, ref port_events, _, _, _) = *input_state else {
                        return;
                    };

                    pub_queue
                        .send(ZmqTransportEvent::CloseInput(input_port_id))
                        .await
                        .expect("input worker send close event");

                    port_events
                        .send(ZmqInputPortEvent::Closed)
                        .await
                        .expect("input worker send port closed");

                    input_state.with_upgraded(|state| *state = ZmqInputPortState::Closed);

                    response_chan
                        .send(Ok(()))
                        .await
                        .expect("input worker respond close")
                }
            }
        }

        tokio::task::spawn_local(async move {
            // Input worker loop:
            //   1. Receive connection attempts and respond
            //   2. Receive messages and forward to channel
            //   3. Receive and handle disconnects
            loop {
                tokio::select! {
                    Some(event) = to_worker_recv.recv() => {
                        handle_socket_event(event, &inputs, &req_send, &to_worker_send, &pub_queue, input_port_id).await;
                    }
                    Some((request, response_chan)) = req_recv.recv() => {
                        handle_input_request(request, response_chan, &inputs, &pub_queue, input_port_id).await;
                    }
                };
            }
        });

        let topic = format!("{}:", input_port_id);

        // send sub request
        self.tokio
            .block_on(
                self.sub_queue
                    .send(ZmqSubscriptionRequest::Subscribe(topic)),
            )
            .map_err(|e| PortError::Other(e.to_string()))
    }

    fn start_output_worker(&self, output_port_id: OutputPortID) -> Result<(), PortError> {
        let (conn_send, mut conn_recv) = sync_channel(1);

        let (to_worker_send, mut to_worker_recv) = sync_channel(1);

        {
            let mut outputs = self.outputs.write();
            if outputs.contains_key(&output_port_id) {
                return Ok(()); // TODO
            }
            let state = ZmqOutputPortState::Open(conn_send, to_worker_send.clone());
            let state = RwLock::new(state);
            outputs.insert(output_port_id, state);
        }

        let outputs = self.outputs.clone();
        let pub_queue = self.pub_queue.clone();
        tokio::task::spawn_local(async move {
            let Some((input_port_id, conn_confirm)) = conn_recv.recv().await else {
                todo!();
            };

            let (msg_req_send, mut msg_req_recv) = sync_channel(1);

            // Output worker loop:
            //   1. Send connection attempt
            //   2. Send messages
            //     2.1 Wait for ACK
            //     2.2. Resend on timeout
            //   3. Send disconnect events

            loop {
                pub_queue
                    .send(ZmqTransportEvent::Connect(output_port_id, input_port_id))
                    .await
                    .expect("output worker send connect event");

                let response = to_worker_recv
                    .recv()
                    .await
                    .expect("output worker recv ack-conn event");

                use ZmqTransportEvent::*;
                match response {
                    AckConnection(_, input_port_id) => {
                        let outputs = outputs.read();
                        let Some(output_state) = outputs.get(&output_port_id) else {
                            todo!();
                        };
                        let mut output_state = output_state.write();
                        debug_assert!(matches!(*output_state, ZmqOutputPortState::Open(..)));
                        *output_state = ZmqOutputPortState::Connected(
                            msg_req_send,
                            to_worker_send,
                            input_port_id,
                        );

                        conn_confirm
                            .send(Ok(()))
                            .await
                            .expect("output worker respond conn");

                        break;
                    }
                    _ => continue,
                }
            }

            let mut seq_id = 1;
            'outer: loop {
                let (request, response_chan) = msg_req_recv
                    .recv()
                    .await
                    .expect("output worker recv msg req");

                match request {
                    ZmqOutputPortRequest::Close => {
                        let response = pub_queue
                            .send(ZmqTransportEvent::CloseOutput(
                                output_port_id,
                                input_port_id,
                            ))
                            .await
                            .map_err(|e| PortError::Other(e.to_string()));

                        response_chan
                            .send(response)
                            .await
                            .expect("output worker respond close");
                    }
                    ZmqOutputPortRequest::Send(bytes) => {
                        pub_queue
                            .send(ZmqTransportEvent::Message(
                                output_port_id,
                                input_port_id,
                                seq_id,
                                bytes,
                            ))
                            .await
                            .expect("output worker send message event");

                        loop {
                            let event = to_worker_recv
                                .recv()
                                .await
                                .expect("output worker event recv");

                            use ZmqTransportEvent::*;
                            match event {
                                AckMessage(_, _, ack_id) => {
                                    if ack_id == seq_id {
                                        break;
                                    }
                                }
                                CloseInput(_) => {
                                    let outputs = outputs.read();
                                    let Some(output_state) = outputs.get(&output_port_id) else {
                                        todo!();
                                    };
                                    let mut output_state = output_state.write();
                                    debug_assert!(matches!(
                                        *output_state,
                                        ZmqOutputPortState::Connected(..)
                                    ));
                                    *output_state = ZmqOutputPortState::Closed;

                                    response_chan
                                        .send(Err(PortError::Disconnected))
                                        .await
                                        .expect("output worker respond msg");

                                    break 'outer;
                                }

                                // ignore others, we shouldn't receive any new conn-acks
                                // nor should we be receiving input port events
                                AckConnection(_, _)
                                | Connect(_, _)
                                | Message(_, _, _, _)
                                | CloseOutput(_, _) => continue,
                            }
                        }
                    }
                }

                seq_id += 1;
            }
        });

        Ok(())
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
        let inputs = self.inputs.read();
        let new_id = InputPortID::try_from(-(inputs.len() as isize + 1))
            .map_err(|e| PortError::Other(e.to_string()))?;
        self.start_input_worker(new_id).map(|_| new_id)
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        let outputs = self.outputs.read();
        let new_id = OutputPortID::try_from(outputs.len() as isize + 1)
            .map_err(|e| PortError::Other(e.to_string()))?;
        self.start_output_worker(new_id).map(|_| new_id)
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        let inputs = self.inputs.read();

        let Some(state) = inputs.get(&input) else {
            return Err(PortError::Invalid(input.into()));
        };

        let state = state.read();

        let ZmqInputPortState::Connected(sender, _, _, _, _) = &*state else {
            return Err(PortError::Disconnected);
        };

        let (close_send, mut close_recv) = sync_channel(1);

        self.tokio
            .block_on(sender.send((ZmqInputPortRequest::Close, close_send)))
            .map_err(|e| PortError::Other(e.to_string()))?;

        self.tokio
            .block_on(close_recv.recv())
            .ok_or(PortError::Disconnected)?
            .map(|_| true)
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        let outputs = self.outputs.read();

        let Some(state) = outputs.get(&output) else {
            return Err(PortError::Invalid(output.into()));
        };

        let state = state.write();

        let ZmqOutputPortState::Connected(sender, _, _) = &*state else {
            return Err(PortError::Disconnected);
        };

        let (close_send, mut close_recv) = sync_channel(1);

        self.tokio
            .block_on(sender.send((ZmqOutputPortRequest::Close, close_send)))
            .map_err(|e| PortError::Other(e.to_string()))?;

        self.tokio
            .block_on(close_recv.recv())
            .ok_or(PortError::Disconnected)?
            .map(|_| true)
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        let outputs = self.outputs.read();
        let Some(output_state) = outputs.get(&source) else {
            return Err(PortError::Invalid(source.into()));
        };

        let output_state = output_state.read();
        let ZmqOutputPortState::Open(ref sender, _) = *output_state else {
            return Err(PortError::Invalid(source.into()));
        };

        let (confirm_send, mut confirm_recv) = sync_channel(1);

        self.tokio
            .block_on(sender.send((target, confirm_send)))
            .map_err(|e| PortError::Other(e.to_string()))?;

        self.tokio
            .block_on(confirm_recv.recv())
            .ok_or(PortError::Disconnected)?
            .map(|_| true)
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        let outputs = self.outputs.read();
        let Some(output) = outputs.get(&output) else {
            return Err(PortError::Invalid(output.into()));
        };
        let output = output.read();

        let ZmqOutputPortState::Connected(sender, _, _) = &*output else {
            return Err(PortError::Disconnected);
        };

        let (ack_send, mut ack_recv) = sync_channel(1);

        self.tokio
            .block_on(sender.send((ZmqOutputPortRequest::Send(message), ack_send)))
            .map_err(|e| PortError::Other(e.to_string()))?;

        self.tokio
            .block_on(ack_recv.recv())
            .ok_or(PortError::Disconnected)?
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        let inputs = self.inputs.read();
        let Some(input) = inputs.get(&input) else {
            return Err(PortError::Invalid(input.into()));
        };
        let input = input.read();

        let ZmqInputPortState::Connected(_, _, receiver, _, _) = &*input else {
            return Err(PortError::Disconnected);
        };
        let mut receiver = receiver.lock();

        use ZmqInputPortEvent::*;
        match self.tokio.block_on(receiver.recv()) {
            Some(Closed) => Ok(None), // EOS
            Some(Message(bytes)) => Ok(Some(bytes)),
            None => Err(PortError::Disconnected),
        }
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!();
    }
}

async fn handle_zmq_msg(
    msg: ZmqMessage,
    outputs: &RwLock<BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>>,
    inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
) -> Result<(), Error> {
    let Ok(event) = ZmqTransportEvent::try_from(msg) else {
        todo!();
    };

    use ZmqTransportEvent::*;
    match event {
        // input ports
        Connect(_, input_port_id) => {
            let inputs = inputs.read();
            let Some(input) = inputs.get(&input_port_id) else {
                todo!();
            };
            let input = input.read();

            use ZmqInputPortState::*;
            match &*input {
                Closed => todo!(),
                Open(sender) | Connected(_, _, _, sender, _) => sender.send(event).await.unwrap(),
            };
        }
        Message(_, input_port_id, _, _) => {
            let inputs = inputs.read();
            let Some(input) = inputs.get(&input_port_id) else {
                todo!();
            };

            let input = input.read();
            let ZmqInputPortState::Connected(_, _, _, sender, _) = &*input else {
                todo!();
            };

            sender.send(event).await.unwrap();
        }
        CloseOutput(_, input_port_id) => {
            let inputs = inputs.read();
            let Some(input) = inputs.get(&input_port_id) else {
                todo!();
            };
            let input = input.read();

            use ZmqInputPortState::*;
            match &*input {
                Closed => todo!(),
                Open(sender) | Connected(_, _, _, sender, _) => {
                    sender.send(event).await.unwrap();
                }
            };
        }

        // output ports
        AckConnection(output_port_id, _) => {
            let outputs = outputs.read();
            let Some(output) = outputs.get(&output_port_id) else {
                todo!();
            };
            let output = output.read();

            let ZmqOutputPortState::Open(_, sender) = &*output else {
                todo!();
            };
            sender.send(event).await.unwrap();
        }
        AckMessage(output_port_id, _, _) => {
            let outputs = outputs.read();
            let Some(output) = outputs.get(&output_port_id) else {
                todo!();
            };
            let output = output.read();
            let ZmqOutputPortState::Connected(_, sender, _) = &*output else {
                todo!();
            };
            sender.send(event).await.unwrap();
        }
        CloseInput(input_port_id) => {
            for (_, state) in outputs.read().iter() {
                let state = state.read();
                let ZmqOutputPortState::Connected(_, ref sender, ref id) = *state else {
                    continue;
                };
                if *id != input_port_id {
                    continue;
                }
                if let Err(_e) = sender.send(event.clone()).await {
                    continue; // TODO
                }
            }
        }
    }

    Ok(())
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
