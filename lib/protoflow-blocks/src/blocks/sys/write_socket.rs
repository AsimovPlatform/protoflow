extern crate std;

use core::str::FromStr;

use crate::{IoBlocks, StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{Block, BlockError, BlockResult, BlockRuntime, InputPort, SystemBuilding};
use protoflow_derive::Block;
use tracing::error;

use crate::prelude::{vec, String};

use super::SysBlocks;
use serde::{Deserialize, Serialize};
#[derive(Block, Clone)]
pub struct WriteSocket<T: protoflow_core::Message> {
    #[output]
    pub input: InputPort<T>,
    #[parameter]
    pub config: WriteSocketConfig,
    #[state]
    pub stream: std::sync::Arc<std::sync::Mutex<Option<std::net::TcpStream>>>,
}
impl<T: protoflow_core::Message> WriteSocket<T> {
    pub fn with_params(input: InputPort<T>, config: Option<WriteSocketConfig>) -> Self {
        Self {
            input,
            config: config.unwrap_or(Default::default()),
            stream: std::sync::Arc::new(std::sync::Mutex::new(None)),
        }
    }
    pub fn with_config(self, config: WriteSocketConfig) -> Self {
        Self { config, ..self }
    }
}
impl<T: protoflow_core::Message + 'static> WriteSocket<T> {
    pub fn with_system(system: &System, config: Option<WriteSocketConfig>) -> Self {
        Self::with_params(system.input(), config)
    }
}

impl<T: protoflow_core::Message + 'static + FromStr> StdioSystem for WriteSocket<T> {
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
                let address = std::format!("{}:{}", self.config.host, self.config.port);
                *stream_guard = Some(std::net::TcpStream::connect(&address).map_err(|e| {
                    error!("Failed to connect to {}: {}", &address, e);
                    BlockError::Other(std::format!("Failed to connect to {}: {}", &address, e))
                })?);
            }

            let stream = stream_guard.as_mut().ok_or_else(|| {
                error!("Stream is not connected");
                BlockError::Other("Stream is not connected".into())
            })?;

            let mut response = vec::Vec::new();
            input
                .encode(&mut response)
                .map_err(|e| BlockError::Other(std::format!("Encoding failed: {}", e)))?;

            std::io::Write::write_all(stream, &response)?;
        }
        Ok(())
    }
}
fn lock_error<T>(err: std::sync::PoisonError<T>) -> BlockError {
    BlockError::Other(std::format!("Failed to acquire lock: {:?}", err))
}

#[cfg(test)]
pub mod tests {

    use protoflow_core::SystemBuilding;

    use crate::System;

    use super::WriteSocket;

    extern crate std;

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(WriteSocket::<std::string::String>::with_system(s, None));
        });
    }
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
            port: 7777,
            buffer_size: 512,
        }
    }
}
