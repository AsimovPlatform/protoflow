// This is free and unencumbered software released into the public domain.

use crate::{subscribe_topics, unsubscribe_topics, ZmqTransport, ZmqTransportEvent};
use protoflow_core::{
    prelude::{fmt, format, vec, BTreeMap, Bytes, String, ToString, Vec},
    InputPortID, OutputPortID, PortError, PortState,
};
use tokio::sync::{
    mpsc::{channel, Sender},
    RwLock,
};

#[cfg(feature = "tracing")]
use tracing::{debug, error, info, trace, trace_span, warn};

#[derive(Clone, Debug)]
pub enum ZmqOutputPortRequest {
    Close,
    Send(Bytes),
}

const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(200);
const DEFAULT_MAX_RETRIES: u64 = 10;

#[derive(Clone, Debug)]
pub enum ZmqOutputPortState {
    Open(
        // channel for connection requests from public `connect` method
        Sender<(InputPortID, Sender<Result<(), PortError>>)>,
        // channel for close requests from the public `close` method
        Sender<Sender<Result<(), PortError>>>,
        // channel used internally for events from socket
        Sender<ZmqTransportEvent>,
    ),
    Connected(
        // channel for public `send` and `close` methods, contained channel is for the ack back
        Sender<(ZmqOutputPortRequest, Sender<Result<(), PortError>>)>,
        // channel used internally for events from socket
        Sender<ZmqTransportEvent>,
        // id of the connected input port
        InputPortID,
    ),
    Closed,
}

impl fmt::Display for ZmqOutputPortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ZmqOutputPortState::*;
        match *self {
            Open(..) => write!(f, "Open"),
            Connected(.., ref id) => {
                write!(f, "Connected({:?})", isize::from(*id),)
            }
            Closed => write!(f, "Closed"),
        }
    }
}

impl ZmqOutputPortState {
    pub fn state(&self) -> PortState {
        use ZmqOutputPortState::*;
        match self {
            Open(..) => PortState::Open,
            Connected(..) => PortState::Connected,
            Closed => PortState::Closed,
        }
    }

    pub async fn event_sender(&self) -> Option<Sender<ZmqTransportEvent>> {
        use ZmqOutputPortState::*;
        match self {
            Open(.., sender) | Connected(.., sender, _) => Some(sender.clone()),
            Closed => None,
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

pub async fn output_port_event_sender(
    outputs: &RwLock<BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>>,
    id: OutputPortID,
) -> Option<Sender<ZmqTransportEvent>> {
    outputs
        .read()
        .await
        .get(&id)?
        .read()
        .await
        .event_sender()
        .await
}

pub fn start_output_worker(
    transport: &ZmqTransport,
    output_port_id: OutputPortID,
) -> Result<(), PortError> {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::output_port_worker", ?output_port_id);

    let (conn_send, mut conn_recv) = channel(1);
    let (close_send, mut close_recv) = channel(1);
    let (to_worker_send, mut to_worker_recv) = channel(1);

    {
        let mut outputs = transport.tokio.block_on(transport.outputs.write());
        if outputs.contains_key(&output_port_id) {
            return Err(PortError::Invalid(output_port_id.into()));
        }
        let state = ZmqOutputPortState::Open(conn_send, close_send, to_worker_send.clone());
        #[cfg(feature = "tracing")]
        span.in_scope(|| trace!("saving new state: {}", state));
        outputs.insert(output_port_id, RwLock::new(state));
    }

    let sub_queue = transport.sub_queue.clone();
    let pub_queue = transport.pub_queue.clone();
    let outputs = transport.outputs.clone();

    #[cfg(feature = "tracing")]
    span.in_scope(|| trace!("spawning"));

    tokio::task::spawn(async move {
        let (input_port_id, conn_confirm) = tokio::select! {
            Some((input_port_id, conn_confirm)) = conn_recv.recv() => (input_port_id, conn_confirm),
            Some(close_confirm) = close_recv.recv() => {
                let response = {
                    if let Some(output_state) = outputs.read().await.get(&output_port_id) {
                        let mut output_state = output_state.write().await;
                        debug_assert!(matches!(*output_state, ZmqOutputPortState::Open(..)));
                        *output_state = ZmqOutputPortState::Closed;
                        Ok(())
                    } else {
                        #[cfg(feature = "tracing")]
                        span.in_scope(|| error!("port state not found"));
                        Err(PortError::Invalid(output_port_id.into()))
                    }
                };

                let _ = close_confirm.try_send(response);
                return;
            }
            else => {
                // all senders have dropped, i.e. there's no connection request coming

                if let Some(output_state) = outputs.read().await.get(&output_port_id) {
                    let mut output_state = output_state.write().await;
                    debug_assert!(matches!(*output_state, ZmqOutputPortState::Open(..)));
                    *output_state = ZmqOutputPortState::Closed;
                }

                #[cfg(feature = "tracing")]
                debug!(parent: &span, "no connection or close request");
                return;
            }
        };

        #[cfg(feature = "tracing")]
        let span = trace_span!(parent: &span, "task", ?input_port_id);

        let topics = output_topics(output_port_id, input_port_id);
        if subscribe_topics(&topics, &sub_queue).await.is_err() {
            #[cfg(feature = "tracing")]
            span.in_scope(|| error!("topic subscription failed"));
            return;
        }

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

            if pub_queue
                .send(ZmqTransportEvent::Connect(output_port_id, input_port_id))
                .await
                .is_err()
            {
                #[cfg(feature = "tracing")]
                span.in_scope(|| error!("publish channel is closed"));
                return;
            }

            let Some(response) = to_worker_recv.recv().await else {
                #[cfg(feature = "tracing")]
                span.in_scope(|| error!("all senders to worker have dropped?"));
                return;
            };

            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!(?response, "got response"));

            use ZmqTransportEvent::*;
            match response {
                AckConnection(_, input_port_id) => {
                    let response = match outputs.read().await.get(&output_port_id) {
                        None => {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| error!("port state not found"));
                            Err(PortError::Invalid(output_port_id.into()))
                        }
                        Some(output_state) => {
                            let mut output_state = output_state.write().await;
                            debug_assert!(matches!(*output_state, ZmqOutputPortState::Open(..)));
                            *output_state = ZmqOutputPortState::Connected(
                                msg_req_send,
                                to_worker_send,
                                input_port_id,
                            );

                            #[cfg(feature = "tracing")]
                            span.in_scope(|| info!("Connected!"));

                            Ok(())
                        }
                    };

                    if conn_confirm.send(response).await.is_err() {
                        #[cfg(feature = "tracing")]
                        span.in_scope(|| warn!("connection confirmation channel is closed"));
                        // don't exit, proceed to send loop
                    }
                    drop(conn_confirm);

                    break;
                }
                _ => continue,
            }
        }

        let mut seq_id = 1;
        'send: while let Some((request, response_chan)) = msg_req_recv.recv().await {
            #[cfg(feature = "tracing")]
            let span = trace_span!(parent: &span, "send_loop", ?seq_id);

            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!(?request, "sending request"));

            let respond = |response| async {
                if response_chan.send(response).await.is_err() {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| warn!("response channel is closed"));
                }
            };

            match request {
                ZmqOutputPortRequest::Close => {
                    let response = pub_queue
                        .send(ZmqTransportEvent::CloseOutput(
                            output_port_id,
                            input_port_id,
                        ))
                        .await
                        .map_err(|e| PortError::Other(e.to_string()));
                    respond(response).await;
                    break 'send;
                }
                ZmqOutputPortRequest::Send(bytes) => {
                    let msg = ZmqTransportEvent::Message(
                        output_port_id,
                        input_port_id,
                        seq_id,
                        bytes.clone(),
                    );

                    let mut attempts = 0;
                    'retry: loop {
                        attempts += 1;

                        #[cfg(feature = "tracing")]
                        let span = trace_span!(parent: &span, "retry_loop", ?attempts);

                        if attempts >= DEFAULT_MAX_RETRIES {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| trace!("reached max send attempts"));
                            respond(Err(PortError::Disconnected)).await;
                            break 'send;
                        }

                        #[cfg(feature = "tracing")]
                        span.in_scope(|| trace!("attempting to send message"));

                        if pub_queue.send(msg.clone()).await.is_err() {
                            // the socket for publishing has closed, we won't be able to send any
                            // messages
                            respond(Err(PortError::Disconnected)).await;
                            break 'send;
                        }

                        'recv: loop {
                            #[cfg(feature = "tracing")]
                            let span = trace_span!(parent: &span, "recv_loop");

                            let timeout = tokio::time::sleep(DEFAULT_TIMEOUT);

                            let event = tokio::select! {
                                // after DEFAULT_TIMEOUT duration has passed since the last
                                // received event from the socket, retry
                                _ = timeout => continue 'retry,
                                event_opt = to_worker_recv.recv() => match event_opt {
                                    Some(event) => event,
                                    None => {
                                        #[cfg(feature = "tracing")]
                                        span.in_scope(|| error!("all senders to worker have dropped"));
                                        respond(Err(PortError::Invalid(output_port_id.into()))).await;
                                        break 'send;
                                    }
                                }
                            };

                            #[cfg(feature = "tracing")]
                            span.in_scope(|| trace!(?event, "received event"));

                            use ZmqTransportEvent::*;
                            match event {
                                AckMessage(_, _, ack_id) => {
                                    if ack_id == seq_id {
                                        #[cfg(feature = "tracing")]
                                        span.in_scope(|| trace!(?ack_id, "msg-ack matches"));
                                        respond(Ok(())).await;
                                        break 'retry;
                                    } else {
                                        #[cfg(feature = "tracing")]
                                        span.in_scope(|| {
                                            trace!(?ack_id, "got msg-ack for different sequence")
                                        });
                                        continue 'recv;
                                    }
                                }
                                CloseInput(_) => {
                                    // report that the input port was closed
                                    respond(Err(PortError::Disconnected)).await;
                                    break 'send;
                                }

                                // ignore others, we shouldn't receive any new conn-acks
                                // nor should we be receiving input port events
                                AckConnection(..) | Connect(..) | Message(..) | CloseOutput(..) => {
                                    continue 'recv
                                }
                            }
                        }
                    }
                }
            }

            seq_id += 1;
        }

        let outputs = outputs.read().await;
        let Some(output_state) = outputs.get(&output_port_id) else {
            #[cfg(feature = "tracing")]
            span.in_scope(|| error!("port state not found"));
            return;
        };
        let mut output_state = output_state.write().await;
        debug_assert!(matches!(*output_state, ZmqOutputPortState::Connected(..)));
        *output_state = ZmqOutputPortState::Closed;

        #[cfg(feature = "tracing")]
        span.in_scope(|| {
            trace!(
                events_closed = to_worker_recv.is_closed(),
                requests_closed = msg_req_recv.is_closed(),
                state = ?*output_state,
                "exited output worker loop"
            )
        });

        if unsubscribe_topics(&topics, &sub_queue).await.is_err() {
            #[cfg(feature = "tracing")]
            span.in_scope(|| error!("topic unsubscription failed"));
        }
    });

    Ok(())
}
