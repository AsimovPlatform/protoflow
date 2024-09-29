// This is free and unencumbered software released into the public domain.

pub mod text {
    pub trait TextBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum TextBlocksConfig {}
}

pub use text::*;
