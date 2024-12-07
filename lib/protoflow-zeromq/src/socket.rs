// This is free and unencumbered software released into the public domain.

use crate::{
    input_port_event_sender, output_port_event_sender, ZmqInputPortState, ZmqOutputPortState,
    ZmqTransport, ZmqTransportEvent,
};
use protoflow_core::{
    prelude::{BTreeMap, String, Vec},
    InputPortID, OutputPortID, PortError,
};
use tokio::sync::{
    mpsc::{error::SendError, Receiver, Sender},
    RwLock,
};
use zeromq::{SocketRecv, SocketSend, ZmqMessage};

#[derive(Clone, Debug)]
pub enum ZmqSubscriptionRequest {
    Subscribe(String),
    Unsubscribe(String),
}

#[cfg(feature = "tracing")]
use tracing::{debug, error, trace, trace_span, warn};

pub fn start_pub_socket_worker(
    transport: &ZmqTransport,
    psock: zeromq::PubSocket,
    pub_queue: Receiver<ZmqTransportEvent>,
) {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::pub_socket");
    let outputs = transport.outputs.clone();
    let inputs = transport.inputs.clone();
    let mut psock = psock;
    let mut pub_queue = pub_queue;
    tokio::task::spawn(async move {
        while let Some(event) = pub_queue.recv().await {
            #[cfg(feature = "tracing")]
            span.in_scope(|| trace!(?event, "sending event to socket"));

            use ZmqTransportEvent::*;
            let shortcut_sender = match event {
                Connect(_, id) | Message(_, id, _, _) | CloseOutput(_, id) => {
                    input_port_event_sender(&inputs, id).await
                }
                AckConnection(id, _) | AckMessage(id, ..) => {
                    output_port_event_sender(&outputs, id).await
                }
                CloseInput(..) => None,
            };

            if let Some(sender) = shortcut_sender {
                #[cfg(feature = "tracing")]
                span.in_scope(|| debug!("attempting to shortcut send directly to target port"));
                if sender.send(event.clone()).await.is_ok() {
                    continue;
                }
                #[cfg(feature = "tracing")]
                span.in_scope(|| warn!("failed to send message with shortcut, sending to socket"));
            }

            if let Err(err) = psock.send(event.into()).await {
                #[cfg(feature = "tracing")]
                span.in_scope(|| error!(?err, "failed to send message"));
            }
        }
    });
}

pub async fn subscribe_topics(
    topics: &[String],
    sub_queue: &Sender<ZmqSubscriptionRequest>,
) -> Result<(), SendError<ZmqSubscriptionRequest>> {
    let mut handles = Vec::with_capacity(topics.len());
    for topic in topics {
        handles.push(sub_queue.send(ZmqSubscriptionRequest::Subscribe(topic.clone())));
    }
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

pub async fn unsubscribe_topics(
    topics: &[String],
    sub_queue: &Sender<ZmqSubscriptionRequest>,
) -> Result<(), SendError<ZmqSubscriptionRequest>> {
    let mut handles = Vec::with_capacity(topics.len());
    for topic in topics {
        handles.push(sub_queue.send(ZmqSubscriptionRequest::Unsubscribe(topic.clone())));
    }
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

pub fn start_sub_socket_worker(
    transport: &ZmqTransport,
    ssock: zeromq::SubSocket,
    sub_queue: Receiver<ZmqSubscriptionRequest>,
) {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::sub_socket");
    let outputs = transport.outputs.clone();
    let inputs = transport.inputs.clone();
    let mut ssock = ssock;
    let mut sub_queue = sub_queue;
    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                Ok(msg) = ssock.recv() => {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| trace!(?msg, "got message from socket"));

                    if let Err(err) = handle_zmq_msg(msg, &outputs, &inputs).await {
                        #[cfg(feature = "tracing")]
                        span.in_scope(|| error!(?err, "failed to process message"));
                    }
                },
                Some(req) = sub_queue.recv() => {
                    #[cfg(feature = "tracing")]
                    span.in_scope(|| trace!(?req,  "got sub update request"));

                    use ZmqSubscriptionRequest::*;
                    match req {
                        Subscribe(topic) => if let Err(err) = ssock.subscribe(&topic).await {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| error!(?err, ?topic, "subscribe failed"));
                        },
                        Unsubscribe(topic) => if let Err(err) = ssock.unsubscribe(&topic).await {
                            #[cfg(feature = "tracing")]
                            span.in_scope(|| error!(?err, ?topic, "unsubscribe failed"));
                        }
                    };
                }
            };
        }
    });
}

async fn handle_zmq_msg(
    msg: ZmqMessage,
    outputs: &RwLock<BTreeMap<OutputPortID, RwLock<ZmqOutputPortState>>>,
    inputs: &RwLock<BTreeMap<InputPortID, RwLock<ZmqInputPortState>>>,
) -> Result<(), PortError> {
    #[cfg(feature = "tracing")]
    let span = trace_span!("ZmqTransport::handle_zmq_msg");

    let event = ZmqTransportEvent::try_from(msg)?;

    #[cfg(feature = "tracing")]
    span.in_scope(|| trace!(?event, "got event"));

    use ZmqTransportEvent::*;
    match event {
        // input ports
        Connect(_, input_port_id)
        | Message(_, input_port_id, _, _)
        | CloseOutput(_, input_port_id) => {
            let sender = input_port_event_sender(inputs, input_port_id)
                .await
                .ok_or_else(|| PortError::Invalid(input_port_id.into()))?;

            sender.send(event).await.map_err(|_| PortError::Closed)
        }

        // output ports
        AckConnection(output_port_id, _) | AckMessage(output_port_id, _, _) => {
            let sender = output_port_event_sender(outputs, output_port_id)
                .await
                .ok_or_else(|| PortError::Invalid(output_port_id.into()))?;

            sender.send(event).await.map_err(|_| PortError::Closed)
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
            Ok(())
        }
    }
}
