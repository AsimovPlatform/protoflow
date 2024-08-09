// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "blocks")]
    "blocks",
    #[cfg(feature = "crossbeam")]
    "crossbeam",
    #[cfg(feature = "derive")]
    "derive",
    #[cfg(feature = "flume")]
    "flume",
    #[cfg(feature = "rand")]
    "rand",
    #[cfg(feature = "syntax")]
    "syntax",
    #[cfg(feature = "sysml")]
    "sysml",
    #[cfg(feature = "tracing")]
    "tracing",
    #[cfg(feature = "web")]
    "web",
    #[cfg(feature = "zeromq")]
    "zeromq",
];
