// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{vec, Bytes, BytesMut},
    types::HashAlgorithm,
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, OutputPort, Port, PortError};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// Computes the cryptographic hash of a byte stream, while optionally
/// passing it through.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/hash/hash.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/hash/hash.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let hasher = s.hash_blake3();
///     let hex_encoder = s.encode_hex();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &hasher.input);
///     s.connect(&hasher.hash, &hex_encoder.input);
///     s.connect(&hex_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Hash algorithm=blake3
/// ```
///
#[derive(Block, Clone)]
pub struct Hash {
    /// The input byte stream.
    #[input]
    pub input: InputPort<Bytes>,

    /// The (optional) output target for the stream being passed through.
    #[output]
    pub output: OutputPort<Bytes>,

    /// The output port for the computed hash.
    #[output]
    pub hash: OutputPort<Bytes>,

    /// A configuration parameter for which algorithm to use.[blake3, sha256, sha1, md5]
    #[parameter]
    pub algorithm: HashAlgorithm,

    /// The internal state for computing the hash.
    #[state]
    buffer: BytesMut, // Buffer,
}

impl Hash {
    pub fn new(
        input: InputPort<Bytes>,
        output: OutputPort<Bytes>,
        hash: OutputPort<Bytes>,
    ) -> Self {
        Self::with_params(input, output, hash, None)
    }

    pub fn with_params(
        input: InputPort<Bytes>,
        output: OutputPort<Bytes>,
        hash: OutputPort<Bytes>,
        algorithm: Option<HashAlgorithm>,
    ) -> Self {
        Self {
            input,
            output,
            hash,
            algorithm: algorithm.unwrap_or_default(),
            buffer: BytesMut::new(),
        }
    }

    pub fn with_system(system: &System, algorithm: Option<HashAlgorithm>) -> Self {
        use crate::SystemBuilding;
        Self::with_params(system.input(), system.output(), system.output(), algorithm)
    }
}

impl Block for Hash {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.buffer.extend_from_slice(&message);

            if self.output.is_connected() {
                self.output.send(&message)?;
            } else {
                drop(message);
            }
        }
        self.output.close()?;

        runtime.wait_for(&self.hash)?;

        let hash = Bytes::from(self.algorithm.compute_hash(&self.buffer));
        self.buffer.clear();
        match self.hash.send(&hash) {
            Ok(()) => {}
            Err(PortError::Closed | PortError::Disconnected) => {
                // TODO: log the error
            }
            Err(e) => return Err(e)?,
        };

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for Hash {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{IoBlocks, SystemBuilding};

        config.allow_only(vec!["algorithm"])?;

        Ok(System::build(|s| {
            let stdin = config.read_stdin(s);
            let hash_algorithm = config.get::<HashAlgorithm>("algorithm").unwrap_or_default();
            let hasher = Hash::with_system(&s, Some(hash_algorithm));
            let hex_encoder = s.encode_hex();
            let stdout = config.write_stdout(s);
            s.connect(&stdin.output, &hasher.input);
            s.connect(&hasher.hash, &hex_encoder.input);
            s.connect(&hex_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Hash;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Hash::new(s.input(), s.output(), s.output()));
        });
    }
}
