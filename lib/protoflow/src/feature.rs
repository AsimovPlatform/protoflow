// This is free and unencumbered software released into the public domain.

#[allow(unused)]
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
