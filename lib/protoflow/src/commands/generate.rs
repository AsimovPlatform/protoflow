// This is free and unencumbered software released into the public domain.

use crate::exit::ExitCode;
use protoflow_syntax::{Code, SystemParser};
use std::path::PathBuf;

pub fn generate(path: &PathBuf) -> Result<(), ExitCode> {
    let mut parser = SystemParser::from_file(path)?;
    let model = parser.check()?;
    let code = Code::try_from(model)?;
    std::print!("{}", code.unparse());
    Ok(())
}

#[derive(Clone, Debug)]
pub enum GenerateError {}
