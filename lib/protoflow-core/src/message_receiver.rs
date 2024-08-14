// This is free and unencumbered software released into the public domain.

//! Common methods for receiving messages.

use crate::{prelude::ToString, Message, PortError, PortResult};

pub trait MessageReceiver<T: Message> {
    /// Receives a message, blocking until one is available.
    ///
    /// Returns `Ok(Some(message))` if a message was received.
    /// Returns `Ok(None)` if the port is closed or disconnected.
    /// Returns `Err(PortError)` if an error occurs.
    fn recv(&self) -> PortResult<Option<T>> {
        Err(PortError::Other("not implemented".to_string()))
    }

    /// Tries to receive a message, returning immediately.
    ///
    /// Returns `Ok(Some(message))` if a message was received.
    /// Returns `Ok(None)` if no message was immediately available.
    /// Returns `Err(PortError::Disconnected)` if the port is disconnected.
    /// Returns `Err(PortError::Closed)` if the port is closed.
    /// Returns `Err(PortError)` if another error occurs.
    fn try_recv(&self) -> PortResult<Option<T>> {
        Err(PortError::Other("not implemented".to_string()))
    }
}
