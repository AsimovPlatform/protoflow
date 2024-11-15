// This is free and unencumbered software released into the public domain.

pub mod text {
    use super::{
        prelude::{vec, Box, Cow, Named, Vec, String},
        BlockConnections, BlockInstantiation, InputPortName, OutputPortName, System
    };
    use protoflow_core::Block;

    pub trait TextBlocks {
        fn concat_strings(&mut self) -> ConcatStrings;
        fn concat_strings_by(&mut self, joiner: &str) -> ConcatStrings;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum TextBlockTag {
        ConcatStrings
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum TextBlockConfig {
        ConcatStrings {
            input: InputPortName,
            output: OutputPortName,
            joiner: Option<String>
        }
    }

    impl Named for TextBlockConfig {
        fn name(&self) -> Cow<str> {
            use TextBlockConfig::*;
            Cow::Borrowed(match self {
                ConcatStrings { .. } => "ConcatStrings",
            })
        }
    }

    impl BlockConnections for TextBlockConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use TextBlockConfig::*;
            match self {
                ConcatStrings { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
            }
        }
    }

    impl BlockInstantiation for TextBlockConfig {
        fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
            use TextBlockConfig::*;
            match self {
                ConcatStrings { joiner, .. } => {
                    Box::new(super::ConcatStrings::with_system(system, joiner.clone()))
                }
            }
        }
    }

    mod concat_strings;
    pub use concat_strings::*;
}

pub use text::*;
