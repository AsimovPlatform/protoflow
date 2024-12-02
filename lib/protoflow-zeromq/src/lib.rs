// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

#[path = "protoflow.zmq.rs"]
mod protoflow_zmq;

mod input_port;
use input_port::*;

mod output_port;
use output_port::*;

extern crate std;

use protoflow_core::{
    prelude::{vec, Arc, BTreeMap, Bytes, String, ToString, Vec},
    InputPortID, OutputPortID, PortError, PortResult, PortState, Transport,
};

use core::fmt::Error;
use std::{format, write};
use tokio::sync::{
    mpsc::{channel, error::TryRecvError, Receiver, Sender},
    RwLock,
};
use zeromq::{util::PeerIdentity, Socket, SocketOptions, SocketRecv, SocketSend, ZmqMessage};

#[cfg(feature = "tracing")]
use tracing::{trace, trace_span};

const DEFAULT_PUB_SOCKET: &str = "tcp://127.0.0.1:10000";
const DEFAULT_SUB_SOCKET: &str = "tcp://127.0.0.1:10001";

pub struct ZmqTransport {
    tokio: tokio::runtime::Handle,

    pub_queue: Sender<ZmqTransportEvent>,
    sub_queue: Sender<ZmqSubscriptionRequest>,

    outputs: Arc<RwLock<BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>>>,
    inputs: Arc<RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>>,
}

type SequenceID = u64;

/// ZmqTransportEvent represents the data that goes over the wire from one port to another.
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
        let mut msg = ZmqMessage::from(topic);

        fn map_id<T>(id: T) -> i64
        where
            isize: From<T>,
        {
            isize::from(id) as i64
        }

        // second frame of the message is the payload
        use prost::Message;
        use protoflow_zmq::{event::Payload, Event};
        use ZmqTransportEvent::*;
        let payload = match value {
            Connect(output, input) => Payload::Connect(protoflow_zmq::Connect {
                output: map_id(output),
                input: map_id(input),
            }),
            AckConnection(output, input) => Payload::AckConnection(protoflow_zmq::AckConnection {
                output: map_id(output),
                input: map_id(input),
            }),
            Message(output, input, sequence, message) => Payload::Message(protoflow_zmq::Message {
                output: map_id(output),
                input: map_id(input),
                sequence,
                message: message.to_vec(),
            }),
            AckMessage(output, input, sequence) => Payload::AckMessage(protoflow_zmq::AckMessage {
                output: map_id(output),
                input: map_id(input),
                sequence,
            }),
            CloseOutput(output, input) => Payload::CloseOutput(protoflow_zmq::CloseOutput {
                output: map_id(output),
                input: map_id(input),
            }),
            CloseInput(input) => Payload::CloseInput(protoflow_zmq::CloseInput {
                input: map_id(input),
            }),
        };

        let bytes = Event {
            payload: Some(payload),
        }
        .encode_to_vec();
        msg.push_back(bytes.into());

        msg
    }
}

impl TryFrom<ZmqMessage> for ZmqTransportEvent {
    type Error = protoflow_core::DecodeError;

    fn try_from(value: ZmqMessage) -> Result<Self, Self::Error> {
        use prost::Message;
        use protoflow_core::DecodeError;
        use protoflow_zmq::{event::Payload, Event};

        fn map_id<T>(id: i64) -> Result<T, DecodeError>
        where
            T: TryFrom<isize>,
            std::borrow::Cow<'static, str>: From<<T as TryFrom<isize>>::Error>,
        {
            (id as isize).try_into().map_err(DecodeError::new)
        }

        value
            .get(1)
            .ok_or_else(|| {
                protoflow_core::DecodeError::new(
                    "message from socket contains less than two frames",
                )
            })
            .and_then(|bytes| {
                let event = Event::decode(bytes.as_ref())?;

                use ZmqTransportEvent::*;
                Ok(match event.payload {
                    None => todo!(),
                    Some(Payload::Connect(protoflow_zmq::Connect { output, input })) => {
                        Connect(map_id(output)?, map_id(input)?)
                    }

                    Some(Payload::AckConnection(protoflow_zmq::AckConnection {
                        output,
                        input,
                    })) => AckConnection(map_id(output)?, map_id(input)?),

                    Some(Payload::Message(protoflow_zmq::Message {
                        output,
                        input,
                        sequence,
                        message,
                    })) => Message(
                        map_id(output)?,
                        map_id(input)?,
                        sequence,
                        Bytes::from(message),
                    ),

                    Some(Payload::AckMessage(protoflow_zmq::AckMessage {
                        output,
                        input,
                        sequence,
                    })) => AckMessage(map_id(output)?, map_id(input)?, sequence),

                    Some(Payload::CloseOutput(protoflow_zmq::CloseOutput { output, input })) => {
                        CloseOutput(map_id(output)?, map_id(input)?)
                    }

                    Some(Payload::CloseInput(protoflow_zmq::CloseInput { input })) => {
                        CloseInput(map_id(input)?)
                    }
                })
            })
    }
}

impl Default for ZmqTransport {
    fn default() -> Self {
        Self::new(DEFAULT_PUB_SOCKET, DEFAULT_SUB_SOCKET)
    }
}

#[derive(Clone, Debug)]
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

        let (pub_queue, pub_queue_recv) = channel(1);

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
        let mut psock = psock;
        let mut pub_queue = pub_queue;
        tokio::task::spawn(async move {
            while let Some(event) = pub_queue.recv().await {
                #[cfg(feature = "tracing")]
                trace!(
                    target: "ZmqTransport::pub_socket",
                    ?event,
                    "sending event to socket"
                );

                psock
                    .send(event.into())
                    .await
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
        tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    Ok(msg) = ssock.recv() => {
                        #[cfg(feature = "tracing")]
                        trace!(
                            target: "ZmqTransport::sub_socket",
                            ?msg,
                            "got message from socket"
                        );

                        handle_zmq_msg(msg, &outputs, &inputs).await.unwrap()
                    },
                    Some(req) = sub_queue.recv() => {
                        #[cfg(feature = "tracing")]
                        trace!(
                            target: "ZmqTransport::sub_socket",
                            ?req,
                            "got sub update request"
                        );

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
}

impl Transport for ZmqTransport {
    fn input_state(&self, input: InputPortID) -> PortResult<PortState> {
        self.tokio.block_on(async {
            Ok(self
                .inputs
                .read()
                .await
                .get(&input)
                .ok_or_else(|| PortError::Invalid(input.into()))?
                .read()
                .await
                .state())
        })
    }

    fn output_state(&self, output: OutputPortID) -> PortResult<PortState> {
        self.tokio.block_on(async {
            Ok(self
                .outputs
                .read()
                .await
                .get(&output)
                .ok_or_else(|| PortError::Invalid(output.into()))?
                .read()
                .await
                .state())
        })
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::open_input", "creating new input port");

        let new_id = {
            let inputs = self.tokio.block_on(self.inputs.read());
            InputPortID::try_from(-(inputs.len() as isize + 1))
                .map_err(|e| PortError::Other(e.to_string()))?
        };

        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::open_input", ?new_id, "created new input port");

        start_input_worker(self, new_id).map(|_| new_id)
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::open_output", "creating new output port");

        let new_id = {
            let outputs = self.tokio.block_on(self.outputs.read());
            OutputPortID::try_from(outputs.len() as isize + 1)
                .map_err(|e| PortError::Other(e.to_string()))?
        };

        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::open_output", ?new_id, "created new output port");

        start_output_worker(self, new_id).map(|_| new_id)
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        self.tokio.block_on(async {
            let sender = {
                let inputs = self.inputs.read().await;
                let Some(input_state) = inputs.get(&input) else {
                    return Err(PortError::Invalid(input.into()));
                };

                let input_state = input_state.read().await;
                let ZmqInputPortState::Connected(sender, _, _, _, _) = &*input_state else {
                    return Err(PortError::Disconnected);
                };

                sender.clone()
            };

            let (close_send, mut close_recv) = channel(1);

            sender
                .send((ZmqInputPortRequest::Close, close_send))
                .await
                .map_err(|e| PortError::Other(e.to_string()))?;

            close_recv
                .recv()
                .await
                .ok_or(PortError::Disconnected)?
                .map(|_| true)
        })
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        self.tokio.block_on(async {
            let sender = {
                let outputs = self.outputs.read().await;
                let Some(output_state) = outputs.get(&output) else {
                    return Err(PortError::Invalid(output.into()));
                };

                let output_state = output_state.read().await;
                let ZmqOutputPortState::Connected(sender, _, _) = &*output_state else {
                    return Err(PortError::Disconnected);
                };

                sender.clone()
            };

            let (close_send, mut close_recv) = channel(1);

            sender
                .send((ZmqOutputPortRequest::Close, close_send))
                .await
                .map_err(|e| PortError::Other(e.to_string()))?;

            close_recv
                .recv()
                .await
                .ok_or(PortError::Disconnected)?
                .map(|_| true)
        })
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::connect", ?source, ?target, "connecting ports");

        self.tokio.block_on(async {
            let sender = {
                let outputs = self.outputs.read().await;
                let Some(output_state) = outputs.get(&source) else {
                    return Err(PortError::Invalid(source.into()));
                };

                let output_state = output_state.read().await;
                let ZmqOutputPortState::Open(ref sender, _) = *output_state else {
                    return Err(PortError::Invalid(source.into()));
                };

                sender.clone()
            };

            let (confirm_send, mut confirm_recv) = channel(1);

            sender
                .send((target, confirm_send))
                .await
                .map_err(|e| PortError::Other(e.to_string()))?;

            confirm_recv
                .recv()
                .await
                .ok_or(PortError::Disconnected)?
                .map(|_| true)
        })
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::send", ?output, "sending from output port");

        self.tokio.block_on(async {
            let sender = {
                let outputs = self.outputs.read().await;
                let Some(output) = outputs.get(&output) else {
                    return Err(PortError::Invalid(output.into()));
                };
                let output = output.read().await;

                let ZmqOutputPortState::Connected(sender, _, _) = &*output else {
                    return Err(PortError::Disconnected);
                };

                sender.clone()
            };

            let (ack_send, mut ack_recv) = channel(1);

            sender
                .send((ZmqOutputPortRequest::Send(message), ack_send))
                .await
                .map_err(|e| PortError::Other(e.to_string()))?;

            ack_recv.recv().await.ok_or(PortError::Disconnected)?
        })
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::recv", ?input, "receiving from input port");

        self.tokio.block_on(async {
            let receiver = {
                let inputs = self.inputs.read().await;
                let Some(input_state) = inputs.get(&input) else {
                    return Err(PortError::Invalid(input.into()));
                };

                let input_state = input_state.read().await;
                let ZmqInputPortState::Connected(_, _, receiver, _, _) = &*input_state else {
                    return Err(PortError::Disconnected);
                };

                receiver.clone()
            };

            let mut receiver = receiver.lock().await;

            use ZmqInputPortEvent::*;
            match receiver.recv().await {
                Some(Closed) => Ok(None), // EOS
                Some(Message(bytes)) => Ok(Some(bytes)),
                None => Err(PortError::Disconnected),
            }
        })
    }

    fn try_recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        #[cfg(feature = "tracing")]
        trace!(target: "ZmqTransport::try_recv", ?input, "receiving from input port");

        self.tokio.block_on(async {
            let receiver = {
                let inputs = self.inputs.read().await;
                let Some(input_state) = inputs.get(&input) else {
                    return Err(PortError::Invalid(input.into()));
                };

                let input_state = input_state.read().await;
                let ZmqInputPortState::Connected(_, _, receiver, _, _) = &*input_state else {
                    return Err(PortError::Disconnected);
                };

                receiver.clone()
            };

            let mut receiver = receiver.lock().await;

            use ZmqInputPortEvent::*;
            match receiver.try_recv() {
                Ok(Closed) => Ok(None), // EOS
                Ok(Message(bytes)) => Ok(Some(bytes)),
                Err(TryRecvError::Disconnected) => Err(PortError::Disconnected),
                // TODO: what should we answer with here?:
                Err(TryRecvError::Empty) => Err(PortError::RecvFailed),
            }
        })
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

    #[cfg(feature = "tracing")]
    trace!(target: "handle_zmq_msg", ?event, "got event");

    use ZmqTransportEvent::*;
    match event {
        // input ports
        Connect(_, input_port_id) => {
            let sender = {
                let inputs = inputs.read().await;
                let Some(input) = inputs.get(&input_port_id) else {
                    todo!();
                };
                let input = input.read().await;

                use ZmqInputPortState::*;
                match &*input {
                    Closed => todo!(),
                    Open(sender) | Connected(_, _, _, sender, _) => sender.clone(),
                }
            };

            sender.send(event).await.unwrap();
        }
        Message(_, input_port_id, _, _) => {
            let sender = {
                let inputs = inputs.read().await;
                let Some(input) = inputs.get(&input_port_id) else {
                    todo!();
                };

                let input = input.read().await;
                let ZmqInputPortState::Connected(_, _, _, sender, _) = &*input else {
                    todo!();
                };

                sender.clone()
            };

            sender.send(event).await.unwrap();
        }
        CloseOutput(_, input_port_id) => {
            let sender = {
                let inputs = inputs.read().await;
                let Some(input) = inputs.get(&input_port_id) else {
                    todo!();
                };
                let input = input.read().await;

                use ZmqInputPortState::*;
                match &*input {
                    Closed => todo!(),
                    Open(sender) | Connected(_, _, _, sender, _) => sender.clone(),
                }
            };

            sender.send(event).await.unwrap();
        }

        // output ports
        AckConnection(output_port_id, _) => {
            let sender = {
                let outputs = outputs.read().await;
                let Some(output) = outputs.get(&output_port_id) else {
                    todo!();
                };
                let output = output.read().await;

                let ZmqOutputPortState::Open(_, sender) = &*output else {
                    todo!();
                };

                sender.clone()
            };

            sender.send(event).await.unwrap();
        }
        AckMessage(output_port_id, _, _) => {
            let sender = {
                let outputs = outputs.read().await;
                let Some(output) = outputs.get(&output_port_id) else {
                    todo!();
                };
                let output = output.read().await;
                let ZmqOutputPortState::Connected(_, sender, _) = &*output else {
                    todo!();
                };

                sender.clone()
            };

            sender.send(event).await.unwrap();
        }
        CloseInput(input_port_id) => {
            for (_, state) in outputs.read().await.iter() {
                let sender = {
                    let state = state.read().await;
                    let ZmqOutputPortState::Connected(_, ref sender, ref id) = *state else {
                        continue;
                    };
                    if *id != input_port_id {
                        continue;
                    }

                    sender.clone()
                };

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

    use protoflow_core::{runtimes::StdRuntime, System};

    use futures_util::future::TryFutureExt;
    use zeromq::{PubSocket, SocketRecv, SocketSend, SubSocket};

    async fn start_zmqtransport_server() {
        // bind a *SUB* socket to the *PUB* address so that the transport can *PUB* to it
        let mut pub_srv = SubSocket::new();
        pub_srv.bind(DEFAULT_PUB_SOCKET).await.unwrap();

        // bind a *PUB* socket to the *SUB* address so that the transport can *SUB* to it
        let mut sub_srv = PubSocket::new();
        sub_srv.bind(DEFAULT_SUB_SOCKET).await.unwrap();

        // subscribe to all messages
        pub_srv.subscribe("").await.unwrap();

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

        rt.block_on(start_zmqtransport_server());

        let _ = System::<ZmqTransport>::build(|_s| { /* do nothing */ });
    }

    #[test]
    fn run_transport() {
        tracing_subscriber::fmt::init();

        let rt = tokio::runtime::Runtime::new().unwrap();
        let _guard = rt.enter();

        rt.block_on(start_zmqtransport_server());

        let transport = ZmqTransport::default();
        let runtime = StdRuntime::new(transport).unwrap();
        let system = System::new(&runtime);

        let output = system.output();
        let input = system.input();

        system.connect(&output, &input);

        let output = std::thread::spawn(move || {
            let mut output = output;
            output.send(&"Hello world!".to_string())?;
            output.close()
        });

        let input = std::thread::spawn(move || {
            let mut input = input;

            let msg = input.recv()?;
            assert_eq!(Some("Hello world!".to_string()), msg);

            let msg = input.recv()?;
            assert_eq!(None, msg);

            input.close()
        });

        output.join().expect("thread failed").unwrap();
        input.join().expect("thread failed").unwrap();
    }
}
