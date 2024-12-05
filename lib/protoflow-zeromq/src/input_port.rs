// This is free and unencumbered software released into the public domain.

use crate::{
    subscribe_topics, unsubscribe_topics, SequenceID, ZmqSubscriptionRequest, ZmqTransport,
    ZmqTransportEvent,
};
use protoflow_core::{
    prelude::{fmt, format, vec, Arc, BTreeMap, Bytes, String, ToString, Vec},
    InputPortID, OutputPortID, PortError, PortState,
};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex, RwLock,
};

#[cfg(feature = "tracing")]
use tracing::{error, info, trace, trace_span, warn};

#[derive(Clone, Debug)]
pub enum ZmqInputPortRequest {
    Close,
}

/// ZmqInputPortEvent represents events that we receive from the background worker of the port.
#[derive(Clone, Debug, PartialEq)]
pub enum ZmqInputPortEvent {
    Message(Bytes),
    Closed,
}

#[derive(Clone, Debug)]
pub enum ZmqInputPortState {
    Open(
        // channel for close requests from the public `close` method
        Sender<(ZmqInputPortRequest, Sender<Result<(), PortError>>)>,
        // channel used internally for events from socket
        Sender<ZmqTransportEvent>,
    ),
    Connected(
        // channel for requests from public close
        Sender<(ZmqInputPortRequest, Sender<Result<(), PortError>>)>,
        // channels to send-to and receive-from the public `recv` method
        Sender<ZmqInputPortEvent>,
        Arc<Mutex<Receiver<ZmqInputPortEvent>>>,
        // channel used internally for events from socket
        Sender<ZmqTransportEvent>,
        // vec of the connected port ids
        BTreeMap<OutputPortID, SequenceID>,
    ),
    Closed,
}

impl fmt::Display for ZmqInputPortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ZmqInputPortState::*;
        match *self {
            Open(..) => write!(f, "Open"),
            Connected(.., ref ids) => {
                write!(
                    f,
                    "Connected({:?})",
                    ids.keys().map(|id| isize::from(*id)).collect::<Vec<_>>()
                )
            }
            Closed => write!(f, "Closed"),
        }
    }
}

impl ZmqInputPortState {
    pub fn state(&self) -> PortState {
        use ZmqInputPortState::*;
        match self {
            Open(..) => PortState::Open,
            Connected(..) => PortState::Connected,
            Closed => PortState::Closed,
        }
    }

    pub async fn event_sender(&self) -> Option<Sender<ZmqTransportEvent>> {
        use ZmqInputPortState::*;
        match self {
            Open(_, sender) | Connected(.., sender, _) => Some(sender.clone()),
            Closed => None,
        }
    }
}

fn input_topics(id: InputPortID) -> Vec<String> {
    vec![
        format!("{}:conn", id),
        format!("{}:msg", id),
        format!("{}:closeOut", id),
    ]
}

pub async fn input_port_event_sender(
    inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
    id: InputPortID,
) -> Option<Sender<ZmqTransportEvent>> {
    inputs
        .read()
        .await
        .get(&id)?
        .read()
        .await
        .event_sender()
        .await
}

pub fn start_input_worker(
    transport: &ZmqTransport,
    input_port_id: InputPortID,
) -> Result<(), PortError> {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::input_port_worker", ?input_port_id);

    let (to_worker_send, mut to_worker_recv) = channel(1);
    let (req_send, mut req_recv) = channel(1);

    {
        let mut inputs = transport.tokio.block_on(transport.inputs.write());
        if inputs.contains_key(&input_port_id) {
            return Err(PortError::Invalid(input_port_id.into()));
        }
        let state = ZmqInputPortState::Open(req_send.clone(), to_worker_send.clone());
        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!("saving new state: {}", state));
        inputs.insert(input_port_id, RwLock::new(state));
    }

    let sub_queue = transport.sub_queue.clone();
    let pub_queue = transport.pub_queue.clone();
    let inputs = transport.inputs.clone();

    let topics = input_topics(input_port_id);
    if transport
        .tokio
        .block_on(subscribe_topics(&topics, &sub_queue))
        .is_err()
    {
        #[cfg(feature = "tracing")]
        span.in_scope(|| error!("topic subscription failed"));
        return Err(PortError::Other("topic subscription failed".to_string()));
    }

    async fn handle_socket_event(
        event: ZmqTransportEvent,
        inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
        pub_queue: &Sender<ZmqTransportEvent>,
        input_port_id: InputPortID,
    ) {
        #[cfg(feature = "tracing")]
        let span = trace_span!(
            "ZmqTransport::input_port_worker::handle_socket_event",
            ?input_port_id
        );

        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!(?event, "got socket event"));

        use ZmqTransportEvent::*;
        match event {
            Connect(output_port_id, target_id) => {
                #[cfg(feature = "tracing")]
                let span = trace_span!(parent: &span, "Connect", ?output_port_id);

                debug_assert_eq!(input_port_id, target_id);

                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| error!("port state not found"));
                    return;
                };
                let mut input_state = input_state.write().await;

                use ZmqInputPortState::*;
                match &*input_state {
                    Open(..) => (),
                    Connected(.., connected_ids) => {
                        if connected_ids.contains_key(&output_port_id) {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| trace!("output port is already connected"));
                            return;
                        }
                    }
                    Closed => return,
                };

                let add_connection = |input_state: &mut ZmqInputPortState| match input_state {
                    Open(req_send, to_worker_send) => {
                        let (msgs_send, msgs_recv) = channel(1);
                        let msgs_recv = Arc::new(Mutex::new(msgs_recv));
                        let mut connected_ids = BTreeMap::new();
                        connected_ids.insert(output_port_id, 0);
                        *input_state = Connected(
                            req_send.clone(),
                            msgs_send,
                            msgs_recv,
                            to_worker_send.clone(),
                            connected_ids,
                        );
                    }
                    Connected(.., ids) => {
                        ids.insert(output_port_id, 0);
                    }
                    Closed => unreachable!(),
                };

                if pub_queue
                    .send(ZmqTransportEvent::AckConnection(
                        output_port_id,
                        input_port_id,
                    ))
                    .await
                    .is_err()
                {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| warn!("publish channel is closed"));
                    return;
                }

                #[cfg(feature = "tracing")]
                span.in_scope(|| trace!("sent conn-ack"));

                add_connection(&mut input_state);

                #[cfg(feature = "tracing")]
                span.in_scope(|| info!("Connected new port: {}", input_state));
            }
            Message(output_port_id, target_id, msg_seq_id, bytes) => {
                #[cfg(feature = "tracing")]
                let span = trace_span!(parent: &span, "Message", ?output_port_id, ?msg_seq_id);

                debug_assert_eq!(input_port_id, target_id);

                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| error!("port state not found"));
                    return;
                };
                let mut input_state = input_state.write().await;

                use ZmqInputPortState::*;
                match *input_state {
                    Connected(_, ref sender, _, _, ref mut connected_ids) => {
                        let Some(&last_seen_seq_id) = connected_ids.get(&output_port_id) else {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| trace!("got message from non-connected output port"));
                            return;
                        };

                        let send_ack = {
                            #[cfg(feature = "tracing")]
                            let span = span.clone();

                            |ack_id| async move {
                                if pub_queue
                                    .send(ZmqTransportEvent::AckMessage(
                                        output_port_id,
                                        input_port_id,
                                        ack_id,
                                    ))
                                    .await
                                    .is_err()
                                {
                                    #[cfg(feature = "tracing")]
                                    span.in_scope(|| warn!("publish channel is closed"));
                                }
                                #[cfg(feature = "tracing")]
                                span.in_scope(|| trace!(?ack_id, "sent msg-ack"));
                            }
                        };

                        use std::cmp::Ordering::*;
                        match msg_seq_id.cmp(&last_seen_seq_id) {
                            // seq_id for msg is greater than last seen seq_id by one
                            Greater if (msg_seq_id - last_seen_seq_id == 1) => {
                                if sender
                                    .send(ZmqInputPortEvent::Message(bytes))
                                    .await
                                    .is_err()
                                {
                                    #[cfg(feature = "tracing")]
                                    span.in_scope(|| warn!("receiver for input events has closed"));
                                    return;
                                }
                                send_ack(msg_seq_id).await;
                                let _ = connected_ids.insert(output_port_id, msg_seq_id);
                            }
                            Equal => {
                                send_ack(last_seen_seq_id).await;
                            }
                            // either the seq_id is greater than  the last seen seq_id by more than
                            // one, or somehow less than the last seen seq_id:
                            _ => {
                                send_ack(last_seen_seq_id).await;
                            }
                        }
                    }

                    Open(..) | Closed => {
                        #[cfg(feature = "tracing")]
                        span.in_scope(|| warn!("port is not connected: {}", input_state));
                    }
                }
            }
            CloseOutput(output_port_id, target_id) => {
                #[cfg(feature = "tracing")]
                let span = trace_span!(parent: &span, "CloseOutput", ?output_port_id);

                debug_assert_eq!(input_port_id, target_id);

                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| error!("port state not found"));
                    return;
                };

                let mut input_state = input_state.write().await;

                use ZmqInputPortState::*;
                let Connected(ref req_send, ref sender, _, ref event_sender, ref mut connected_ids) =
                    *input_state
                else {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| trace!("input port wasn't connected"));
                    return;
                };

                if connected_ids.remove(&output_port_id).is_none() {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| trace!("output port doesn't match any connected port"));
                    return;
                }

                if !connected_ids.is_empty() {
                    return;
                }

                #[cfg(feature = "tracing")]
                span.in_scope(|| trace!("last connected port disconnected"));

                if let Err(err) = sender.try_send(ZmqInputPortEvent::Closed) {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| warn!("did not send InputPortEvent::Closed: {}", err));
                }

                // TODO: Should last connection closing close the input port too?
                // It does in the MPSC transport.
                //*input_state = ZmqInputPortState::Closed;

                *input_state = Open(req_send.clone(), event_sender.clone())
            }

            // ignore, ideally we never receive these here:
            AckConnection(..) | AckMessage(..) | CloseInput(_) => (),
        }
    }

    async fn handle_input_request(
        request: ZmqInputPortRequest,
        response_chan: Sender<Result<(), PortError>>,
        inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
        pub_queue: &Sender<ZmqTransportEvent>,
        sub_queue: &Sender<ZmqSubscriptionRequest>,
        input_port_id: InputPortID,
    ) {
        #[cfg(feature = "tracing")]
        let span = trace_span!(
            "ZmqTransport::input_port_worker::handle_input_event",
            ?input_port_id
        );

        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!(?request, "got input request"));

        use ZmqInputPortRequest::*;
        match request {
            Close => {
                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| error!("port state not found"));
                    return;
                };
                let mut input_state = input_state.write().await;

                use ZmqInputPortState::*;

                if let Closed = *input_state {
                    return;
                }

                if let Connected(_, ref port_events, ..) = *input_state {
                    if let Err(err) = port_events.try_send(ZmqInputPortEvent::Closed) {
                        #[cfg(feature = "tracing")]
                        span.in_scope(|| warn!("did not send InputPortEvent::Closed: {}", err));
                    }
                };

                if pub_queue
                    .send(ZmqTransportEvent::CloseInput(input_port_id))
                    .await
                    .is_err()
                {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| error!("can't publish CloseInput event"));
                    // don't exit, continue to close the port
                }

                *input_state = ZmqInputPortState::Closed;

                let topics = input_topics(input_port_id);
                if unsubscribe_topics(&topics, sub_queue).await.is_err() {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| error!("topic unsubscription failed"));
                }

                if response_chan.send(Ok(())).await.is_err() {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| warn!("response channel is closed"));
                }
            }
        }
    }

    #[cfg(feature = "tracing")]
    span.in_scope(|| trace!("spawning"));

    tokio::task::spawn(async move {
        // Input worker loop:
        //   1. Receive connection attempts and respond
        //   2. Receive messages and forward to channel
        //   3. Receive and handle disconnects
        loop {
            tokio::select! {
                Some(event) = to_worker_recv.recv() => {
                    handle_socket_event(event, &inputs, &pub_queue, input_port_id).await;
                }
                Some((request, response_chan)) = req_recv.recv() => {
                    handle_input_request(request, response_chan, &inputs, &pub_queue, &sub_queue, input_port_id).await;
                }
                else => break,
            };
        }

        #[cfg(feature = "tracing")]
        {
            let state = match inputs.read().await.get(&input_port_id) {
                Some(input) => Some(input.read().await.clone()),
                None => None,
            };
            span.in_scope(|| {
                trace!(
                    events_closed = to_worker_recv.is_closed(),
                    requests_closed = req_recv.is_closed(),
                    ?state,
                    "exited input worker loop"
                )
            });
        }
    });

    Ok(())
}
