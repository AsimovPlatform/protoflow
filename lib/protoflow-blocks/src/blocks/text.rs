// This is free and unencumbered software released into the public domain.

pub mod text {
    use super::prelude::{Cow, Named};

    pub trait TextBlocks {}

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum TextBlocksConfig {}

    impl Named for TextBlocksConfig {
        fn name(&self) -> Cow<str> {
            unreachable!()
        }
    }
}

pub use text::*;
