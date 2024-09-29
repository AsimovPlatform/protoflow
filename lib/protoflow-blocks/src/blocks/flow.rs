// This is free and unencumbered software released into the public domain.

pub mod flow {
    pub trait FlowBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum FlowBlocksConfig {}
}

pub use flow::*;
