// This is free and unencumbered software released into the public domain.

pub mod flow {
    use super::{
        prelude::{Cow, Named},
        BlockConfigConnections, BlockConfigInstantiation,
    };

    pub trait FlowBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum FlowBlockTag {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum FlowBlocksConfig {}

    impl Named for FlowBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }

    impl BlockConfigConnections for FlowBlocksConfig {}

    impl BlockConfigInstantiation for FlowBlocksConfig {}
}

pub use flow::*;
