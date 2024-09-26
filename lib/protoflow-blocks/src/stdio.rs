// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{Encoding, ReadStdin, SysBlocks, System, WriteStderr, WriteStdout};
use protoflow_core::prelude::{BTreeMap, String};

pub trait StdioSystem {
    fn build_system(config: StdioConfig) -> Result<System, StdioError>;
}

pub struct StdioConfig {
    pub encoding: Encoding,
    pub params: BTreeMap<String, String>,
}

impl StdioConfig {
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
            MissingParameter(parameter) => {
                write!(f, "missing parameter: {}", parameter)
            }
            InvalidParameter(parameter) => {
                write!(f, "invalid parameter: {}", parameter)
            }
        }
    }
}
