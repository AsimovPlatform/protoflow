// This is free and unencumbered software released into the public domain.

pub mod flow {
    use super::prelude::{Cow, Named};

    pub trait FlowBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum FlowBlocksConfig {}

    impl Named for FlowBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }
}

pub use flow::*;
