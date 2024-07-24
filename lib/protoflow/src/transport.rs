// This is free and unencumbered software released into the public domain.

use crate::Message;

/// A trait for sending messages.
pub trait Sender<M: Message> {
    /// Sends a message.
    fn send(&mut self, message: M) -> Result<(), ()>;

    /// Closes the sender.
    fn close(&mut self) -> Result<(), ()>;

    /// Returns whether the sender is closed.
    fn is_closed(&self) -> bool;
}

/// A trait for receiving messages.
pub trait Receiver<M: Message> {
    /// Receives a message.
    fn recv(&mut self) -> Result<M, ()>;

    /// Closes the receiver.
    fn close(&mut self) -> Result<(), ()>;

    /// Returns whether the receiver is closed.
    fn is_closed(&self) -> bool;
}
