// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "flume")]
    "flume",
    #[cfg(feature = "rand")]
    "rand",
    #[cfg(feature = "tracing")]
    "tracing",
    #[cfg(feature = "zeromq")]
    "zeromq",
];
