// This is free and unencumbered software released into the public domain.

use crate::prelude::{FromStr, ToString};
use protoflow_core::Message;

pub trait IoBlocks {
    fn decode<T: Message + FromStr + 'static>(&self) -> Decode<T>;
    fn decode_with<T: Message + FromStr + 'static>(&self, encoding: ReadEncoding) -> Decode<T>;
    fn encode<T: Message + ToString + 'static>(&self) -> Encode<T>;
    fn encode_with<T: Message + ToString + 'static>(&self, encoding: WriteEncoding) -> Encode<T>;
}

mod decode;
pub use decode::*;

mod encode;
pub use encode::*;
