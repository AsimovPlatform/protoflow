extern crate std;

use crate::{IoBlocks, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    types::Any, Block, BlockError, BlockResult, BlockRuntime, Message, OutputPort, Port,
    PortResult, SystemBuilding,
};
use protoflow_derive::Block;
use tracing::{error, info};

use crate::prelude::{vec, String};

use serde::{Deserialize, Serialize};

#[derive(Block, Clone)]
pub struct ReadSocket<T: protoflow_core::Message = Any> {
    #[output]
    pub output: OutputPort<T>,
    #[parameter]
    pub config: ReadSocketConfig,
    #[cfg(feature = "std")]
    pub listener: std::sync::Arc<std::sync::Mutex<Option<std::net::TcpListener>>>,
    #[cfg(feature = "std")]
    pub stream: std::sync::Arc<std::sync::Mutex<Option<std::net::TcpStream>>>,
}

impl<T: protoflow_core::Message> ReadSocket<T> {
    pub fn with_params(output: OutputPort<T>, config: Option<ReadSocketConfig>) -> Self {
        Self {
            output,
            config: config.unwrap_or(Default::default()),
            listener: std::sync::Arc::new(std::sync::Mutex::new(None)),
            stream: std::sync::Arc::new(std::sync::Mutex::new(None)),
        }
    }
}

impl<T: protoflow_core::Message + 'static> ReadSocket<T> {
    pub fn with_system(system: &System, config: Option<ReadSocketConfig>) -> Self {
        Self::with_params(system.output(), config)
    }
}

impl<T: protoflow_core::Message + std::string::ToString + 'static> StdioSystem for ReadSocket<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        config.allow_only(vec!["host", "port", "buffer_size"])?;

        let host_string = config.get_string("host")?;
        let port: u16 = config.get("port")?;
        let buffer_size: usize = config.get("buffer_size")?;

        Ok(System::build(|s| {
            let line_encoder = s.encode_with(config.encoding);
            let stdout = config.write_stdout(s);
            let read_socket: ReadSocket<T> = s.block(ReadSocket::with_system(
                s,
                Some(ReadSocketConfig {
                    host: host_string,
                    port,
                    buffer_size,
                }),
            ));
            s.connect(&read_socket.output, &line_encoder.input);
            s.connect(&line_encoder.output, &stdout.input);
        }))
    }
}

impl<T: protoflow_core::Message + 'static> Block for ReadSocket<T> {
    fn prepare(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let address = std::format!("{}:{}", &self.config.host, &self.config.port);
        let listener = std::net::TcpListener::bind(&address)?;
        *self.listener.lock().map_err(lock_error)? = Some(listener);
        info!("Server listening on {}", address);
        Ok(())
    }

    fn execute(&mut self, _: &dyn BlockRuntime) -> BlockResult {
        let mut stream_guard = self.stream.lock().map_err(lock_error)?;

        if stream_guard.is_none() {
            let listener_lock = self.listener.lock().map_err(lock_error)?;
            let listener = listener_lock
                .as_ref()
                .ok_or(BlockError::Other("Invalid TCP listener".into()))?;

            let (stream, addr) = listener.accept().map_err(|e| {
                error!("Failed to accept client connection: {}", e);
                BlockError::Other("Failed to accept client connection".into())
            })?;

            info!("Accepted connection from {}", addr);
            *stream_guard = Some(stream);
        }

        if let Some(stream) = stream_guard.as_mut() {
            handle_client::<T, _>(stream, self.config.buffer_size, |message| {
                info!("Processing received message");
                if self.output.is_connected() {
                    self.output.send(message)?;
                }
                Ok(())
            })
            .map_err(|e| {
                error!("Error handling client: {}", e);
                BlockError::Other("Error handling client".into())
            })?;
        }

        Ok(())
    }
}

fn lock_error<T>(err: std::sync::PoisonError<T>) -> BlockError {
    BlockError::Other(std::format!("Failed to acquire lock: {:?}", err))
}

fn handle_client<M, F>(
    stream: &mut std::net::TcpStream,
    buffer_size: usize,
    process_fn: F,
) -> Result<(), BlockError>
where
    M: Message + Default,
    F: Fn(&M) -> PortResult<()>,
{
    let mut buffer = vec![0; buffer_size];

    while let Ok(bytes_read) = std::io::Read::read(stream, &mut buffer) {
        if bytes_read == 0 {
            info!("Client disconnected");
            break;
        }

        let message = M::decode(&buffer[..bytes_read])
            .map_err(|_| BlockError::Other("Failed to decode message".into()))?;

        info!("Received message: {:?}", message);

        process_fn(&message).map_err(|e| {
            error!("Failed to send response: {:?}", e);
            BlockError::Other("Failed to send response".into())
        })?;
    }

    Ok(())
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadSocketConfig {
    pub host: String,
    pub port: u16,
    pub buffer_size: usize,
}

impl Default for ReadSocketConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 7777,
            buffer_size: 512,
        }
    }
}
