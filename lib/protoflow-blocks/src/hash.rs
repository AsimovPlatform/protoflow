// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "hash"))]
pub mod hash {
    pub trait HashBlocks {}
    pub enum HashBlocksConfig {}
}

#[cfg(feature = "hash")]
pub mod hash {
    use super::{InputPortName, OutputPortName};

    pub trait HashBlocks {
        fn hash_blake3(&mut self) -> Hash;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum HashBlocksConfig {
        Hash {
            input: InputPortName,
            output: Option<OutputPortName>,
            hash: OutputPortName,
            algorithm: HashAlgorithm,
        },
    }

    mod hash;
    pub use hash::*;
}

pub use hash::*;
