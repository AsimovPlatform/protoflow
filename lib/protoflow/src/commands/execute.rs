// This is free and unencumbered software released into the public domain.

use crate::sysexits::Sysexits;
use protoflow_blocks::{Encoding, StdioError, StdioSystem};
use protoflow_core::SystemExecution;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ExecuteError {
    InvalidEncoding(String),
    MissingParameter(&'static str),
    InvalidParameter(&'static str),
    UnknownSystem(String),
}

impl std::error::Error for ExecuteError {}

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExecuteError::InvalidEncoding(encoding) => {
                write!(f, "invalid encoding: {}", encoding)
            }
            ExecuteError::MissingParameter(parameter) => {
                write!(f, "missing parameter: {}", parameter)
            }
            ExecuteError::InvalidParameter(parameter) => {
                write!(f, "invalid parameter: {}", parameter)
            }
            ExecuteError::UnknownSystem(system) => {
                write!(f, "unknown system: {}", system)
            }
        }
    }
}

impl From<StdioError> for ExecuteError {
    fn from(error: StdioError) -> Self {
        use StdioError::*;
        match error {
            MissingParameter(parameter) => Self::MissingParameter(parameter),
            InvalidParameter(parameter) => Self::InvalidParameter(parameter),
        }
    }
}

pub fn execute(
    block: &PathBuf,
    encoding: Encoding,
    params: &Vec<(String, String)>,
) -> Result<(), Sysexits> {
    use protoflow_blocks::{Buffer, Const, Count, Delay, Drop, Random};
    #[cfg(feature = "std")]
    use protoflow_blocks::{
        ReadDir, ReadEnv, ReadFile, ReadStdin, StdioConfig, WriteFile, WriteStderr, WriteStdout,
    };
    let path_or_uri = block.to_string_lossy().to_string();
    let config = StdioConfig {
        encoding,
        params: params.iter().cloned().collect(),
    };
    // TODO: factor this out from here:
    let system = match path_or_uri.as_ref() {
        // core
        "Buffer" => Buffer::<String>::build_system(config)?,
        "Const" => Const::<String>::build_system(config)?,
        "Count" => Count::<String>::build_system(config)?,
        "Delay" => Delay::<String>::build_system(config)?,
        "Drop" => Drop::<String>::build_system(config)?,
        "Random" => Random::<u64>::build_system(config)?,
        // sys
        #[cfg(feature = "std")]
        "ReadDir" => ReadDir::build_system(config)?,
        #[cfg(feature = "std")]
        "ReadEnv" => ReadEnv::<String>::build_system(config)?,
        #[cfg(feature = "std")]
        "ReadFile" => ReadFile::build_system(config)?,
        #[cfg(feature = "std")]
        "ReadStdin" => ReadStdin::build_system(config)?,
        #[cfg(feature = "std")]
        "WriteFile" => WriteFile::build_system(config)?,
        #[cfg(feature = "std")]
        "WriteStderr" => WriteStderr::build_system(config)?,
        #[cfg(feature = "std")]
        "WriteStdout" => WriteStdout::build_system(config)?,
        _ => return Err(ExecuteError::UnknownSystem(path_or_uri.to_string()))?,
    };
    system.execute().unwrap().join().unwrap(); // TODO
    Ok(())
}
