extern crate std;
use crate::prelude::{vec, String};
use crate::{IoBlocks, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    types::Any, Block, BlockError, BlockResult, BlockRuntime, Message, OutputPort, Port,
    PortResult, SystemBuilding,
};
use protoflow_derive::Block;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;
use std::{
    format,
    io::Read,
    net::{TcpListener, TcpStream},
    string::ToString,
    sync::{Arc, Mutex, PoisonError},
};
use tracing::{error, info};
/// A block that reads a proto object from a TCP port.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/sys/read_socket.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/sys/read_socket.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     // TODO
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute ReadSocket host="127.0.0.1" port="7077" buffer_size="1024"
/// ```
///
#[derive(Block, Clone)]
pub struct ReadSocket<T: Message = Any> {
    #[output]
    pub output: OutputPort<T>,
    #[parameter]
    pub config: ReadSocketConfig,
    #[cfg(feature = "std")]
    #[state]
    pub listener: Arc<Mutex<Option<TcpListener>>>,
    #[cfg(feature = "std")]
    #[state]
    pub stream: Arc<Mutex<Option<TcpStream>>>,
}

impl<T: Message> ReadSocket<T> {
    pub fn with_params(output: OutputPort<T>, config: Option<ReadSocketConfig>) -> Self {
        Self {
            output,
            config: config.unwrap_or(Default::default()),
            listener: Arc::new(Mutex::new(None)),
            stream: Arc::new(Mutex::new(None)),
        }
    }
}

impl<T: Message + 'static> ReadSocket<T> {
    pub fn with_system(system: &System, config: Option<ReadSocketConfig>) -> Self {
        Self::with_params(system.output(), config)
    }
}

impl<T: Message + ToString + 'static> StdioSystem for ReadSocket<T> {
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

impl<T: Message + 'static> Block for ReadSocket<T> {
    fn prepare(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        let address = format!("{}:{}", &self.config.host, &self.config.port);
        let listener = TcpListener::bind(&address)?;
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

fn lock_error<T>(err: PoisonError<T>) -> BlockError {
    BlockError::Other(format!("Failed to acquire lock: {:?}", err))
}

fn handle_client<M, F>(
    stream: &mut TcpStream,
    buffer_size: usize,
    process_fn: F,
) -> Result<(), BlockError>
where
    M: Message + Default,
    F: Fn(&M) -> PortResult<()>,
{
    let mut buffer = vec![0; buffer_size];

    while let Ok(bytes_read) = Read::read(stream, &mut buffer) {
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
            port: 7070,
            buffer_size: 512,
        }
    }
}
#[cfg(test)]
pub mod read_socket_tests {

    use protoflow_core::SystemBuilding;

    use crate::{Encoding, SysBlocks, System};

    use super::ReadSocket;
    use std::string::String;
    extern crate std;

    #[test]
    #[ignore]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(ReadSocket::<String>::with_system(s, None));
        });
    }
    #[test]
    #[ignore = "requires port"]
    fn run_server() {
        use super::*;
        use SystemBuilding;
        if let Err(e) = System::run(|s| {
            let std_out = s.write_stdout();
            let message_encoder = s.encode_with::<String>(Encoding::TextWithNewlineSuffix);

            let read_socket = s.block(ReadSocket::<String>::with_system(
                s,
                Some(ReadSocketConfig {
                    host: String::from("127.0.0.1"),
                    port: 7070,
                    buffer_size: 512,
                }),
            ));
            s.connect(&read_socket.output, &message_encoder.input);
            s.connect(&message_encoder.output, &std_out.input);
        }) {
            error!("{}", e)
        }
    }
}
