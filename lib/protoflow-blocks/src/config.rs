// This is free and unencumbered software released into the public domain.

use super::prelude::{Cow, Named, String};
use crate::{
    CoreBlocksConfig, FlowBlocksConfig, HashBlocksConfig, IoBlocksConfig, MathBlocksConfig,
    SysBlocksConfig, TextBlocksConfig,
};

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
                tag: _,
                value: Value::Mapping(_mapping),
            } => {
                let result = BlockConfig::deserialize(value.clone());
                Ok(result.unwrap()) // FIXME
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
