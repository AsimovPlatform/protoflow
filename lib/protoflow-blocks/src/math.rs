// This is free and unencumbered software released into the public domain.

pub mod math {
    pub trait MathBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum MathBlocksConfig {}
}

pub use math::*;
