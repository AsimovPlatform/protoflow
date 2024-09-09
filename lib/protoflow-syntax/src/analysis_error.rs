// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

use crate::prelude::String;
use displaydoc::Display;
use error_stack::Result;
use sysml_model::QualifiedName;

pub type AnalysisResult<T> = Result<T, AnalysisError>;

#[derive(Debug, Display)]
pub enum AnalysisError {
    /// Parse failure
    ParseFailure,
    /// Invalid import: `{0}`.
    InvalidImport(QualifiedName),
    /// Unknown name: `{0}`.
    UnknownName(QualifiedName),
    /// Other error: `{0}`.
    Other(String),
}

#[cfg(feature = "std")]
impl std::error::Error for AnalysisError {}

#[cfg(not(feature = "std"))]
impl error_stack::Context for AnalysisError {}
