// This is free and unencumbered software released into the public domain.

use crate::exit::ExitCode;
use protoflow_syntax::SystemParser;
use std::path::PathBuf;

pub fn check(paths: &Vec<PathBuf>) -> Result<(), ExitCode> {
    for path in paths {
        let mut parser = SystemParser::from_file(path)?;
        let _ = parser.check()?;
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub enum CheckError {}
