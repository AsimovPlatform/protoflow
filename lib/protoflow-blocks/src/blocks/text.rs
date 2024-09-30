// This is free and unencumbered software released into the public domain.

pub mod text {
    use super::{
        prelude::{Cow, Named},
        BlockConfigConnections, BlockConfigInstantiation,
    };

    pub trait TextBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum TextBlockTag {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum TextBlocksConfig {}

    impl Named for TextBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }

    impl BlockConfigConnections for TextBlocksConfig {}

    impl BlockConfigInstantiation for TextBlocksConfig {}
}

pub use text::*;
