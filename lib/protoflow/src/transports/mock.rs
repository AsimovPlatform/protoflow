// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Box, Vec},
    transport::{Receiver, Sender, Transport},
    Message,
};

#[derive(Debug, Default)]
pub struct MockTransport;

impl MockTransport {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Transport for MockTransport {}

#[derive(Debug, Default)]
pub struct MockSender<T> {
    pub messages: Option<Vec<T>>,
}

impl<T: Message> MockSender<T> {
    pub fn new() -> Self {
        Self {
            messages: Some(Vec::new()),
        }
    }
}

impl<T: Message> Sender<T> for MockSender<T> {
    fn send(&mut self, message: T) -> Result<(), ()> {
        if let Some(messages) = &mut self.messages {
            messages.push(message);
            Ok(())
        } else {
            Err(())
        }
    }

    fn close(&mut self) -> Result<(), ()> {
        self.messages = None;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.messages.is_none()
    }
}

#[derive(Debug, Default)]
pub struct MockReceiver<T> {
    pub messages: Option<Vec<T>>,
}

impl<T: Message> MockReceiver<T> {
    pub fn new(messages: Option<Vec<T>>) -> Self {
        Self { messages }
    }
}

impl<T: Message> Receiver<T> for MockReceiver<T> {
    fn recv(&mut self) -> Result<T, ()> {
        self.messages
            .as_mut()
            .and_then(|messages| messages.pop())
            .ok_or(())
    }

    fn close(&mut self) -> Result<(), ()> {
        self.messages = None;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.messages.is_none()
    }
}
