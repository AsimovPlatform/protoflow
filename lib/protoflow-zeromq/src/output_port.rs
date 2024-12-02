// This is free and unencumbered software released into the public domain.

use crate::{subscribe_topics, unsubscribe_topics, ZmqTransport, ZmqTransportEvent};
use protoflow_core::{
    prelude::{format, vec, Bytes, String, ToString, Vec},
    InputPortID, OutputPortID, PortError, PortState,
};
use tokio::sync::{
    mpsc::{channel, Sender},
    RwLock,
};

#[cfg(feature = "tracing")]
use tracing::{trace, trace_span};

#[derive(Debug, Clone)]
pub enum ZmqOutputPortRequest {
    Close,
    Send(Bytes),
}
#[derive(Debug, Clone)]
pub enum ZmqOutputPortState {
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

impl ZmqOutputPortState {
    pub fn state(&self) -> PortState {
        use ZmqOutputPortState::*;
        match self {
            Open(_, _) => PortState::Open,
            Connected(_, _, _) => PortState::Connected,
            Closed => PortState::Closed,
        }
    }
}

fn output_topics(source: OutputPortID, target: InputPortID) -> Vec<String> {
    vec![
        format!("{}:ackConn:{}", target, source),
        format!("{}:ackMsg:{}:", target, source),
        format!("{}:closeIn", target),
    ]
}

pub fn start_output_worker(
    transport: &ZmqTransport,
    output_port_id: OutputPortID,
) -> Result<(), PortError> {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::start_output_worker", ?output_port_id);

    let (conn_send, mut conn_recv) = channel(1);

    let (to_worker_send, mut to_worker_recv) = channel(1);

    {
        let mut outputs = transport.tokio.block_on(transport.outputs.write());

        if outputs.contains_key(&output_port_id) {
            return Ok(()); // TODO
        }
        let state = ZmqOutputPortState::Open(conn_send, to_worker_send.clone());
        let state = RwLock::new(state);

        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!(?state, "saving new opened state"));

        outputs.insert(output_port_id, state);
    }

    let sub_queue = transport.sub_queue.clone();
    let pub_queue = transport.pub_queue.clone();
    let outputs = transport.outputs.clone();

    #[cfg(feature = "tracing")]
    span.in_scope(|| trace!("spawning"));

    tokio::task::spawn(async move {
        let Some((input_port_id, conn_confirm)) = conn_recv.recv().await else {
            // all senders have dropped, i.e. there's no connection request coming
            return;
        };

        #[cfg(feature = "tracing")]
        let span = trace_span!(
            "ZmqTransport::start_output_worker::spawn",
            ?output_port_id,
            ?input_port_id
        );

        let topics = output_topics(output_port_id, input_port_id);
        subscribe_topics(&topics, &sub_queue).await.unwrap();

        let (msg_req_send, mut msg_req_recv) = channel(1);

        // Output worker loop:
        //   1. Send connection attempt
        //   2. Send messages
        //     2.1 Wait for ACK
        //     2.2. Resend on timeout
        //   3. Send disconnect events

        loop {
            #[cfg(feature = "tracing")]
            let span = trace_span!(parent: &span, "connect_loop");

            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!("sending connection attempt..."));

            pub_queue
                .send(ZmqTransportEvent::Connect(output_port_id, input_port_id))
                .await
                .expect("output worker send connect event");

            let response = to_worker_recv
                .recv()
                .await
                .expect("output worker recv ack-conn event");

            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!(?response, "got response"));

            use ZmqTransportEvent::*;
            match response {
                AckConnection(_, input_port_id) => {
                    let outputs = outputs.read().await;
                    let Some(output_state) = outputs.get(&output_port_id) else {
                        todo!();
                    };
                    let mut output_state = output_state.write().await;
                    debug_assert!(matches!(*output_state, ZmqOutputPortState::Open(..)));
                    *output_state =
                        ZmqOutputPortState::Connected(msg_req_send, to_worker_send, input_port_id);

                    #[cfg(feature = "tracing")]
                    span.in_scope(|| trace!(?output_state, "Connected!"));

                    conn_confirm
                        .send(Ok(()))
                        .await
                        .expect("output worker respond conn");
                    drop(conn_confirm);

                    break;
                }
                _ => continue,
            }
        }

        let mut seq_id = 1;
        'send: loop {
            #[cfg(feature = "tracing")]
            let span = trace_span!(parent: &span, "send_loop", ?seq_id);

            let (request, response_chan) = msg_req_recv
                .recv()
                .await
                .expect("output worker recv msg req");

            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!(?request, "sending request"));

            match request {
                ZmqOutputPortRequest::Close => {
                    let response = pub_queue
                        .send(ZmqTransportEvent::CloseOutput(
                            output_port_id,
                            input_port_id,
                        ))
                        .await
                        .map_err(|e| PortError::Other(e.to_string()));

                    unsubscribe_topics(&topics, &sub_queue).await.unwrap();

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

                    'recv: loop {
                        let event = to_worker_recv
                            .recv()
                            .await
                            .expect("output worker event recv");

                        #[cfg(feature = "tracing")]
                        span.in_scope(|| trace!(?event, "received event"));

                        use ZmqTransportEvent::*;
                        match event {
                            AckMessage(_, _, ack_id) => {
                                if ack_id == seq_id {
                                    #[cfg(feature = "tracing")]
                                    span.in_scope(|| trace!(?ack_id, "msg-ack matches"));
                                    response_chan
                                        .send(Ok(()))
                                        .await
                                        .expect("output worker respond send");
                                    break 'recv;
                                }
                            }
                            CloseInput(_) => {
                                let outputs = outputs.read().await;
                                let Some(output_state) = outputs.get(&output_port_id) else {
                                    todo!();
                                };
                                let mut output_state = output_state.write().await;
                                debug_assert!(matches!(
                                    *output_state,
                                    ZmqOutputPortState::Connected(..)
                                ));
                                *output_state = ZmqOutputPortState::Closed;

                                unsubscribe_topics(&topics, &sub_queue).await.unwrap();

                                response_chan
                                    .send(Err(PortError::Disconnected))
                                    .await
                                    .expect("output worker respond msg");

                                break 'send;
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
