// This is free and unencumbered software released into the public domain.

pub mod flow {
    use crate::{InputPortName, OutputPortName};

    use super::{
        prelude::{vec, Box, Cow, Named, Vec},
        BlockConnections, BlockInstantiation, System,
    };

    use protoflow_core::{Block, Message};

    pub trait FlowBlocks {
        fn split<T: Message + Into<T> + 'static>(&mut self) -> Split<T>;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum FlowBlockTag {
        Split,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum FlowBlockConfig {
        Split {
            input: InputPortName,
            output_1: OutputPortName,
            output_2: OutputPortName,
        },
    }

    impl Named for FlowBlockConfig {
        fn name(&self) -> Cow<str> {
            use FlowBlockConfig::*;
            Cow::Borrowed(match self {
                Split { .. } => "Split",
            })
        }
    }

    impl BlockConnections for FlowBlockConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use FlowBlockConfig::*;
            match self {
                Split {
                    output_1, output_2, ..
                } => {
                    vec![
                        ("output_1", Some(output_1.clone())),
                        ("output_2", Some(output_2.clone())),
                    ]
                }
            }
        }
    }

    impl BlockInstantiation for FlowBlockConfig {
        fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
            use super::SystemBuilding;
            use FlowBlockConfig::*;
            match self {
                Split { .. } => Box::new(super::Split::new(
                    system.input_any(),
                    system.output(),
                    system.output(),
                )),
            }
        }
    }

    mod split;
    pub use split::*;
}

pub use flow::*;
