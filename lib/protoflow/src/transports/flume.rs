// This is free and unencumbered software released into the public domain.

use crate::{
    transport::{Receiver, Sender, Transport},
    Message,
};

#[derive(Debug, Default)]
pub struct FlumeTransport;

impl Transport for FlumeTransport {}

#[derive(Debug, Default)]
pub struct FlumeSender<M> {
    sender: Option<flume::Sender<M>>,
}

impl<M: Message> FlumeSender<M> {
    pub fn new() -> Self {
        Self { sender: None }
    }

    pub fn open(sender: flume::Sender<M>) -> Self {
        Self {
            sender: Some(sender),
        }
    }
}

impl<M: Message> Sender<M> for FlumeSender<M> {
    fn send(&mut self, message: M) -> Result<(), ()> {
        if let Some(sender) = &self.sender {
            sender.send(message).map_err(|_e| ())
        } else {
            Err(())
        }
    }

    fn close(&mut self) -> Result<(), ()> {
        let sender = self.sender.take();
        if let Some(sender) = sender {
            drop(sender);
            Ok(())
        } else {
            Err(())
        }
    }

    fn is_closed(&self) -> bool {
        if let Some(sender) = &self.sender {
            sender.is_disconnected()
        } else {
            true
        }
    }
}

#[derive(Debug, Default)]
pub struct FlumeReceiver<T> {
    receiver: Option<flume::Receiver<T>>,
}

impl<M: Message> FlumeReceiver<M> {
    pub fn new() -> Self {
        Self { receiver: None }
    }

    pub fn open(receiver: flume::Receiver<M>) -> Self {
        Self {
            receiver: Some(receiver),
        }
    }
}

impl<M: Message> Receiver<M> for FlumeReceiver<M> {
    fn recv(&mut self) -> Result<M, ()> {
        if let Some(receiver) = &self.receiver {
            receiver.recv().map_err(|_e| ())
        } else {
            Err(())
        }
    }

    fn close(&mut self) -> Result<(), ()> {
        let receiver = self.receiver.take();
        if let Some(receiver) = receiver {
            drop(receiver);
            Ok(())
        } else {
            Err(())
        }
    }

    fn is_closed(&self) -> bool {
        if let Some(receiver) = &self.receiver {
            receiver.is_disconnected()
        } else {
            true
        }
    }
}
