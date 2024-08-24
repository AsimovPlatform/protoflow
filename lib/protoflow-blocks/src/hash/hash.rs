// This is free and unencumbered software released into the public domain.

use crate::{prelude::Bytes, StdioConfig, StdioError, StdioSystem, System};
use blake3::Hasher;
use protoflow_core::{Block, BlockResult, BlockRuntime, InputPort, OutputPort, Port, PortError};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// Computes the cryptographic hash of a byte stream, while optionally
/// passing it through.
///
/// # Block Diagram
#[doc = mermaid!("../../doc/hash/hash.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../doc/hash/hash.seq.mmd" framed)]
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

    /// A configuration parameter for which algorithm to use.
    #[parameter]
    pub algorithm: HashAlgorithm,

    /// The internal state for computing the hash.
    #[state]
    hasher: Hasher,
}

/// The cryptographic hash algorithm to use.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HashAlgorithm {
    #[default]
    BLAKE3,
}

impl Hash {
    pub fn new(
        input: InputPort<Bytes>,
        output: OutputPort<Bytes>,
        hash: OutputPort<Bytes>,
    ) -> Self {
        Self::with_params(input, output, hash, HashAlgorithm::default())
    }

    pub fn with_params(
        input: InputPort<Bytes>,
        output: OutputPort<Bytes>,
        hash: OutputPort<Bytes>,
        algorithm: HashAlgorithm,
    ) -> Self {
        Self {
            input,
            output,
            hash,
            algorithm,
            hasher: Hasher::new(),
        }
    }
}

impl Block for Hash {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            self.hasher.update(&message);

            if self.output.is_connected() {
                self.output.send(&message)?;
            } else {
                drop(message);
            }
        }
        self.output.close()?;

        runtime.wait_for(&self.hash)?;

        let hash = Bytes::from(self.hasher.finalize().as_bytes().to_vec());
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
    fn build_system(_config: StdioConfig) -> Result<System, StdioError> {
        use crate::{HashBlocks, IoBlocks, SysBlocks, SystemBuilding};

        // TODO: parse the algorithm parameter

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let hasher = s.hash_blake3();
            let hex_encoder = s.encode_hex();
            let stdout = s.write_stdout();
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
