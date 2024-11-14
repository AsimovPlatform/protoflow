extern crate std;
use super::SysBlocks;
use crate::prelude::{vec, String};
use crate::{IoBlocks, StdioConfig, StdioError, StdioSystem, System};
use core::str::FromStr;
use protoflow_core::{
    Block, BlockError, BlockResult, BlockRuntime, InputPort, Message, SystemBuilding,
};
use protoflow_derive::Block;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;
use std::{
    format,
    net::TcpStream,
    sync::{Arc, Mutex, PoisonError},
};
use tracing::error;
/// A block that writes a proto object to a TCP port.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/sys/write_socket.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/sys/write_socket.seq.mmd" framed)]
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
/// $ protoflow execute WriteSocket host="127.0.0.1" port="7077" buffer_size="1024"
/// ```
///
#[derive(Block, Clone)]
pub struct WriteSocket<T: Message> {
    #[output]
    pub input: InputPort<T>,
    #[parameter]
    pub config: WriteSocketConfig,
    #[state]
    pub stream: Arc<Mutex<Option<TcpStream>>>,
}

impl<T: Message> WriteSocket<T> {
    pub fn with_params(input: InputPort<T>, config: Option<WriteSocketConfig>) -> Self {
        Self {
            input,
            config: config.unwrap_or(Default::default()),
            stream: Arc::new(Mutex::new(None)),
        }
    }
    pub fn with_config(self, config: WriteSocketConfig) -> Self {
        Self { config, ..self }
    }
}

impl<T: Message + 'static> WriteSocket<T> {
    pub fn with_system(system: &System, config: Option<WriteSocketConfig>) -> Self {
        Self::with_params(system.input(), config)
    }
}

impl<T: Message + 'static + FromStr> StdioSystem for WriteSocket<T> {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        config.allow_only(vec!["host", "port", "buffer_size"])?;

        let host = config.get_string("host")?;
        let port: u16 = config.get("port")?;
        let buffer_size: usize = config.get("buffer_size")?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let message_decoder = s.decode_with::<T>(config.encoding);
            let write_socket: WriteSocket<T> = s.write_socket().with_config(WriteSocketConfig {
                host,
                port,
                buffer_size,
            });

            s.connect(&stdin.output, &message_decoder.input);
            s.connect(&message_decoder.output, &write_socket.input);
        }))
    }
}

impl<T: protoflow_core::Message> Block for WriteSocket<T> {
    fn execute(&mut self, _: &dyn BlockRuntime) -> BlockResult {
        while let Some(input) = self.input.recv()? {
            let mut stream_guard = self.stream.lock().map_err(lock_error)?;

            if stream_guard.is_none() {
                let address = format!("{}:{}", self.config.host, self.config.port);
                *stream_guard = Some(TcpStream::connect(&address).map_err(|e| {
                    error!("Failed to connect to {}: {}", &address, e);
                    BlockError::Other(format!("Failed to connect to {}: {}", &address, e))
                })?);
            }

            let stream = stream_guard.as_mut().ok_or_else(|| {
                error!("Stream is not connected");
                BlockError::Other("Stream is not connected".into())
            })?;

            let mut response = vec::Vec::new();
            input
                .encode(&mut response)
                .map_err(|e| BlockError::Other(format!("Encoding failed: {}", e)))?;

            std::io::Write::write_all(stream, &response)?;
        }
        Ok(())
    }
}

fn lock_error<T>(err: PoisonError<T>) -> BlockError {
    BlockError::Other(format!("Failed to acquire lock: {:?}", err))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteSocketConfig {
    pub host: String,
    pub port: u16,
    pub buffer_size: usize,
}

impl<'a> Default for WriteSocketConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 7070,
            buffer_size: 512,
        }
    }
}

#[cfg(test)]
pub mod write_socket_tests {
    use protoflow_core::SystemBuilding;

    use crate::{Encoding, System};

    use super::WriteSocket;

    extern crate std;
    use std::string::String;

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(WriteSocket::<String>::with_system(s, None));
        });
    }
    #[test]
    #[ignore = "requires port"]
    fn run_client() {
        use super::*;
        use protoflow_core::SystemBuilding;
        if let Err(e) = System::run(|s| {
            let stdin = s.read_stdin();
            let message_decoder = s.decode_with::<String>(Encoding::TextWithNewlineSuffix);

            let write_socket = s.write_socket().with_config(WriteSocketConfig {
                host: String::from("127.0.0.1"),
                port: 7070,
                buffer_size: 512,
            });
            s.connect(&stdin.output, &message_decoder.input);
            s.connect(&message_decoder.output, &write_socket.input);
        }) {
            error!("{}", e)
        }
    }
}
