// This is free and unencumbered software released into the public domain.

pub mod math {
    use super::{
        prelude::{Cow, Named},
        BlockConnections, BlockInstantiation,
    };

    pub trait MathBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum MathBlockTag {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum MathBlocksConfig {}

    impl Named for MathBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }

    impl BlockConnections for MathBlocksConfig {}

    impl BlockInstantiation for MathBlocksConfig {}
}

pub use math::*;
