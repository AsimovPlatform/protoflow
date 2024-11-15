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
        fn split_string(&mut self, delimiter: &str) -> SplitString;
        fn split_string_whitespace(&mut self) -> SplitString;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum TextBlockTag {
        ConcatStrings,
        SplitString,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum TextBlockConfig {
        ConcatStrings {
            input: InputPortName,
            output: OutputPortName,
            joiner: Option<String>
        },
        SplitString {
            input: InputPortName,
            output: OutputPortName,
            delimiter: Option<String>
        }
    }

    impl Named for TextBlockConfig {
        fn name(&self) -> Cow<str> {
            use TextBlockConfig::*;
            Cow::Borrowed(match self {
                ConcatStrings { .. } => "ConcatStrings",
                SplitString { .. } => "SplitString",
            })
        }
    }

    impl BlockConnections for TextBlockConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use TextBlockConfig::*;
            match self {
                ConcatStrings { output, .. }
                | SplitString { output, .. } => {
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
                SplitString { delimiter, .. } => {
                    Box::new(super::SplitString::with_system(system, delimiter.clone()))
                }
            }
        }
    }

    mod concat_strings;
    pub use concat_strings::*;

    mod split_string;
    pub use split_string::*;
}

pub use text::*;
