// This is free and unencumbered software released into the public domain.

use crate::exit::ExitCode;
use protoflow_blocks::{build_stdio_system, Encoding, StdioConfig, StdioError};
use protoflow_core::SystemExecution;
use std::path::PathBuf;

pub fn execute(
    system_uri: &PathBuf,
    system_params: &Vec<(String, String)>,
    stdio_encoding: Encoding,
) -> Result<(), ExitCode> {
    let system_uri = system_uri.to_string_lossy().to_string();
    let system_config = StdioConfig {
        encoding: stdio_encoding,
        params: system_params.iter().cloned().collect(),
    };
    let system = build_stdio_system(system_uri, system_config)?;
    system.execute().unwrap().join().unwrap(); // TODO: improve error handling
    Ok(())
}

#[derive(Clone, Debug)]
pub enum ExecuteError {
    UnknownSystem(String),
    MissingParameter(&'static str),
    InvalidParameter(&'static str),
    InvalidEncoding(String),
}

impl std::error::Error for ExecuteError {}

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ExecuteError::*;
        match self {
            UnknownSystem(system) => {
                write!(f, "unknown system: {}", system)
            }
            MissingParameter(parameter) => {
                write!(f, "missing parameter: {}", parameter)
            }
            InvalidParameter(parameter) => {
                write!(f, "invalid parameter: {}", parameter)
            }
            InvalidEncoding(encoding) => {
                write!(f, "invalid encoding: {}", encoding)
            }
        }
    }
}

impl From<StdioError> for ExecuteError {
    fn from(error: StdioError) -> Self {
        use StdioError::*;
        match error {
            UnknownSystem(system) => Self::UnknownSystem(system),
            MissingParameter(parameter) => Self::MissingParameter(parameter),
            InvalidParameter(parameter) => Self::InvalidParameter(parameter),
        }
    }
}
