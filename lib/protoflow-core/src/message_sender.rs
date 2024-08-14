// This is free and unencumbered software released into the public domain.

//! Common methods for sending messages.

use crate::{prelude::ToString, Message, PortError, PortResult};

pub trait MessageSender<T: Message> {
    /// Sends a message, blocking until it has been sent.
    ///
    /// Returns `Ok(())` if the message was sent.
    /// Returns `Err(PortError::Disconnected)` if the port is disconnected.
    /// Returns `Err(PortError::Closed)` if the port is closed.
    /// Returns `Err(PortError)` if another error occurs.
    fn send<'a>(&self, _message: impl Into<&'a T>) -> PortResult<()>
    where
        T: 'a,
    {
        Err(PortError::Other("not implemented".to_string()))
    }

    /// Tries to send a message, returning immediately.
    ///
    /// Returns `Ok(true)` if the message was sent.
    /// Returns `Ok(false)` if the message could not be immediately sent.
    /// Returns `Err(PortError::Disconnected)` if the port is disconnected.
    /// Returns `Err(PortError::Closed)` if the port is closed.
    /// Returns `Err(PortError)` if another error occurs.
    fn try_send<'a>(&self, _message: impl Into<&'a T>) -> PortResult<bool>
    where
        T: 'a,
    {
        Err(PortError::Other("not implemented".to_string()))
    }
}
