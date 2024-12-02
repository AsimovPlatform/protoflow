// This is free and unencumbered software released into the public domain.

use crate::{ZmqSubscriptionRequest, ZmqTransport, ZmqTransportEvent};
use protoflow_core::{
    prelude::{format, vec, Arc, BTreeMap, Bytes, String, Vec},
    InputPortID, OutputPortID, PortError, PortState,
};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex, RwLock,
};

#[cfg(feature = "tracing")]
use tracing::{trace, trace_span};

#[derive(Debug, Clone)]
pub enum ZmqInputPortRequest {
    Close,
}

/// ZmqInputPortEvent represents events that we receive from the background worker of the port.
#[derive(Clone, Debug)]
pub enum ZmqInputPortEvent {
    Message(Bytes),
    Closed,
}

#[derive(Debug, Clone)]
pub enum ZmqInputPortState {
    Open(Sender<ZmqTransportEvent>),
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

impl ZmqInputPortState {
    pub fn state(&self) -> PortState {
        use ZmqInputPortState::*;
        match self {
            Open(_) => PortState::Open,
            Connected(_, _, _, _, _) => PortState::Connected,
            Closed => PortState::Closed,
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

pub fn start_input_worker(
    transport: &ZmqTransport,
    input_port_id: InputPortID,
) -> Result<(), PortError> {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::start_input_worker", ?input_port_id);

    let (to_worker_send, mut to_worker_recv) = channel(1);

    let (req_send, mut req_recv) = channel(1);

    {
        let mut inputs = transport.tokio.block_on(transport.inputs.write());
        if inputs.contains_key(&input_port_id) {
            return Ok(()); // TODO
        }
        let state = ZmqInputPortState::Open(to_worker_send.clone());
        let state = RwLock::new(state);

        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!(?state, "saving new opened state"));

        inputs.insert(input_port_id, state);
    }

    {
        let mut handles = Vec::new();
        for topic in input_topics(input_port_id).into_iter() {
            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!(?topic, "sending subscription request"));

            let handle = transport
                .sub_queue
                .send(ZmqSubscriptionRequest::Subscribe(topic));
            handles.push(handle);
        }
        for handle in handles.into_iter() {
            transport
                .tokio
                .block_on(handle)
                .expect("input worker send sub req");
        }
    }

    let sub_queue = transport.sub_queue.clone();
    let pub_queue = transport.pub_queue.clone();
    let inputs = transport.inputs.clone();

    async fn handle_socket_event(
        event: ZmqTransportEvent,
        inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
        req_send: &Sender<(ZmqInputPortRequest, Sender<Result<(), PortError>>)>,
        pub_queue: &Sender<ZmqTransportEvent>,
        input_port_id: InputPortID,
    ) {
        #[cfg(feature = "tracing")]
        let span = trace_span!(
            "ZmqTransport::start_input_worker::handle_socket_event",
            ?input_port_id
        );

        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!(?event, "got socket event"));

        use ZmqTransportEvent::*;
        match event {
            Connect(output_port_id, input_port_id) => {
                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    todo!();
                };
                let mut input_state = input_state.write().await;

                use ZmqInputPortState::*;
                match &*input_state {
                    Open(_) => (),
                    Connected(_, _, _, _, connected_ids) => {
                        if connected_ids.iter().any(|&id| id == output_port_id) {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| {
                                trace!(?output_port_id, "output port is already connected")
                            });
                            return;
                        }
                    }
                    Closed => return,
                };

                let add_connection = |input_state: &mut ZmqInputPortState| match input_state {
                    Open(to_worker_send) => {
                        let (msgs_send, msgs_recv) = channel(1);
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

                #[cfg(feature = "tracing")]
                span.in_scope(|| trace!(?output_port_id, "sent conn-ack"));

                add_connection(&mut input_state);

                #[cfg(feature = "tracing")]
                span.in_scope(|| trace!(?input_state, "connected new port"));
            }
            Message(output_port_id, _, seq_id, bytes) => {
                #[cfg(feature = "tracing")]
                let span = trace_span!(parent: &span, "Message", ?output_port_id, ?seq_id);

                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    todo!();
                };
                let input_state = input_state.read().await;

                use ZmqInputPortState::*;
                match &*input_state {
                    Connected(_, sender, _, _, connected_ids) => {
                        if !connected_ids.iter().any(|id| *id == output_port_id) {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| trace!("got message from non-connected output port"));
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

                        #[cfg(feature = "tracing")]
                        span.in_scope(|| trace!("sent msg-ack"));
                    }

                    Open(_) | Closed => todo!(),
                }
            }
            CloseOutput(output_port_id, input_port_id) => {
                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    todo!();
                };

                let mut input_state = input_state.write().await;

                use ZmqInputPortState::*;
                let Connected(_, _, _, _, ref connected_ids) = *input_state else {
                    return;
                };

                if !connected_ids.iter().any(|id| *id == output_port_id) {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| {
                        trace!(
                            ?output_port_id,
                            "output port doesn't match any connected port"
                        )
                    });
                    return;
                }

                match *input_state {
                    Open(_) | Closed => (),
                    Connected(_, ref sender, _, _, ref mut connected_ids) => {
                        connected_ids.retain(|&id| id != output_port_id);
                        if connected_ids.is_empty() {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| trace!("last connected port disconnected"));
                            sender
                                .send(ZmqInputPortEvent::Closed)
                                .await
                                .expect("input worker publish Closed event");
                        }
                    }
                };
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
        sub_queue: &Sender<ZmqSubscriptionRequest>,
        input_port_id: InputPortID,
    ) {
        #[cfg(feature = "tracing")]
        let span = trace_span!(
            "ZmqTransport::start_input_worker::handle_input_event",
            ?input_port_id
        );

        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!(?request, "got input request"));

        use ZmqInputPortRequest::*;
        match request {
            Close => {
                let inputs = inputs.read().await;
                let Some(input_state) = inputs.get(&input_port_id) else {
                    todo!();
                };
                let mut input_state = input_state.write().await;

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

                *input_state = ZmqInputPortState::Closed;

                {
                    let mut handles = Vec::new();
                    for topic in input_topics(input_port_id).into_iter() {
                        #[cfg(feature = "tracing")]
                        span.in_scope(|| trace!(?topic, "sending unsubscription request"));

                        let handle = sub_queue.send(ZmqSubscriptionRequest::Unsubscribe(topic));
                        handles.push(handle);
                    }
                    for handle in handles.into_iter() {
                        handle.await.expect("input worker send unsub req");
                    }
                }

                response_chan
                    .send(Ok(()))
                    .await
                    .expect("input worker respond close")
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
                    handle_socket_event(event, &inputs, &req_send, &pub_queue, input_port_id).await;
                }
                Some((request, response_chan)) = req_recv.recv() => {
                    handle_input_request(request, response_chan, &inputs, &pub_queue, &sub_queue, input_port_id).await;
                }
            };
        }
    });

    Ok(())
}
