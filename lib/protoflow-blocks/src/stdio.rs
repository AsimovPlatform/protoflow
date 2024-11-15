// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{types::Encoding, ReadStdin, SysBlocks, System, WriteStderr, WriteStdout};
use protoflow_core::prelude::{BTreeMap, FromStr, String, Vec};

pub trait StdioSystem {
    fn build_system(config: StdioConfig) -> Result<System, StdioError>;
}

#[derive(Debug, Default, Clone)]
pub struct StdioConfig {
    pub encoding: Encoding,
    pub params: BTreeMap<String, String>,
}

impl StdioConfig {
    pub fn reject_any(&self) -> Result<(), StdioError> {
        if !self.params.is_empty() {
            return Err(StdioError::UnknownParameter(
                self.params.keys().next().unwrap().clone(),
            ));
        }
        Ok(())
    }

    pub fn allow_only(&self, keys: Vec<&'static str>) -> Result<(), StdioError> {
        for key in self.params.keys() {
            if !keys.contains(&key.as_str()) {
                return Err(StdioError::UnknownParameter(key.clone()));
            }
        }
        Ok(())
    }

    pub fn get<T: FromStr>(&self, key: &'static str) -> Result<T, StdioError> {
        self.get_string(key)?
            .parse::<T>()
            .map_err(|_| StdioError::InvalidParameter(key))
    }

    pub fn get_opt<T: FromStr>(&self, key: &'static str) -> Result<Option<T>, StdioError> {
        match self.params.get(key) {
            Some(value) => value
                .parse::<T>()
                .map_err(|_| StdioError::InvalidParameter(key))
                .map(Some),
            None => Ok(None),
        }
    }

    pub fn get_string(&self, key: &'static str) -> Result<String, StdioError> {
        let Some(value) = self.params.get(key).map(String::clone) else {
            return Err(StdioError::MissingParameter(key))?;
        };
        Ok(value)
    }

    pub fn read_stdin(&self, system: &mut System) -> ReadStdin {
        system.read_stdin() // TODO: support override
    }

    pub fn write_stdout(&self, system: &mut System) -> WriteStdout {
        system.write_stdout() // TODO: support override
    }

    pub fn write_stderr(&self, system: &mut System) -> WriteStderr {
        system.write_stderr() // TODO: support override
    }
}

#[derive(Clone, Debug)]
pub enum StdioError {
    UnknownSystem(String),
    UnknownParameter(String),
    MissingParameter(&'static str),
    InvalidParameter(&'static str),
}

impl std::error::Error for StdioError {}

impl std::fmt::Display for StdioError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use StdioError::*;
        match self {
            UnknownSystem(system) => {
                write!(f, "unknown system: {}", system)
            }
            UnknownParameter(parameter) => {
                write!(f, "unknown parameter: {}", parameter)
            }
            MissingParameter(parameter) => {
                write!(f, "missing parameter: {}", parameter)
            }
            InvalidParameter(parameter) => {
                write!(f, "invalid parameter: {}", parameter)
            }
        }
    }
}
