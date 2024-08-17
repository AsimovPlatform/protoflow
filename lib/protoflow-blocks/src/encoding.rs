// This is free and unencumbered software released into the public domain.

/// The encoding to use when (de)serializing messages from/to bytes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Encoding {
    #[default]
    ProtobufWithLengthPrefix,
    ProtobufWithoutLengthPrefix,
    TextWithNewlineSuffix,
}
