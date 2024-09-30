// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "hash"))]
pub mod hash {
    pub trait HashBlocks {}
    pub enum HashBlocksConfig {}
}

#[cfg(feature = "hash")]
pub mod hash {
    use super::{
        prelude::{vec, Box, Cow, Named, Vec},
        types::HashAlgorithm,
        BlockConnections, BlockInstantiation, InputPortName, OutputPortName, System,
    };
    use protoflow_core::Block;

    pub trait HashBlocks {
        fn hash_blake3(&mut self) -> Hash;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum HashBlockTag {
        Hash,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum HashBlocksConfig {
        Hash {
            input: InputPortName,
            output: Option<OutputPortName>,
            hash: OutputPortName,
            algorithm: Option<HashAlgorithm>,
        },
    }

    impl Named for HashBlocksConfig {
        fn name(&self) -> Cow<str> {
            use HashBlocksConfig::*;
            Cow::Borrowed(match self {
                Hash { .. } => "Hash",
            })
        }
    }

    impl BlockConnections for HashBlocksConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use HashBlocksConfig::*;
            match self {
                Hash { output, hash, .. } => {
                    vec![("output", output.clone()), ("hash", Some(hash.clone()))]
                }
            }
        }
    }

    impl BlockInstantiation for HashBlocksConfig {
        fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
            use HashBlocksConfig::*;
            match self {
                Hash { algorithm, .. } => Box::new(super::Hash::with_system(system, *algorithm)),
            }
        }
    }

    mod hash;
    pub use hash::*;
}

pub use hash::*;
