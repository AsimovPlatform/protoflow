// This is free and unencumbered software released into the public domain.

pub mod flow {
    use crate::{InputPortName, OutputPortName};

    use super::{
        prelude::{vec, Box, Cow, Named, Vec},
        BlockConnections, BlockInstantiation, System,
    };

    use protoflow_core::{Block, Message};

    pub trait FlowBlocks {
        fn gate<T: Message + 'static>(&mut self) -> Gate<T>;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum FlowBlockTag {
        Gate,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum FlowBlockConfig {
        Gate {
            input: InputPortName,
            trigger: InputPortName,
            output: OutputPortName,
        },
    }

    impl Named for FlowBlockConfig {
        fn name(&self) -> Cow<str> {
            use FlowBlockConfig::*;
            Cow::Borrowed(match self {
                Gate { .. } => "Gate",
            })
        }
    }

    impl BlockConnections for FlowBlockConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use FlowBlockConfig::*;
            match self {
                Gate { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
            }
        }
    }

    impl BlockInstantiation for FlowBlockConfig {
        fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
            use super::SystemBuilding;
            use FlowBlockConfig::*;
            match self {
                Gate { .. } => Box::new(super::Gate::<_, ()>::new(
                    system.input_any(),
                    system.input(),
                    system.output_any(),
                )),
            }
        }
    }

    mod gate;
    pub use gate::*;
}

pub use flow::*;
