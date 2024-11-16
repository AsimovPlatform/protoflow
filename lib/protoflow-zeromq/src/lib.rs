// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use protoflow_core::prelude;

extern crate std;

use protoflow_core::{prelude::Bytes, InputPortID, OutputPortID, PortResult, PortState, Transport};

use zeromq::{Socket, SocketRecv, SocketSend};

pub struct ZMQTransport {
    psock: zeromq::PubSocket,
    ssock: zeromq::SubSocket,
    tokio: tokio::runtime::Handle,
}

impl ZMQTransport {
    pub fn new(url: &str) -> Self {
        let tokio = tokio::runtime::Handle::current();
        let mut psock = zeromq::PubSocket::new();
        tokio.block_on(psock.connect(url)).expect("psock conn");
        let mut ssock = zeromq::SubSocket::new();
        tokio.block_on(ssock.connect(url)).expect("ssock conn");
        Self {
            psock,
            ssock,
            tokio,
        }
    }
}

impl Transport for ZMQTransport {
    fn input_state(&self, input: InputPortID) -> PortResult<PortState> {
        todo!();
    }

    fn output_state(&self, output: OutputPortID) -> PortResult<PortState> {
        todo!();
    }

    fn open_input(&self) -> PortResult<InputPortID> {
        todo!();
    }

    fn open_output(&self) -> PortResult<OutputPortID> {
        todo!();
    }

    fn close_input(&self, input: InputPortID) -> PortResult<bool> {
        todo!();
    }

    fn close_output(&self, output: OutputPortID) -> PortResult<bool> {
        todo!();
    }

    fn connect(&self, source: OutputPortID, target: InputPortID) -> PortResult<bool> {
        todo!();
    }

    fn send(&self, output: OutputPortID, message: Bytes) -> PortResult<()> {
        todo!();
    }

    fn recv(&self, input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!();
    }

    fn try_recv(&self, _input: InputPortID) -> PortResult<Option<Bytes>> {
        todo!();
    }
}
