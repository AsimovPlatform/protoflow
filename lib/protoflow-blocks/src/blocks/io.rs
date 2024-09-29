// This is free and unencumbered software released into the public domain.

pub mod io {
    use super::{
        prelude::{Cow, Named},
        InputPortName, OutputPortName,
    };
    use crate::{
        prelude::{FromStr, ToString},
        Encoding,
    };
    use protoflow_core::Message;

    pub trait IoBlocks {
        fn decode<T: Message + FromStr + 'static>(&mut self) -> Decode<T>;
        fn decode_with<T: Message + FromStr + 'static>(&mut self, encoding: Encoding) -> Decode<T>;

        fn decode_lines<T: Message + FromStr + 'static>(&mut self) -> Decode<T> {
            self.decode_with::<T>(Encoding::TextWithNewlineSuffix)
        }

        fn encode<T: Message + ToString + 'static>(&mut self) -> Encode<T>;
        fn encode_with<T: Message + ToString + 'static>(&mut self, encoding: Encoding)
            -> Encode<T>;

        fn encode_lines<T: Message + ToString + 'static>(&mut self) -> Encode<T> {
            self.encode_with::<T>(Encoding::TextWithNewlineSuffix)
        }

        fn encode_hex(&mut self) -> EncodeHex;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum IoBlocksConfig {
        Decode {
            input: InputPortName,
            output: OutputPortName,
            encoding: Encoding,
        },

        Encode {
            input: InputPortName,
            output: OutputPortName,
            encoding: Encoding,
        },

        EncodeHex {
            input: InputPortName,
            output: OutputPortName,
        },
    }

    impl Named for IoBlocksConfig {
        fn name(&self) -> Cow<str> {
            use IoBlocksConfig::*;
            Cow::Borrowed(match self {
                Decode { .. } => "Decode",
                Encode { .. } => "Encode",
                EncodeHex { .. } => "EncodeHex",
            })
        }
    }

    mod decode;
    pub use decode::*;

    mod encode;
    pub use encode::*;

    mod encode_hex;
    pub use encode_hex::*;
}

pub use io::*;
