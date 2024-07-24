use futures::TryFutureExt;
use prost::Message;
use tokio::{runtime::Handle, task};
use zeromq::{PullSocket, PushSocket, Socket, SocketRecv, SocketSend};

// This is free and unencumbered software released into the public domain.
use crate::transport::{Receiver, Sender};

pub struct ZmqSender {
    socket: Option<PushSocket>,
}

impl ZmqSender {
    pub fn open(endpoint: &str) -> Result<Self, ()> {
        let mut socket = PushSocket::new();
        let socket = task::block_in_place(|| {
            Handle::current().block_on(async {
                socket.connect(endpoint).map_err(|_| ()).await?;
                Ok(socket)
            })
        })?;
        Ok(Self {
            socket: Some(socket),
        })
    }
}

impl<M: Message> Sender<M> for ZmqSender {
    fn send(&mut self, message: M) -> Result<(), ()> {
        task::block_in_place(move || {
            Handle::current().block_on(async move {
                if let Some(socket) = &mut self.socket {
                    let bytes = message.encode_to_vec();
                    socket.send(bytes.into()).await.map_err(|_| ())
                } else {
                    Err(())
                }
            })
        })
    }

    fn close(&mut self) -> Result<(), ()> {
        let socket = self.socket.take();
        if let Some(socket) = socket {
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    let _ = socket.close().await;
                    Ok(())
                })
            })
        } else {
            Err(())
        }
    }

    fn is_closed(&self) -> bool {
        self.socket.is_none()
    }
}

pub struct ZmqReceiver {
    socket: Option<PullSocket>,
}

impl ZmqReceiver {
    pub fn open(endpoint: &str) -> Result<Self, ()> {
        let mut socket = PullSocket::new();
        let socket = task::block_in_place(|| {
            Handle::current().block_on(async {
                socket.bind(endpoint).map_err(|_| ()).await?;
                Ok(socket)
            })
        })?;
        Ok(Self {
            socket: Some(socket),
        })
    }
}

impl<M: Message + Default> Receiver<M> for ZmqReceiver {
    fn recv(&mut self) -> Result<M, ()> {
        task::block_in_place(move || {
            Handle::current().block_on(async move {
                if let Some(socket) = &mut self.socket {
                    socket
                        .recv()
                        .await
                        .map_err(|_| ())?
                        .get(0)
                        .map(|b| M::decode(b.as_ref()).map_err(|_| ()))
                        .ok_or(())?
                } else {
                    Err(())
                }
            })
        })
    }

    fn close(&mut self) -> Result<(), ()> {
        let socket = self.socket.take();
        if let Some(socket) = socket {
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    let _ = socket.close().await;
                    Ok(())
                })
            })
        } else {
            Err(())
        }
    }

    fn is_closed(&self) -> bool {
        self.socket.is_none()
    }
}
