// This is free and unencumbered software released into the public domain.

use super::prelude::{Box, Cow, Named, String, Vec};
use crate::{
    BlockConnections, BlockInstantiation, CoreBlocksConfig, FlowBlocksConfig, HashBlocksConfig,
    IoBlocksConfig, MathBlocksConfig, SysBlocksConfig, System, TextBlocksConfig,
};
use protoflow_core::Block;

pub type InputPortName = String;
pub type OutputPortName = String;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Clone, Debug)]
pub enum BlockConfig {
    Core(CoreBlocksConfig),
    Flow(FlowBlocksConfig),
    #[cfg(feature = "hash")]
    Hash(HashBlocksConfig),
    Io(IoBlocksConfig),
    Math(MathBlocksConfig),
    #[cfg(feature = "std")]
    Sys(SysBlocksConfig),
    Text(TextBlocksConfig),
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BlockConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_yml::{value::TaggedValue, Value};
        let value = TaggedValue::deserialize(deserializer)?;
        match &value {
            TaggedValue {
                tag,
                value: Value::Mapping(_mapping),
            } => {
                Ok(match tag.string.as_str() {
                    "Buffer" | "Const" | "Count" | "Delay" | "Drop" | "Random" => {
                        CoreBlocksConfig::deserialize(value.clone())
                            .map(BlockConfig::Core)
                            .unwrap()
                    }

                    #[cfg(feature = "hash")]
                    "Hash" => HashBlocksConfig::deserialize(value.clone())
                        .map(BlockConfig::Hash)
                        .unwrap(),

                    "Decode" | "Encode" | "EncodeHex" => IoBlocksConfig::deserialize(value.clone())
                        .map(BlockConfig::Io)
                        .unwrap(),

                    #[cfg(feature = "std")]
                    "ReadDir" | "ReadEnv" | "ReadFile" | "ReadStdin" | "WriteFile"
                    | "WriteStderr" | "WriteStdout" => SysBlocksConfig::deserialize(value.clone())
                        .map(BlockConfig::Sys)
                        .unwrap(),

                    _ => unimplemented!(), // TODO
                })
            }
            _ => unimplemented!(), // TODO
        }
    }
}

impl Named for BlockConfig {
    fn name(&self) -> Cow<str> {
        use BlockConfig::*;
        match self {
            Core(config) => config.name(),
            Flow(config) => config.name(),
            #[cfg(feature = "hash")]
            Hash(config) => config.name(),
            Io(config) => config.name(),
            Math(config) => config.name(),
            #[cfg(feature = "std")]
            Sys(config) => config.name(),
            Text(config) => config.name(),
        }
    }
}

impl BlockConnections for BlockConfig {
    fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
        use BlockConfig::*;
        match self {
            Core(config) => config.output_connections(),
            Flow(config) => config.output_connections(),
            #[cfg(feature = "hash")]
            Hash(config) => config.output_connections(),
            Io(config) => config.output_connections(),
            Math(config) => config.output_connections(),
            #[cfg(feature = "std")]
            Sys(config) => config.output_connections(),
            Text(config) => config.output_connections(),
        }
    }
}

impl BlockInstantiation for BlockConfig {
    fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
        use BlockConfig::*;
        match self {
            Core(config) => config.instantiate(system),
            Flow(config) => config.instantiate(system),
            #[cfg(feature = "hash")]
            Hash(config) => config.instantiate(system),
            Io(config) => config.instantiate(system),
            Math(config) => config.instantiate(system),
            #[cfg(feature = "std")]
            Sys(config) => config.instantiate(system),
            Text(config) => config.instantiate(system),
        }
    }
}
