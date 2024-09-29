// This is free and unencumbered software released into the public domain.

pub mod math {
    use super::{
        prelude::{Cow, Named, Vec},
        BlockConfigConnections, OutputPortName,
    };

    pub trait MathBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum MathBlocksConfig {}

    impl Named for MathBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }

    impl BlockConfigConnections for MathBlocksConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            unreachable!()
        }
    }
}

pub use math::*;
