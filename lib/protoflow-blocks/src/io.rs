// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{FromStr, ToString},
    Encoding,
};
use protoflow_core::Message;

pub trait IoBlocks {
    fn decode<T: Message + FromStr + 'static>(&self) -> Decode<T>;
    fn decode_with<T: Message + FromStr + 'static>(&self, encoding: Encoding) -> Decode<T>;

    fn decode_lines<T: Message + FromStr + 'static>(&self) -> Decode<T> {
        self.decode_with::<T>(Encoding::TextWithNewlineSuffix)
    }

    fn encode<T: Message + ToString + 'static>(&self) -> Encode<T>;
    fn encode_with<T: Message + ToString + 'static>(&self, encoding: Encoding) -> Encode<T>;

    fn encode_lines<T: Message + ToString + 'static>(&self) -> Encode<T> {
        self.encode_with::<T>(Encoding::TextWithNewlineSuffix)
    }

    fn encode_hex(&self) -> EncodeHex;
}

mod decode;
pub use decode::*;

mod encode;
pub use encode::*;

mod encode_hex;
pub use encode_hex::*;
