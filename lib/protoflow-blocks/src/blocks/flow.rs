// This is free and unencumbered software released into the public domain.

pub mod flow {
    use crate::{InputPortName, OutputPortName};

    use super::{
        prelude::{vec, Box, Cow, Named, Vec},
        BlockConnections, BlockInstantiation, System,
    };

    use protoflow_core::{Block, ComparableAny, Message};

    pub trait FlowBlocks {
        fn batch<T: Message + Into<T> + 'static>(&mut self, batch_size: usize) -> Batch<T>;
        fn concat<T: Message + Into<T> + 'static>(&mut self) -> Concat<T>;
        fn distinct<T: Message + Into<T> + PartialEq + 'static>(&mut self) -> Distinct<T>;
        fn merge<T: Message + Into<T> + 'static>(&mut self) -> Merge<T>;
        fn replicate<T: Message + Into<T> + 'static>(&mut self) -> Replicate<T>;
        fn sort<T: Message + Into<T> + PartialOrd + 'static>(&mut self) -> Sort<T>;
        fn split<T: Message + Into<T> + 'static>(&mut self) -> Split<T>;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum FlowBlockTag {
        Batch,
        Concat,
        Distinct,
        Merge,
        Replicate,
        Sort,
        Split,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum FlowBlockConfig {
        Batch {
            input: InputPortName,
            output: OutputPortName,
        },
        Concat {
            input_1: InputPortName,
            input_2: InputPortName,
            output: OutputPortName,
        },
        Distinct {
            input: InputPortName,
            output: OutputPortName,
        },
        Merge {
            input_1: InputPortName,
            input_2: InputPortName,
            output: OutputPortName,
        },
        Replicate {
            input: InputPortName,
            output_1: OutputPortName,
            output_2: OutputPortName,
        },
        Sort {
            input: InputPortName,
            output: OutputPortName,
        },
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
                Batch { .. } => "Batch",
                Concat { .. } => "Concat",
                Distinct { .. } => "Distinct",
                Merge { .. } => "Merge",
                Replicate { .. } => "Replicate",
                Sort { .. } => "Sort",
                Split { .. } => "Split",
            })
        }
    }

    impl BlockConnections for FlowBlockConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use FlowBlockConfig::*;
            match self {
                Batch { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
                Concat { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
                Distinct { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
                Merge { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
                Replicate {
                    output_1, output_2, ..
                } => {
                    vec![
                        ("output_1", Some(output_1.clone())),
                        ("output_2", Some(output_2.clone())),
                    ]
                }
                Sort { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
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
                Batch { .. } => Box::new(super::Batch::new(system.input_any(), system.output())),
                Concat { .. } => Box::new(super::Concat::new(
                    system.input_any(),
                    system.input_any(),
                    system.output(),
                )),
                Distinct { .. } => {
                    Box::new(super::Distinct::new(system.input_any(), system.output()))
                }
                Merge { .. } => Box::new(super::Merge::new(
                    system.input_any(),
                    system.input_any(),
                    system.output(),
                )),
                Replicate { .. } => Box::new(super::Replicate::new(
                    system.input_any(),
                    system.output(),
                    system.output(),
                )),
                Sort { .. } => Box::new(super::Sort::new(
                    system.input::<ComparableAny>(),
                    system.output(),
                )),
                Split { .. } => Box::new(super::Split::new(
                    system.input_any(),
                    system.output(),
                    system.output(),
                )),
            }
        }
    }

    mod batch;
    pub use batch::*;

    mod concat;
    pub use concat::*;

    mod distinct;
    pub use distinct::*;

    mod merge;
    pub use merge::*;

    mod replicate;
    pub use replicate::*;

    mod sort;
    pub use sort::*;

    mod split;
    pub use split::*;
}

pub use flow::*;
