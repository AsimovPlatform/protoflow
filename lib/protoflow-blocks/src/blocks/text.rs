// This is free and unencumbered software released into the public domain.

pub mod text {
    use super::{
        prelude::{Cow, Named, Vec},
        BlockConfigConnections, OutputPortName,
    };

    pub trait TextBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum TextBlocksConfig {}

    impl Named for TextBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }

    impl BlockConfigConnections for TextBlocksConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            unreachable!()
        }
    }
}

pub use text::*;
